// Test cases for indices.

use cudb::db::Collection;

#[path = "./utils.rs"]
mod utils;

#[cfg(test)]
pub mod tests {
    use super::*;

    // Create a collection with an index.
    #[test]
    fn test_create_index() {
        let mut col = Collection::from(utils::DB_NAME);

        for doc in utils::sample_documents(5) {
            println!("Inserting document: {:#?}", doc);
            col.get_pool().write_new(doc);
        }

        col.create_index(vec![vec![String::from("key")]]);
        col.create_index(vec![vec![String::from("y")]]);

        println!("Pool: {}", col.get_pool());
        println!("Indices: {:#?}", col.get_indices());

        col.drop();
    }

    // Insert and remove values from the index.
    // TODO
}
