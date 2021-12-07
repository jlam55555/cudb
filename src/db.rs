//! User-facing structural API of database.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::Path;
use crate::index::{Index, IndexSchema};
use crate::mmapv1::{block, Pool};
use crate::query::{ConstraintDocument, FieldPath};

/// User API for connection/client-level actions.
pub struct Client {}

/// User API for database-level actions.
pub struct Database {}

/// User API for collection-level actions.
pub struct Collection {
    pool: Pool,
    indices: HashMap<IndexSchema, BTreeMap<Index, HashSet<block::Offset>>>,
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
    pub fn from(path: &str) -> Collection {
        let pool_path = Path::new(path);
        let p = Pool::new(&pool_path);

        Collection {
            pool: p,
            indices: HashMap::new(),
        }
    }

    /// Get the collection's underlying pool.
    // TODO: replace usages of this with a Collection-level API,
    // instead of the pool-level API.
    pub fn get_pool(&mut self) -> &mut Pool {
        &mut self.pool
    }

    /// Create a B-tree index on a list of fields in the collection.
    // TODO: make sure the field exists on all documents, or assign default value.
    //       i.e., `Document::get()` should not panic! if key does not exist
    pub fn create_index(&mut self, ind_names: Vec<FieldPath>) {
        let index_schema = IndexSchema::new(ind_names);

        // ToDo: Make get_const_doc()
        // Loop through all the documents and insert them into the B-tree
        let mut b_tree = BTreeMap::new();
        for mut top_level_doc in self.pool.scan() {
            // Dereference and re-reference to get immutable doc
            let doc = &*top_level_doc.get_doc();

            // Create the index for the document
            let index = index_schema.create_index_instance(doc);

            if !b_tree.contains_key(&index) {
                b_tree.insert(index.clone(), HashSet::new());
            }

            b_tree.get_mut(&index).unwrap().insert(top_level_doc.get_block().off);
        }

        self.indices.insert(index_schema, b_tree);
    }

    /// Get all indices created on this collection.
    pub fn get_indices(&self) -> &HashMap<IndexSchema, BTreeMap<Index, HashSet<block::Offset>>> {
        &self.indices
    }

    /// Close collection and underlying file pointers.
    pub fn close(self) {
        self.pool.close();
    }

    /// Drop collection.
    pub fn drop(self) {
        self.pool.drop();

        // TODO: Drop index. Right now index isn't stored so no problem.
    }

    // TODO: make this private again
    /// Get the index schema that most closely matches the provided constraints.
    /// Only index schemas that fully match the constraints will be considered.
    pub fn get_best_index_schema(&self, constraints: &ConstraintDocument) -> Option<&IndexSchema> {
        let query_fields = constraints.keys().collect();

        // Get the number of matched fields for each index schema
        // Keep index schemas if every field inside is in the constraints
        // Get the first index schema with the max matches
        let best_index_schema = self.indices.keys()
            .map(|x| (x, x.get_num_matched_fields(&query_fields)))
            .filter(|x| x.0.get_fields().len() == x.1 as usize) // Note that we cast i32 to usize (unsafe vice versa)
            .max_by(|x, y| (*x).1.cmp(&(*y).1))
            .map(|x| x.0);

        best_index_schema
    }

}
