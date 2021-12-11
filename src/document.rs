//! Document model data representation.

use crate::query::FieldPath;
use crate::value::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// (Data) document (as opposed to query document, etc.)
/// Note that `_id` is implemented as a regular field in the `elems`.
// TODO: implement custom serialize/deserialize
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Document {
    elems: HashMap<String, Value>, // Hashmap of elements
}

impl Document {
    /// Construct a new document.
    pub fn new() -> Document {
        Document {
            elems: HashMap::new(),
        }
    }

    /// Construct a document from a hashmap.
    pub fn from(map: HashMap<String, Value>) -> Document {
        Document { elems: map }
    }

    /// Retrieve the value given the path if it exists.
    pub fn get(&self, path: &FieldPath) -> Option<Value> {
        let mut temp_elems = &self.elems;

        // Retrieve a nested document according to the field path.
        for component in path[0..path.len() - 1].iter() {
            temp_elems = match temp_elems.get(&component[..]) {
                Some(value) => match value {
                    Value::Dict(sub_doc) => sub_doc.get_map_ref(),
                    _ => return Option::None,
                },
                None => return Option::None,
            };
        }

        match temp_elems.get(&path.last().unwrap()[..]) {
            Some(value) => Option::from(value.clone()),
            None => Option::None,
        }
    }

    /// Retrieve the value given the path if it exists. Otherwise, use the default value provided.
    pub fn get_or_default(&self, path: &FieldPath, default: Value) -> Value {
        match self.get(path) {
            Some(value) => value,
            None => default,
        }
    }

    /// Get the hashmap from a document.
    pub fn get_map(&self) -> HashMap<String, Value> {
        self.elems.clone()
    }

    /// Get a reference to the hashmap from a document.
    pub fn get_map_ref(&self) -> &HashMap<String, Value> {
        &self.elems
    }

    /// Insert k,v pair into hashtable.
    /// If key already exists, the old value is returned.
    pub fn insert(&mut self, k: String, v: Value) -> Option<Value> {
        self.elems.insert(k, v)
    }

    /// Creates `_id` on document (non-recursively) if it doesn't exist.
    /// Returns whether the `_id` was updated.
    pub fn create_id(&mut self) -> bool {
        unimplemented!("create_id")
    }
}
