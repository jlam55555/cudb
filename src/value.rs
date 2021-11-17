//! Variant types for flexible JSON-like document values.

use crate::document::*;
use serde::{Serialize, Deserialize};

/// Variant type for (data) document values.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
