//! B-tree indexing.

// use compare::{Compare, natural};
use crate::query::FieldPath;
use crate::value::Value;

// TODO: implementing indices/B-trees
/// Store the fields used for an index.
#[derive(PartialEq, Eq, Hash)]
pub struct Index {
    fields: Vec<FieldPath>,
}

impl Index {
    pub fn new(fields: Vec<FieldPath>) -> Index {
        Index{fields: fields}
    }
}

/// Store the values for the fields for a particular document.
pub struct IndexInstance {
    fields: Vec<Value>,
}
