//! B-tree indexing.

use std::collections::HashSet;
use crate::query::FieldPath;
use crate::document::Document;
use crate::value::Value;

// TODO: implementing indices/B-trees
/// Store the fields used for an index.
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct IndexSchema {
    fields: Vec<FieldPath>,
}

impl IndexSchema {
    pub fn new(fields: Vec<FieldPath>) -> IndexSchema {
        IndexSchema {
            fields: fields,
        }
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
    pub fn get_num_matched_columns(&self, query_fields: &HashSet<FieldPath>) -> i32 {
        let index_fields = self.get_fields();

        let mut cur_matched = 0;
        for field in index_fields {
            if query_fields.contains(field) {
                cur_matched += 1;
            }
        }

        cur_matched
    }
}

/// Store the values for the fields for a particular document.
#[derive(PartialOrd, Ord, PartialEq, Eq)]
pub struct Index {
    values: Vec<Value>,
}

impl Index {
    fn new(values: Vec<Value>) -> Index {
        Index {
            values: values,
        }
    }
}
