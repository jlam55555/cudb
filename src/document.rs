use std::collections::HashMap;

pub type Document = HashMap<String, Value>;

trait DocumentTrait {
    fn new() -> Document;
}

impl DocumentTrait for Document {
    fn new() -> Document {
        HashMap::new()
    }
}

pub enum Value {
    Id(String),
    Int32(i32),
    String(String),
    Dict(Document),
}
