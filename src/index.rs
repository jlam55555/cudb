//! B-tree indexing.

use crate::document::Document;
use crate::query::ConstraintDocument;
use crate::query::FieldPath;
use crate::value::Value;

use std::collections::HashMap;
use std::ops::Bound;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct FieldSpec {
    field_path: FieldPath,
    default: Value,
}

/// Store the field path and default value used for a field.
impl FieldSpec {
    pub fn new(field_path: FieldPath, default: Value) -> FieldSpec {
        FieldSpec {
            field_path: field_path,
            default: default,
        }
    }

    pub fn get_field_path(&self) -> &FieldPath {
        &self.field_path
    }

    pub fn get_default(&self) -> &Value {
        &self.default
    }
}

// TODO: implementing indices/B-trees
/// Store the fields used for an Index.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct IndexSchema {
    fields: Vec<FieldSpec>,
}

impl IndexSchema {
    pub fn new(fields: Vec<FieldSpec>) -> IndexSchema {
        IndexSchema { fields: fields }
    }

    pub fn get_fields(&self) -> Vec<FieldPath> {
        self.fields.iter().map(|x| x.field_path.clone()).collect()
    }

    pub fn get_field_specs(&self) -> &Vec<FieldSpec> {
        &self.fields
    }

    /// Convert the Index Schema into a HashMap.
    pub fn get_as_hashmap(&self) -> HashMap<&FieldPath, &Value> {
        self.get_field_specs().iter()
            .map(|x| (x.get_field_path(), x.get_default()))
            .collect()
    }

    /// Check if the Field Path is in the Index Schema and if it is, whether the
    /// default values have the same variant.
    ///
    /// Return true if the Field Path is not in the Index Schema or if the default values have the
    /// same variant.
    /// Return false if the default values do not have the same variant.
    fn is_field_spec_conflicting(&self, field_spec: &FieldSpec, index_schema: &HashMap<&FieldPath, &Value>) -> bool {
        !match index_schema.get(field_spec.get_field_path()) {
            Some(value) => field_spec.get_default().is_variant_equal(value),
            None => true,
        }
    }

    /// Compare the current Index Schema with another Index Schema.
    /// Check if any shared fields have conflicting Value variants (e.g. different types).
    pub fn is_conflicting(&self, index_schema: &IndexSchema) -> bool {
        let index_schema_as_hashmap = index_schema.get_as_hashmap();
        for field_spec in self.get_field_specs() {
            if self.is_field_spec_conflicting(&field_spec, &index_schema_as_hashmap) {
                return true;
            }
        }

        false
    }

    /// Create an Index from the provided Document.
    pub fn create_index(&self, doc: &Document) -> Option<Index> {
        // Extract the Values from the Document
        let mut values = Vec::new();
        for spec in self.get_field_specs() {
            let doc_value = doc.get_or_default(spec.get_field_path(), spec.get_default().clone());

            // Check if type of the Document Value matches the type of the default Value
            // If not, the Index is invalid (mismatched types)
            if !spec.default.is_variant_equal(&doc_value) {
                return Option::None;
            }
            values.push(doc_value);
        }

        Option::from(Index::new(values))
    }

    // ToDo: Decide if public or private
    /// Count the number of matched index fields in the query fields.
    pub fn get_num_matched_fields(&self, query_fields: &HashMap<&FieldPath, &Value>) -> i32 {
        let index_fields = self.get_field_specs();

        let mut cur_matched = 0;
        for field_spec in index_fields {
            if !self.is_field_spec_conflicting(&field_spec, query_fields) {
                cur_matched += 1;
            }
        }

        cur_matched
    }

    /// Calculate the non-overlapping b-tree query ranges for a ConstraintDocument.
    pub fn generate_btree_ranges(
        &self,
        constraints: &ConstraintDocument,
    ) -> Vec<(Bound<Index>, Bound<Index>)> {
        // Loop through each field of Index, in order
        // Convert each constraint to a range or set of ranges on that field
        let field_ranges = self
            .get_fields()
            .iter()
            .map(|field_path| constraints.get(field_path).unwrap().generate_value_ranges())
            .collect();

        // Generate all combinations of one range from each field
        Self::generate_combinations(&field_ranges, 0)
    }

    // Helper function to generate combinations for generate_btree_ranges.
    // The base case is messy but it works.
    fn generate_combinations(
        field_ranges: &Vec<Vec<(Bound<Value>, Bound<Value>)>>,
        i: usize,
    ) -> Vec<(Bound<Index>, Bound<Index>)> {
        assert!(i <= field_ranges.len() - 1);

        // Base case: convert each field/value range into a singleton index range
        if i == field_ranges.len() - 1 {
            return field_ranges
                .last()
                .unwrap()
                .iter()
                .map(|field_range| match field_range {
                    (Bound::Included(value_low), Bound::Included(value_high)) => (
                        Bound::Included(Index::new(vec![value_low.clone()])),
                        Bound::Included(Index::new(vec![value_high.clone()])),
                    ),
                    _ => panic!("non-inclusive value ranges"),
                })
                .collect();
        }

        let next_combinations_indices = Self::generate_combinations(field_ranges, i + 1);
        let next_combinations =
            next_combinations_indices
                .iter()
                .map(|index_range| match index_range {
                    (Bound::Included(index_range_low), Bound::Included(index_range_high)) => {
                        (index_range_low.get_values(), index_range_high.get_values())
                    }
                    _ => panic!("invalid index range"),
                });

        let mut combinations = Vec::new();
        for (value_low, value_high) in &field_ranges[i] {
            // For now, assert these bounds are inclusive; they should always be inclusive
            let (value_low, value_high) = match (value_low, value_high) {
                (Bound::Included(value_low), Bound::Included(value_high)) => {
                    (value_low, value_high)
                }
                _ => panic!("non-inclusive value ranges"),
            };

            for (next_range_low, next_range_high) in next_combinations.clone() {
                // Create new range ((value_low:next_range_low), (value_high:next_range_high))
                let mut low_range = vec![value_low.clone()];
                low_range.append(&mut next_range_low.clone());

                let mut high_range = vec![value_high.clone()];
                high_range.append(&mut next_range_high.clone());

                combinations.push((
                    Bound::Included(Index::new(low_range)),
                    Bound::Included(Index::new(high_range)),
                ));
            }
        }
        combinations
    }
}

/// Store the values for the fields for a particular document.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct Index {
    values: Vec<Value>,
}

impl Index {
    fn new(values: Vec<Value>) -> Index {
        Index { values: values }
    }

    fn get_values(&self) -> &Vec<Value> {
        &self.values
    }
}

// This needs to be in this module since Index::new() is private
#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::query::Constraint;
    use std::collections::HashMap;

    // Test IndexSchema::generate_btree_ranges() and IndexSchema::generate_combinations().
    #[test]
    fn test_generate_btree_ranges_simple() {
        let index_schema = IndexSchema::new(vec![FieldSpec::new(
            vec![String::from("a")],
            Value::Int32(0),
        )]);

        let constraint = &HashMap::from([(
            vec![String::from("a")],
            Constraint::LessThan(Value::Int32(2)),
        )]);

        assert!(
            index_schema.generate_btree_ranges(constraint)
                == vec![(
                    Bound::Included(Index::new(vec![Value::Int32(0).get_min_value()])),
                    Bound::Included(Index::new(vec![Value::Int32(2)]))
                )]
        );
    }

    // Test generate_btree_ranges() on multi-field indices.
    #[test]
    fn test_generate_btree_ranges_multi() {
        let index_schema = IndexSchema::new(vec![
            FieldSpec::new(vec![String::from("a")], Value::Int32(0)),
            FieldSpec::new(vec![String::from("b")], Value::Int32(0)),
            FieldSpec::new(vec![String::from("c")], Value::Int32(0)),
        ]);

        let constraint = &HashMap::from([
            (
                vec![String::from("a")],
                Constraint::LessThan(Value::Int32(2)),
            ),
            (vec![String::from("b")], Constraint::Equals(Value::Int32(5))),
            (
                vec![String::from("c")],
                Constraint::GreaterThan(Value::Int32(4)),
            ),
        ]);

        assert!(
            index_schema.generate_btree_ranges(constraint)
                == vec![(
                    Bound::Included(Index::new(vec![
                        Value::Int32(0).get_min_value(),
                        Value::Int32(5),
                        Value::Int32(4)
                    ])),
                    Bound::Included(Index::new(vec![
                        Value::Int32(2),
                        Value::Int32(5),
                        Value::Int32(0).get_max_value()
                    ]))
                )]
        );
    }

    // Test conjunction btree range generation
    #[test]
    fn test_generate_btree_ranges_conj() {
        let index_schema = IndexSchema::new(vec![FieldSpec::new(
            vec![String::from("a")],
            Value::Int32(0),
        )]);

        let constraint = &HashMap::from([(
            vec![String::from("a")],
            Constraint::And(
                Box::new(Constraint::GreaterThan(Value::Int32(-0))),
                Box::new(Constraint::LessThan(Value::Int32(3))),
            ),
        )]);

        assert!(
            index_schema.generate_btree_ranges(constraint)
                == vec![(
                    Bound::Included(Index::new(vec![Value::Int32(0)])),
                    Bound::Included(Index::new(vec![Value::Int32(3)])),
                )]
        );
    }

    // Test non-intersecting conjunction (empty set)
    #[test]
    fn test_generate_btree_ranges_conj_empty() {
        let index_schema = IndexSchema::new(vec![FieldSpec::new(
            vec![String::from("a")],
            Value::Int32(0),
        )]);

        let constraint = &HashMap::from([(
            vec![String::from("a")],
            Constraint::And(
                Box::new(Constraint::GreaterThan(Value::Int32(3))),
                Box::new(Constraint::LessThan(Value::Int32(0))),
            ),
        )]);

        assert!(index_schema.generate_btree_ranges(constraint) == vec![]);
    }

    // Test disjunction btree range generation
    #[test]
    fn test_generate_btree_ranges_disj() {
        let index_schema = IndexSchema::new(vec![FieldSpec::new(
            vec![String::from("a")],
            Value::Int32(0),
        )]);

        let constraint = &HashMap::from([(
            vec![String::from("a")],
            Constraint::Or(
                Box::new(Constraint::GreaterThan(Value::Int32(3))),
                Box::new(Constraint::LessThan(Value::Int32(0))),
            ),
        )]);

        assert!(
            index_schema.generate_btree_ranges(constraint)
                == vec![
                    (
                        Bound::Included(Index::new(vec![Value::Int32(3)])),
                        Bound::Included(Index::new(vec![Value::Int32(0).get_max_value()])),
                    ),
                    (
                        Bound::Included(Index::new(vec![Value::Int32(0).get_min_value()])),
                        Bound::Included(Index::new(vec![Value::Int32(0)])),
                    ),
                ]
        );
    }

    // Test combining overlapping ranges (i.e., double-counting)
    // TODO: currently fails
    #[test]
    fn test_generate_btree_ranges_disj_overlap() {
        let index_schema = IndexSchema::new(vec![FieldSpec::new(
            vec![String::from("a")],
            Value::Int32(0),
        )]);

        let constraint = &HashMap::from([(
            vec![String::from("a")],
            Constraint::Or(
                Box::new(Constraint::LessThan(Value::Int32(3))),
                Box::new(Constraint::GreaterThan(Value::Int32(0))),
            ),
        )]);

        assert!(
            index_schema.generate_btree_ranges(constraint)
                == vec![(
                    Bound::Included(Index::new(vec![Value::Int32(0).get_min_value()])),
                    Bound::Included(Index::new(vec![Value::Int32(0).get_max_value()])),
                ),]
        );
    }

    // Test combining non-overlapping ranges into DNF.
    #[test]
    fn test_generate_btree_ranges_dnf() {
        let index_schema = IndexSchema::new(vec![FieldSpec::new(
            vec![String::from("a")],
            Value::Int32(0),
        )]);

        let constraint = &HashMap::from([(
            vec![String::from("a")],
            Constraint::Or(
                Box::new(Constraint::And(
                    Box::new(Constraint::LessThan(Value::Int32(10))),
                    Box::new(Constraint::GreaterThan(Value::Int32(5))),
                )),
                Box::new(Constraint::LessThan(Value::Int32(0))),
            ),
        )]);

        assert!(
            index_schema.generate_btree_ranges(constraint)
                == vec![
                    (
                        Bound::Included(Index::new(vec![Value::Int32(5)])),
                        Bound::Included(Index::new(vec![Value::Int32(10)])),
                    ),
                    (
                        Bound::Included(Index::new(vec![Value::Int32(0).get_min_value()])),
                        Bound::Included(Index::new(vec![Value::Int32(0)])),
                    ),
                ]
        );
    }

    // Test multi-index fields with disjoint ranges.
    #[test]
    fn test_generate_btree_ranges_complex() {
        let index_schema = IndexSchema::new(vec![
            FieldSpec::new(vec![String::from("a")], Value::Int32(0)),
            FieldSpec::new(vec![String::from("b")], Value::Int32(0)),
            FieldSpec::new(vec![String::from("c")], Value::Int32(0)),
        ]);

        let constraint = &HashMap::from([
            (
                vec![String::from("a")],
                Constraint::Or(
                    Box::new(Constraint::LessThan(Value::Int32(2))),
                    Box::new(Constraint::GreaterThan(Value::Int32(8))),
                ),
            ),
            (
                vec![String::from("b")],
                Constraint::Or(
                    Box::new(Constraint::Equals(Value::Int32(5))),
                    Box::new(Constraint::LessThan(Value::Int32(3))),
                ),
            ),
            (
                vec![String::from("c")],
                Constraint::And(
                    Box::new(Constraint::GreaterThan(Value::Int32(4))),
                    Box::new(Constraint::LessThan(Value::Int32(9))),
                ),
            ),
        ]);

        assert!(
            index_schema.generate_btree_ranges(constraint)
                == vec![
                    (
                        Bound::Included(Index::new(vec![
                            Value::Int32(0).get_min_value(),
                            Value::Int32(5),
                            Value::Int32(4),
                        ])),
                        Bound::Included(Index::new(vec![
                            Value::Int32(2),
                            Value::Int32(5),
                            Value::Int32(9),
                        ]))
                    ),
                    (
                        Bound::Included(Index::new(vec![
                            Value::Int32(0).get_min_value(),
                            Value::Int32(0).get_min_value(),
                            Value::Int32(4),
                        ])),
                        Bound::Included(Index::new(vec![
                            Value::Int32(2),
                            Value::Int32(3),
                            Value::Int32(9),
                        ]))
                    ),
                    (
                        Bound::Included(Index::new(vec![
                            Value::Int32(8),
                            Value::Int32(5),
                            Value::Int32(4),
                        ])),
                        Bound::Included(Index::new(vec![
                            Value::Int32(0,).get_max_value(),
                            Value::Int32(5,),
                            Value::Int32(9,),
                        ]))
                    ),
                    (
                        Bound::Included(Index::new(vec![
                            Value::Int32(8),
                            Value::Int32(0).get_min_value(),
                            Value::Int32(4),
                        ])),
                        Bound::Included(Index::new(vec![
                            Value::Int32(0).get_max_value(),
                            Value::Int32(3),
                            Value::Int32(9),
                        ]))
                    )
                ]
        );
    }
}
