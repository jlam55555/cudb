//! Document model data representation.

use crate::value::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// (Data) document (as opposed to query document, etc.)
/// Note that `_id` is implemented as a regular field in the `elems`.
// TODO: implement custom serialize/deserialize
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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

    /// Get the hashmap from a document.
    pub fn to_map(&self) -> HashMap<String, Value> {
        self.elems.clone()
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
