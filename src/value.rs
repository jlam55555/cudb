use crate::document::*;

pub enum Value {
    Id(String),
    Int32(i32),
    String(String),
    Dict(Document),
    Array(Vec<Value>),
}
