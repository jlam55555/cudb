//! Document model data representation.

use crate::value::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::query::FieldPath;

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

    /// Retrieve the value given the path.
    pub fn get(&self, path: &FieldPath) -> Value {
        let mut temp_elems = &self.elems;

        for component in path[0..path.len()-1].iter() {
            temp_elems = match temp_elems.get(&component[..]).unwrap() {
                Value::Dict(sub_doc) => sub_doc.get_map_ref(),
                _ => panic!("Invalid path: Non-document found"),
            };
        }

        temp_elems.get(&path.last().unwrap()[..]).unwrap().clone()
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
