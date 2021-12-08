//! User-facing query representation.

use crate::document::Document;
use crate::index::IndexSchema;
use crate::value::Value;

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::ops::Bound;

/*
/// Representations of a (possibly nested) document field.
pub enum FieldPath {
    /// The full path as a string. May include dots representing nesting.
    Field(String),
    /// The full path as a vector of path components.
    Path(Vec<String>),
}
 */

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
    fn remove_index_fields(&self, index_schema: &IndexSchema) -> ConstraintDocument {
        let mut reduced_constraints = HashMap::new();

        let constraints_set: HashSet<&FieldPath> = self.keys().collect();
        let index_constraints_set: HashSet<&FieldPath> =
            HashSet::from_iter(index_schema.get_fields().iter().clone());
        constraints_set
            .difference(&index_constraints_set)
            .for_each(|field_path| {
                match self.get(field_path.clone()) {
                    None => panic!("missing field path"),
                    Some(constraint) => {
                        reduced_constraints.insert(field_path.clone().clone(), constraint.clone())
                    }
                };
            });
        reduced_constraints
    }

    fn matches_document(&self, doc: &Document) -> bool {
        for (field_path, constraint) in self.iter() {
            // TODO: remove
            dbg!(&field_path, &constraint);

            if !constraint.matches(&doc.get(field_path)) {
                return false;
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
    pub fn generate_value_ranges(&self) -> Vec<(Bound<Value>, Bound<Value>)> {
        todo!();
    }
}

/// Projection of fields during a query (analogous to SQL `SELECT`).
pub type ProjectionDocument = HashMap<FieldPath, Projection>;

/// Projection of a single field of the projection document.
pub enum Projection {
    // Recursive projections on subdocuments.
    ProjectDocument(ProjectionDocument),

    // Exclude projecting a value.
    Exclude,

    // Project a value.
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
