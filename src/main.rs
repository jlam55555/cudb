mod crud;
mod db;
mod document;
mod index;
mod query;
mod value;

use value::*;

fn main() {
    let mut d = document::Document::new();

    d.insert(
        "Hello".to_string(),
        Value::Id("12312312541243123".to_string()),
    );
}
