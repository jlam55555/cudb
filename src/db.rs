//! User-facing structural API of database.

use std::collections::{BTreeMap, HashMap};
use std::path::Path;
use crate::index::*;
use crate::mmapv1::{block, Pool};
use crate::query::FieldPath;

/// User API for connection/client-level actions.
pub struct Client {}

/// User API for database-level actions.
pub struct Database {}

/// User API for collection-level actions.
pub struct Collection {
    pool: Pool,
    indices: HashMap<Index, BTreeMap<IndexInstance, block::Offset>>,
}

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

    /// Create a collection from a path
    pub fn from(path: String) -> Collection {
        let pool_path = Path::new(&path);
        let p = Pool::new(&pool_path);

        Collection {
            pool: p,
            indices: HashMap::new(),
        }
    }

    /// Create a B-tree index on a set of fields in the collection.
    pub fn create_index(&mut self, ind_names: Vec<FieldPath>) -> () {
        let ind = Index::new(ind_names);
        let mut b_tree = BTreeMap::new();

        // ToDo: Make get_const_doc() and remove mut doc
        for mut doc in self.pool.scan() {
            doc.get_doc();
        }

        self.indices.insert(ind, b_tree);
    }

    /// Get all indices created on this collection.
    pub fn get_indices(&self) -> &HashMap<Index, BTreeMap<IndexInstance, block::Offset>> {
        &self.indices
    }
}
