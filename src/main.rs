mod crud;
mod db;
mod document;
mod index;
mod mmapv1;
mod query;
mod value;

use document::*;
use value::*;

fn main() {
    // Create a new mutable document
    let mut d = Document::new();

    // Insert some values into the document
    d.insert(
        "Hello".to_string(),
        Value::Id("12312312541243123".to_string()),
    );

    // Print the document using the default debug trait
    println!("d = {:?}", d);

    // bincode serialization
    let encoded = bincode::serialize(&d).unwrap();
    println!("encoded = {:?}", encoded);

    let decoded: Document = bincode::deserialize(&encoded[..]).unwrap();
    println!("decoded = {:?}", decoded);
}
