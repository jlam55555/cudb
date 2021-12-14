// Test cases for the query operation.

use cudb::db::Collection;
use cudb::document::Document;
use cudb::index::FieldSpec;
use cudb::query::{Constraint, Query};
use cudb::value::Value;

use std::collections::HashMap;

#[path = "./utils.rs"]
mod utils;

// Test fixture for query tests
fn fixture() -> Vec<Document> {
    let mut docs = vec![Document::new(); 10];

    docs[0].insert(String::from("a"), Value::Int32(0));

    docs[1].insert(String::from("a"), Value::Int32(5));

    docs[2].insert(String::from("a"), Value::Int32(2));

    docs[3].insert(String::from("a"), Value::Int32(-1));

    docs[4].insert(String::from("a"), Value::Int32(4));

    docs[5].insert(String::from("a"), Value::Int32(4));

    docs[6].insert(String::from("a"), Value::Int32(4));

    docs[7].insert(String::from("a"), Value::Int32(4));

    docs[8].insert(String::from("a"), Value::Int32(4));

    docs[9].insert(String::from("a"), Value::Int32(4));

    docs
}

#[cfg(test)]
pub mod tests {
    use super::*;

    // Query chooses maximal matching index.
    // TODO

    // Query doesn't choose index that doesn't match type.
    // TODO

    // Query for equality on single-index field.
    #[test]
    fn test_find_many_single_index() {
        let mut col = Collection::from(utils::DB_NAME);

        // Visually check that the documents and indices are created correctly
        for doc in fixture() {
            col.get_mut_pool().write_new(doc);
        }

        col.declare_index(vec![FieldSpec::new(
            vec![String::from("a")],
            Value::Int32(-12345),
        )]);

        println!("Indices: {:#?}", col.get_indices());

        // Create query
        let query = Query {
            constraints: HashMap::from([(
                vec![String::from("a")],
                Constraint::LessThan(Value::Int32(2)),
            )]),
            projection: HashMap::new(),
            order: None,
        };

        // TODO: query; working here
        println!("{:#?}", col.find_many(query));

        col.drop();
    }

    // Query doesn't return default values on missing fields.
    // TODO

    // Query for inclusive inequality on single-index field.
    // TODO

    // Query for exclusive inequality on single-index field.
    // TODO

    // Query works when there is no matching index.
    // TODO

    // ... multi fields

    // ... multi-documents
}
