use crate::document::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Value {
    Id(String),
    Int32(i32),
    String(String),
    Dict(Document),
    Array(Vec<Value>),
}
