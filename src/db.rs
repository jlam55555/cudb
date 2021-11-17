//! User-facing structural API of database.

use crate::index::*;

/// User API for connection/client-level actions.
pub struct Client {}

/// User API for database-level actions.
pub struct Database {}

/// User API for collection-level actions.
pub struct Collection {}

impl Client {
    /// List all databases on the server.
    pub fn list_databases() {}
}

impl Database {
    /// Create a collection in the database.
    pub fn create_collection() {}

    /// Delete a collection in the database.
    pub fn delete_collection() {}

    /// List collections in the database.
    pub fn list_collections() {}
}

impl Collection {
    // aggregate/lookup ... ?

    /// Create a B-tree index on a set of fields in the collection.
    pub fn create_index(ind: Index) {}

    /// Get all indices created on this collection.
    pub fn get_indices() -> Vec<Index> {
        Vec::new()
    }
}
