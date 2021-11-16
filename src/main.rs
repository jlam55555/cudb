mod crud;
mod db;
mod document;
mod index;
mod mmapv1;
mod query;
mod value;

use value::*;

fn main() {
    // Create a new mutable document
    let mut d = document::Document::new();

    // Insert some values into the document
    d.insert(
        "Hello".to_string(),
        Value::Id("12312312541243123".to_string()),
    );

    // Print the document using the default debug trait
    println!("d = {:?}", d)
}
