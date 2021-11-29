//! B-tree indexing.

// use compare::{Compare, natural};
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
    pub fn create_index_instance(&self, doc: Document) -> Index {
        // Extract the values from the document
        let mut values = Vec::new();
        for ind in self.get_fields() {
            values.push(doc.get(ind));
        }

        Index::new(values)
    }
}

/// Store the values for the fields for a particular document.
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
