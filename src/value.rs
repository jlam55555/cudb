use crate::document::*;

#[derive(Debug)]
pub enum Value {
    Id(String),
    Int32(i32),
    String(String),
    Dict(Document),
    Array(Vec<Value>),
}
