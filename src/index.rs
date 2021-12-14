//! B-tree indexing.

use crate::document::Document;
use crate::query::ConstraintDocument;
use crate::query::FieldPath;
use crate::value::Value;

use std::collections::{HashMap, HashSet};
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

    /// Compare the current Index Schema with another Index Schema represented by a HashMap.
    /// Check if any shared fields have conflicting Value variants (e.g. different types).
    pub fn is_conflicting(&self, index_schema: &HashMap<&FieldPath, &Value>) -> bool {
        for field_spec in &self.fields {
            // Check if the other Index Schema shares the field and if it does, whether the
            // default values have the same variant.
            if !match index_schema.get(field_spec.get_field_path()) {
                Some(value) => field_spec.get_default().is_variant_equal(value),
                None => true,
            } {
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
    pub fn get_num_matched_fields(&self, query_fields: &HashSet<&FieldPath>) -> i32 {
        let index_fields = self.get_fields();

        let mut cur_matched = 0;
        for field in index_fields {
            if query_fields.contains(&field) {
                cur_matched += 1;
            }
        }

        cur_matched
    }

    /// Calculates the non-overlapping b-tree query ranges for a ConstraintDocument.
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

        dbg!(&next_combinations_indices);
        dbg!(&next_combinations);

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
        let index_schema_1 = IndexSchema::new(vec![FieldSpec::new(
            vec![String::from("a")],
            Value::Int32(0),
        )]);
        let constraint_1 = &HashMap::from([(
            vec![String::from("a")],
            Constraint::LessThan(Value::Int32(2)),
        )]);
        assert!(
            index_schema_1.generate_btree_ranges(constraint_1)
                == vec![(
                    Bound::Included(Index::new(vec![Value::Int32(0).get_min_value()])),
                    Bound::Included(Index::new(vec![Value::Int32(2)]))
                )]
        );
    }
}
