use crate::index::*;

pub struct Client {}

pub struct Database {}

pub struct Collection {}

impl Client {
    pub fn list_databases() {}
}

impl Database {
    pub fn create_collection() {}
    pub fn delete_collection() {}

    pub fn list_collections() {}
}

impl Collection {
    // aggregate/lookup ... ?

    pub fn create_index(ind: Index) {}
    pub fn get_indices() -> Vec<Index> {
        Vec::new()
    }
}
