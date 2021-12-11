//! User-facing API for collection-level CRUD operations.

use crate::db::Collection;
use crate::document::Document;
use crate::index::IndexSchema;
use crate::query::{ConstraintDocument, ConstraintDocumentTrait, Query, UpdateDocument};

// TODO: most of these should return Result<T,E> types
impl Collection {
    /// Insert one document.
    pub fn insert_one(&mut self, doc: Document) {
        self.get_mut_pool().write_new(doc);
    }

    /// Insert a vector of documents.
    pub fn insert_many(&mut self, docs: Vec<Document>) {
        docs.into_iter().for_each(|doc| self.insert_one(doc));
    }

    /// Fetch at most one document matching the query.
    pub fn find_one(&mut self, query: Query) -> Option<Document> {
        // Get best matching index schema, or an empty index schema otherwise
        let index_schema = match self.get_best_index_schema(&query.constraints) {
            Some(index_schema) => index_schema.clone(),
            None => IndexSchema::new(vec![]),
        };

        // Get remaining fields that are not part of the index
        let remaining_constraints = query.constraints.remove_index_fields(&index_schema);

        // TODO: remove
        dbg!(&index_schema);
        dbg!(&query.constraints);
        dbg!(&remaining_constraints);

        // TODO: implement default ID index
        // Fetch documents that match index
        let tldocs = if index_schema.get_fields().len() > 0 {
            // Index exists, get records that match Index
            let btree = self.get_indices().get(&index_schema).unwrap();

            // Convert Index to b-tree ranges
            let btree_ranges = index_schema.generate_btree_ranges(&query.constraints);

            // Join all ranges
            let mut tldocs = Vec::new();
            for btree_range in btree_ranges {
                for (_, tldoc_off_set) in btree.range(btree_range) {
                    for tldoc_off in tldoc_off_set {
                        tldocs.push(self.get_pool().fetch_block_at_offset(*tldoc_off));
                    }
                }
            }
            tldocs
        } else {
            // No matching index, get all records
            self.get_pool().scan()
        };

        // Linearly scan docs and find first matching Document
        for mut tldoc in tldocs {
            if remaining_constraints.matches_document(&mut tldoc.get_doc()) {
                return Some(tldoc.get_doc().clone());
            }
        }

        // No match
        None
    }

    /// Fetch a vector of documents matching the query.
    pub fn find_many(&self, _query: Query) -> Vec<Document> {
        Vec::new()
    }

    /// Update at most one document that matches the query.
    pub fn update_one(&self, _query: ConstraintDocument, _update: UpdateDocument) {}

    /// Update all documents matching the query.
    pub fn update_many(&self, _query: ConstraintDocument, _update: UpdateDocument) {}

    /// Replace at most one document that matches the query.
    pub fn replace_one(&self, _query: ConstraintDocument, _replace: Document) {}

    /// Delete at most one document that matches the query.
    pub fn delete_one(&self, _query: ConstraintDocument) {}

    /// Delete all documents that match the query.
    pub fn delete_many(&self, _query: ConstraintDocument) {}
}
