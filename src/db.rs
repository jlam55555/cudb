//! User-facing structural API of database.

use std::collections::{BTreeMap, HashMap};
use std::path::Path;
use crate::index::IndexSchema;
use crate::index::index_instance_module;
use crate::mmapv1::{block, Pool};
use crate::query::FieldPath;

/// User API for connection/client-level actions.
pub struct Client {}

/// User API for database-level actions.
pub struct Database {}

/// User API for collection-level actions.
pub struct Collection {
    pool: Pool,
    indices: HashMap<IndexSchema, BTreeMap<index_instance_module::IndexInstance, block::Offset>>,
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

    /// Create a collection from a path.
    pub fn from(path: String) -> Collection {
        let pool_path = Path::new(&path);
        let p = Pool::new(&pool_path);

        Collection {
            pool: p,
            indices: HashMap::new(),
        }
    }

    /// Create a B-tree index on a list of fields in the collection.
    pub fn create_index(&mut self, ind_names: Vec<FieldPath>) -> () {
        let index_schema = IndexSchema::new(ind_names);
        let index_instance_factory = index_instance_module::IndexInstanceFactory::new(index_schema.clone());

        // ToDo: Make get_const_doc()
        // Loop through all the documents and insert them into the B-tree
        let mut b_tree = BTreeMap::new();
        for mut top_level_doc in self.pool.scan() {
            // Dereference and re-reference to get immutable doc
            let doc = &*top_level_doc.get_doc();

            // Create the index instance for the document
            let index_instance = index_instance_factory.create_index_instance(doc.clone());

            // b_tree.insert()
        }

        self.indices.insert(index_schema, b_tree);
    }

    /// Get all indices created on this collection.
    pub fn get_indices(&self) -> &HashMap<IndexSchema, BTreeMap<index_instance_module::IndexInstance, block::Offset>> {
        &self.indices
    }
}
