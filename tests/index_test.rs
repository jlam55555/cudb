// Test cases for indices.

use cudb::db::Collection;
use cudb::index::FieldSpec;
use cudb::value::Value;

#[path = "./utils.rs"]
mod utils;

#[cfg(test)]
pub mod tests {
    use super::*;

    // Create a collection with an index.
    #[test]
    fn test_create_index() {
        let mut col = Collection::from(utils::DB_NAME);

        // Visually check that the documents and indices are created correctly
        for doc in utils::sample_documents(5) {
            println!("Inserting document: {:#?}", doc);
            col.get_mut_pool().write_new(doc);
        }

        col.create_index(vec![
            FieldSpec::new(vec![String::from("key")], Value::Int32(0)),
        ]);
        col.create_index(vec![
            FieldSpec::new(vec![String::from("y")], Value::String(String::from(""))),
        ]);

        println!("Indices: {:#?}", col.get_indices());

        col.drop();
    }

    // TODO: Indices on keys with non-unique values.

    // TODO: Indices on keys that do not exist on all documents.

    // TODO: Indices remain correct after CUD operations.

    // TODO: Query matching chooses a query with the most fields matched.

    // TODO: Query returns correct documents.
}
