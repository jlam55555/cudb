// Test cases for indices.

use cudb::db::Collection;
use cudb::document::Document;
use cudb::index::FieldSpec;
use cudb::value::Value;

use std::collections::HashMap;

#[path = "./utils.rs"]
mod utils;

#[cfg(test)]
pub mod tests {
    use super::*;

    // Create a collection with an index.
    #[test]
    fn test_declare_index_simple() {
        let mut col = Collection::from(utils::DB_NAME);

        // Visually check that the documents and indices are created correctly
        for doc in utils::sample_documents(5) {
            println!("Inserting document: {:#?}", doc);
            col.get_mut_pool().write_new(doc);
        }

        // Unique field
        col.declare_index(vec![FieldSpec::new(
            vec![String::from("key")],
            Value::Int32(0),
        )]);

        // Non-unique field
        col.declare_index(vec![FieldSpec::new(
            vec![String::from("y")],
            Value::String(String::from("")),
        )]);

        println!("Indices: {:#?}", col.get_indices());

        // TODO: Implement this check with assertions, rather than being a visual check
        // (Don't have time to do this atm)

        col.drop();
    }

    // Indices on keys that do not exist on all documents.
    #[test]
    fn test_declare_index_missing() {
        let mut col = Collection::from(utils::DB_NAME);

        // Visually determine correctness
        let mut x = 0;
        for mut doc in utils::sample_documents(5) {
            if x % 2 == 0 {
                doc.insert("partialfield".to_string(), Value::Int32(x + 1));
            }
            x += 1;
            println!("Inserting document: {:#?}", doc);
            col.get_mut_pool().write_new(doc);
        }

        // Field that exists on some documents
        col.declare_index(vec![FieldSpec::new(
            vec![String::from("partialfield")],
            Value::Int32(-232232),
        )]);

        // Field that exists on no documents
        col.declare_index(vec![FieldSpec::new(
            vec![String::from("missingfield")],
            Value::String(String::from("all documents have this value")),
        )]);

        println!("Indices: {:#?}", col.get_indices());

        col.drop();
    }

    // Multi-field index.
    #[test]
    fn test_declare_index_multi() {
        let mut col = Collection::from(utils::DB_NAME);

        // Visually check that the documents and indices are created correctly
        for doc in utils::sample_documents(5) {
            println!("Inserting document: {:#?}", doc);
            col.get_mut_pool().write_new(doc);
        }

        // Two-field index
        col.declare_index(vec![
            FieldSpec::new(vec![String::from("key")], Value::Int32(0)),
            FieldSpec::new(vec![String::from("y")], Value::String(String::from(""))),
        ]);

        println!("Indices: {:#?}", col.get_indices());

        // TODO: Implement this check with assertions, rather than being a visual check
        // (Don't have time to do this atm)

        col.drop();
    }

    // TODO: Indices remain correct after CUD operations.
}
