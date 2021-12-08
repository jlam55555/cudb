//! B-tree indexing.

use crate::document::Document;
use crate::query::ConstraintDocument;
use crate::query::FieldPath;
use crate::value::Value;

use std::collections::HashSet;
use std::ops::Bound;

// TODO: implementing indices/B-trees
/// Store the fields used for an index.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct IndexSchema {
    fields: Vec<FieldPath>,
}

impl IndexSchema {
    pub fn new(fields: Vec<FieldPath>) -> IndexSchema {
        IndexSchema { fields: fields }
    }

    pub fn get_fields(&self) -> &Vec<FieldPath> {
        &self.fields
    }

    // ToDo: Support default values here?
    /// Create an IndexInstance from the provided document.
    pub fn create_index_instance(&self, doc: &Document) -> Index {
        // Extract the values from the document
        let mut values = Vec::new();
        for ind in self.get_fields() {
            values.push(doc.get(ind));
        }

        Index::new(values)
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
        // Loop through each field of Index, in order.
        // Convert each constraint to a range or set of ranges on that field.
        let field_ranges = self
            .get_fields()
            .iter()
            .map(|field_path| constraints.get(field_path).unwrap().generate_value_ranges())
            .collect();

        // Generate all combinations of one range from each field.
        // TODO: Deal with mixed bounds (equality and inequality).
        Self::generate_combinations(&field_ranges, 0)
    }

    // Helper function to generate combinations for generarte_btree_ranges.
    // TOOD: test this function.
    fn generate_combinations(
        field_ranges: &Vec<Vec<(Bound<Value>, Bound<Value>)>>,
        i: usize,
    ) -> Vec<(Bound<Index>, Bound<Index>)> {
        if i == field_ranges.len() {
            return Vec::new();
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
            // For now, assert these bounds are inclusive.
            let (value_low, value_high) = match (value_low, value_high) {
                (Bound::Included(value_low), Bound::Included(value_high)) => {
                    (value_low, value_high)
                }
                _ => panic!("non-inclusive value ranges"),
            };

            for (next_range_low, next_range_high) in next_combinations.clone() {
                // Create new range ((value_low:next_range_low), (value_high:next_range_high)).
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
