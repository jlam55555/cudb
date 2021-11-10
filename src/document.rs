use crate::value::*;
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
