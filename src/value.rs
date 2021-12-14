//! Variant types for flexible JSON-like document values.

use crate::document::Document;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

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

impl Value {
    /// Check if the provided Value is the same variant as the current Value.
    /// A variant is a component of an enum.
    ///
    /// Based on: <https://stackoverflow.com/a/32554326>
    pub fn is_variant_equal(&self, val: &Value) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(val)
    }

    /// Returns inclusive minimum for range search on value.
    ///
    /// See Constraint::generate_value_ranges() for more details.
    pub fn get_min_value(&self) -> Self {
        match self {
            Self::Int32(_) => Self::Int32(std::i32::MIN),
            Self::String(_) => Self::String("".to_string()),
            _ => panic!("uncomparable type"),
        }
    }

    /// Returns inclusive maximum for range search on value.
    ///
    /// See Constraint::generate_value_ranges() for more details. Note that this imposes an
    /// arbitrary maximum limit for unbounded values, such as strings.
    pub fn get_max_value(&self) -> Self {
        match self {
            Self::Int32(_) => Self::Int32(std::i32::MAX),
            Self::String(_) => {
                Self::String((0..32).map(|_| std::char::from_u32(255).unwrap()).collect())
            }
            _ => panic!("uncomparable type"),
        }
    }
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

impl Hash for Value {
    /// We do not allow indexing by a Document. Panics if a Document is used.
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Id(id) => id.hash(state),
            Self::Int32(i) => i.hash(state),
            Self::String(str) => str.hash(state),
            Self::Array(arr) => arr.hash(state),
            _ => panic!("trying to hash a document"),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Id(val) => val.fmt(f),
            Self::Int32(val) => val.fmt(f),
            Self::String(val) => val.fmt(f),
            Self::Dict(subdoc) => subdoc.fmt(f),
            Self::Array(arr) => {
                write!(f, "[").unwrap();
                for val in arr {
                    write!(f, " {}", val).unwrap()
                }
                write!(f, " ]")
            }
        }
    }
}
