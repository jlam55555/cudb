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
}
