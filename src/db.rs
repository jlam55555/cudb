//! User-facing structural API of database.

use crate::index::{FieldSpec, Index, IndexSchema};
use crate::mmapv1::block::Offset;
use crate::mmapv1::{block, Pool, TopLevelDocument};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::path::Path;

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

        let indices_buf = &p.read_indices();

        let indices = match bincode::deserialize(&indices_buf) {
            Ok(val) => val,
            Err(_) => HashMap::new(),
        };

        Collection {
            pool: p,
            indices: indices,
        }
    }

    /// Get the collection's underlying pool.
    // TODO: replace usages of this with a Collection-level API,
    //       instead of the pool-level API.
    pub fn get_pool(&self) -> &Pool {
        &self.pool
    }

    /// Get the collection's underlying pool mutably.
    pub fn get_mut_pool(&mut self) -> &mut Pool {
        &mut self.pool
    }

    /// Create a B-tree index on a list of fields in the collection.
    /// If a document doesn't have a field, will use the default value instead.
    pub fn declare_index(&mut self, ind_names: Vec<FieldSpec>) {
        let index_schema = IndexSchema::new(ind_names);

        // Check if this Index Schema will conflict with any existing Index Schemas
        // If there is a conflict, stop without creating the new index
        for existing_index_schema in self.indices.keys() {
            if existing_index_schema.is_conflicting(&index_schema) {
                return;
            }
        }

        // Loop through all the documents and insert them into the B-tree
        let mut b_tree = BTreeMap::new();
        for top_level_doc in self.pool.scan() {
            let doc = top_level_doc.get_doc();
            let index = match index_schema.create_index(doc) {
                Some(value) => value,
                None => panic!("mismatched type when creating index"),
            };

            add_index_to_b_tree(&mut b_tree, &index, top_level_doc.get_block().off);
        }

        self.indices.insert(index_schema, b_tree);
    }

    /// Recursive helper function to create every Index for the Document and then insert each Index into the B-tree.
    /// If any Index is invalid, return without inserting any Index.
    fn add_document_to_index(
        &mut self,
        mut index_schema_queue: VecDeque<&IndexSchema>,
        top_level_doc: &TopLevelDocument,
    ) -> bool {
        if index_schema_queue.len() == 0 {
            return true;
        }

        // Pop the queue to prepare for the next recursive call
        let index_schema = index_schema_queue.pop_front().unwrap();

        // Try to create an Index for the Document
        // If invalid, return false
        let doc = top_level_doc.get_doc();
        let index = match index_schema.create_index(doc) {
            Some(value) => value,
            None => return false,
        };

        // If any index is invalid, return false without adding the document
        if !self.add_document_to_index(index_schema_queue, top_level_doc) {
            return false;
        }

        let mut b_tree = self.indices.get_mut(index_schema).unwrap();
        add_index_to_b_tree(&mut b_tree, &index, top_level_doc.get_block().off);

        true
    }

    /// Add a document to all existing indices.
    /// If any Index is invalid, return without inserting any Index.
    pub fn add_document_to_indices(&mut self, top_level_doc: &TopLevelDocument) {
        self.add_document_to_index(self.indices.clone().keys().collect(), top_level_doc);
    }

    /// Get all indices created on this Collection.
    pub fn get_indices(&self) -> &HashMap<IndexSchema, BTreeMap<Index, HashSet<block::Offset>>> {
        &self.indices
    }

    /// Close collection and underlying file pointers.
    pub fn close(mut self) {
        let indices_buf = bincode::serialize(&self.indices).unwrap();

        self.pool.write_indices(&indices_buf);
        self.pool.close();
    }

    /// Drop collection.
    pub fn drop(self) {
        self.pool.drop();
    }

    // TODO: make this private again
    /// Get the Index Schema that most closely matches the provided Constraints.
    /// Only Index Schemas that fully match the Constraints will be considered.
    pub fn get_best_index_schema(&self, constraint_schema: &IndexSchema) -> Option<&IndexSchema> {
        let query_fields = constraint_schema.get_as_hashmap();

        // Get the number of matched fields for each Index Schema
        // Keep Index Schemas if every field inside is in the Constraints
        // Get the first Index Schema with the most matches
        let best_index_schema = self
            .indices
            .keys()
            .map(|x| (x, x.get_num_matched_fields(&query_fields)))
            .filter(|x| x.0.get_fields().len() == x.1 as usize) // Note that we cast i32 to usize (vice versa is unsafe)
            .max_by(|x, y| (*x).1.cmp(&(*y).1))
            .map(|x| x.0);

        best_index_schema
    }
}

/// Insert an Index with its corresponding Offset into a B-tree
fn add_index_to_b_tree(
    b_tree: &mut BTreeMap<Index, HashSet<block::Offset>>,
    index: &Index,
    offset: Offset,
) {
    // ToDo: We can remove this if statement if we use a unique auto-generated id value
    //       for each document
    if !b_tree.contains_key(&index) {
        b_tree.insert(index.clone(), HashSet::new());
    }

    b_tree.get_mut(&index).unwrap().insert(offset);
}
