//! Variant types for flexible JSON-like document values.

use std::cmp::Ordering;
use crate::document::*;
use serde::{Serialize, Deserialize};

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
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!()
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!()
    }

    fn max(self, other: Self) -> Self where Self: Sized {
        todo!()
    }

    fn min(self, other: Self) -> Self where Self: Sized {
        todo!()
    }

    fn clamp(self, min: Self, max: Self) -> Self where Self: Sized {
        todo!()
    }
}
