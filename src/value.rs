//! Variant types for flexible JSON-like document values.

use crate::document::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// Variant type for (data) document values.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Value {
    /// Special type for unique identification, similar to MongoDB's `_id`.
    Id(String),
    /// Fixed-size integer type.
    Int32(i32),
    /// Arbitrary-length strings.
    String(String),
    /// Recursive documents (hashtables).
    Dict(Document),
    /// Array types.
    Array(Vec<Value>),
}

impl PartialOrd<Self> for Value {
    /// We can only compare like scalar types (i.e., not documents nor arrays).
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Id(i1), Self::Id(i2)) => i1.partial_cmp(i2),
            (Self::Int32(i1), Self::Int32(i2)) => i1.partial_cmp(i2),
            (Self::String(s1), Self::String(s2)) => s1.partial_cmp(s2),
            _ => None,
        }
    }
}

impl Ord for Value {
    /// Define a total ordering on all scalar values. Panics if non-scalar
    /// or unlike values.
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            Some(ord) => ord,
            None => panic!("invalid value comparison"),
        }
    }
}
