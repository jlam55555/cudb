//! User-facing query representation.

use crate::document::Document;
use crate::index::IndexSchema;
use crate::value::Value;

use std::collections::HashMap;
use std::ops::Bound;

pub type FieldPath = Vec<String>;

/// The order that the results should be sorted in.
pub enum ResultOrder {
    /// Sorted ascending according to the given field.
    Asc(FieldPath),
    /// Sorted descending according to the given field.
    Desc(FieldPath),
}

/// Document type modeling query constraints.
pub type ConstraintDocument = HashMap<FieldPath, Constraint>;

/// Exists so that we can implement functions on ConstraintDocument.
pub trait ConstraintDocumentTrait {
    /// Constructs a reduced ConstraintDocument by removing the fields
    /// that are part of the IndexSchema.
    fn remove_index_fields(&self, index_schema: &IndexSchema) -> Self;

    /// Returns whether the current constraint document matches a document.
    fn matches_document(&self, doc: &Document) -> bool;
}

impl ConstraintDocumentTrait for ConstraintDocument {
    fn remove_index_fields(&self, _index_schema: &IndexSchema) -> ConstraintDocument {
        // For the time being, don't remove index fields, due to the issue with
        // inclusive/exclusive bounds
        self.clone()

        // let mut reduced_constraints = HashMap::new();

        // let constraints_set: HashSet<&FieldPath> = self.keys().collect();
        // let index_constraints_set: HashSet<&FieldPath> =
        //     HashSet::from_iter(index_schema.get_fields().iter().clone());
        // constraints_set
        //     .difference(&index_constraints_set)
        //     .for_each(|field_path| {
        //         match self.get(field_path.clone()) {
        //             None => panic!("missing field path"),
        //             Some(constraint) => {
        //                 reduced_constraints.insert(field_path.clone().clone(), constraint.clone())
        //             }
        //         };
        //     });
        // reduced_constraints
    }

    /// Check if a Document matches the constraints. If a Document is missing a Value,
    /// it automatically doesn't match the Constraint.
    fn matches_document(&self, doc: &Document) -> bool {
        for (path, constraint) in self.iter() {
            match doc.get(path) {
                Some(value) => {
                    if !constraint.matches(&value) {
                        return false;
                    }
                }
                None => return false,
            }
        }

        true
    }
}

/// A single query constraint on a field.
///
/// Note that Constraints applied to an array
/// value will map the constraint over the array.
#[derive(Clone, Debug)]
pub enum Constraint {
    /// Constraints on subdocuments (hashtables).
    MatchesDocument(ConstraintDocument),

    /// Equality constraint on a value.
    Equals(Value),

    /// Less-than constraint on a value.
    LessThan(Value),

    /// Greater-than constraint on a value.
    GreaterThan(Value),

    /// Constraint if value is in specified list of values.
    In(Vec<Value>),

    /// Disjunction of constraints on a single field.
    Or(Box<Constraint>, Box<Constraint>),

    /// Conjunction of constraints on a single field.
    And(Box<Constraint>, Box<Constraint>),
}

impl Constraint {
    /// Determine the type of a constraint.
    /// Return None if the Constraint is invalid due to mismatched subconstraints.
    pub fn get_value_type(&self) -> Option<Value> {
        match self {
            Self::Equals(value) | Self::LessThan(value) | Self::GreaterThan(value) => {
                Some(value.clone())
            }
            Self::Or(constraint1, constraint2) | Self::And(constraint1, constraint2) => {
                match (constraint1.get_value_type(), constraint2.get_value_type()) {
                    (None, _) | (_, None) => None,
                    (Some(value1), Some(value2)) => {
                        if value1.is_variant_equal(&value2) {
                            Some(value1)
                        } else {
                            None
                        }
                    }
                }
            }
            _ => panic!("currently unsupported constraint type"),
        }
    }

    /// Whether a Value matches a constraint.
    pub fn matches(&self, value: &Value) -> bool {
        match self {
            Constraint::MatchesDocument(constraint_doc) => match value {
                Value::Dict(doc) => constraint_doc.matches_document(doc),
                _ => false,
            },
            Constraint::Equals(value2) => value == value2,
            Constraint::LessThan(value2) => value < value2,
            Constraint::GreaterThan(value2) => value > value2,
            Constraint::And(constraint1, constraint2) => {
                constraint1.matches(value) && constraint2.matches(value)
            }
            Constraint::Or(constraint1, constraint2) => {
                constraint1.matches(value) || constraint2.matches(value)
            }
            Constraint::In(values) => values.iter().any(|value2| value == value2),
        }
    }

    /// Generate the value range(s) for this constraint.
    ///
    /// Note: for the time being, this generates inclusive ranges only, because
    /// we cannot mix inclusive and exclusive ranges in a multi-field index.
    /// Note that since we also cannot mix unbounded ranges, one-sided ranges
    /// (such as inequalities) use a predefined min/max value. For unbounded numbers,
    /// this limits the range of valid numbers that can be returned with a range.
    /// This can be considered a "feature" of our implementation due to the limitations
    /// of using the builtin BTreeMap implementation. An alternative implementation
    /// is to create a special value type that represents the extrema of a type
    /// (which would probably be a cleaner and more robust solution if only I had
    /// thought of it earlier).
    ///
    /// Note: disjunction (OR) operator assumes ranges are non-overlapping.
    pub fn generate_value_ranges(&self) -> Vec<(Bound<Value>, Bound<Value>)> {
        match self {
            Constraint::Equals(value) => vec![(
                Bound::Included(value.clone()),
                Bound::Included(value.clone()),
            )],
            Constraint::LessThan(value) => vec![(
                Bound::Included(value.get_min_value()),
                Bound::Included(value.clone()),
            )],
            Constraint::GreaterThan(value) => vec![(
                Bound::Included(value.clone()),
                Bound::Included(value.get_max_value()),
            )],

            // Conjunction: Combines each pair of ranges
            Constraint::And(constraint1, constraint2) => {
                let value_ranges1 = constraint1.generate_value_ranges();
                let value_ranges2 = constraint2.generate_value_ranges();

                let mut value_ranges = Vec::new();
                for (value_range1_min, value_range1_max) in &value_ranges1 {
                    for (value_range2_min, value_range2_max) in &value_ranges2 {
                        // Assert that range bounds are inclusive.
                        let (min1, max1, min2, max2) = match (
                            value_range1_min,
                            value_range1_max,
                            value_range2_min,
                            value_range2_max,
                        ) {
                            (
                                Bound::Included(min1),
                                Bound::Included(max1),
                                Bound::Included(min2),
                                Bound::Included(max2),
                            ) => (min1, max1, min2, max2),
                            _ => panic!("non-inclusive bounds"),
                        };

                        // Combine range if overlapping
                        let (range_min, range_max) =
                            (std::cmp::max(min1, min2), std::cmp::min(max1, max2));
                        if range_max > range_min {
                            value_ranges.push((
                                Bound::Included(range_min.clone()),
                                Bound::Included(range_max.clone()),
                            ));
                        }
                    }
                }
                value_ranges
            }

            // Disjunction: Returns separate ranges
            // Assumes non-overlapping ranges
            Constraint::Or(constraint1, constraint2) => {
                let mut value_ranges = constraint1.generate_value_ranges();
                value_ranges.append(&mut constraint2.generate_value_ranges());
                value_ranges
            }

            _ => panic!("unsupported range type"),
        }
    }
}

/// Projection of fields during a query (analogous to SQL `SELECT`).
pub type ProjectionDocument = HashMap<FieldPath, Projection>;

/// Projection of a single field of the projection document.
pub enum Projection {
    // Recursive projections on subdocuments
    ProjectDocument(ProjectionDocument),

    // Exclude projecting a value
    Exclude,

    // Project a value
    Include,
}

/// Complete query operation.
pub struct Query {
    // Constraint document (`WHERE`)
    pub constraints: ConstraintDocument,

    // Projection document (`SELECT`)
    pub projection: ProjectionDocument,

    // Ordering document (`ORDER BY`)
    pub order: Option<Vec<ResultOrder>>,
}

/// Document used in update operations.
pub struct UpdateDocument {
    // TODO
}
