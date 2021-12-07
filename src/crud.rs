//! User-facing API for collection-level CRUD operations.

use crate::db::*;
use crate::document::*;
use crate::index::IndexSchema;
use crate::query::*;

use std::collections::HashSet;
use std::iter::FromIterator;

// TODO: most of these should return Result<T,E> types
impl Collection {
    /// Insert one document.
    pub fn insert_one(&mut self, doc: Document) {
        self.get_pool().write_new(doc);
    }

    /// Insert a vector of documents.
    pub fn insert_many(&mut self, docs: Vec<Document>) {
        docs.into_iter().for_each(|doc| self.insert_one(doc));
    }

    /// Fetch at most one document matching the query.
    pub fn find_one(&mut self, query: Query) -> Option<Document> {
        // Get best matching index schema, or an empty index schema otherwise.
        let index_schema = match self.get_best_index_schema(&query.constraints) {
            Some(index_schema) => index_schema.clone(),
            None => IndexSchema::new(vec![]),
        };

        // Get remaining fields that are not part of the index.
        let constraints: HashSet<&FieldPath> = query.constraints.keys().collect();
        let index_constraints: HashSet<&FieldPath> =
            HashSet::from_iter(index_schema.get_fields().iter().clone());
        let remaining_constraints: Vec<&FieldPath> = constraints
            .difference(&index_constraints)
            .map(|x| x.clone())
            .collect();

        // TODO: remove
        // dbg!(&index_schema);
        // dbg!(&constraints);
        // dbg!(&remaining_constraints);

        // Fetch documents that match index.
        // TODO: implement default ID index
        let docs = if index_schema.get_fields().len() > 0 {
            // Index exists, get records that match index.
            self.get_indices().get(&index_schema);

            // TODO: remove; placeholder for now
            self.get_pool().scan()
        } else {
            // No matching index, get all records.
            self.get_pool().scan()
        };

        // Linearly scan docs and find first matching document.
        for doc in docs {
            // TODO: implement ConstraintDocument::matches(document)
            // if remaining_constraints.matches(doc) {
            //     return Some(doc.get_doc().clone());
            // }
        }

        // No match.
        None
    }

    /// Fetch a vector of documents matching the query.
    pub fn find_many(&self, query: Query) -> Vec<Document> {
        Vec::new()
    }

    /// Update at most one document that matches the query.
    pub fn update_one(&self, query: ConstraintDocument, update: UpdateDocument) {}

    /// Update all documents matching the query.
    pub fn update_many(&self, query: ConstraintDocument, update: UpdateDocument) {}

    /// Replace at most one document that matches the query.
    pub fn replace_one(&self, query: ConstraintDocument, replace: Document) {}

    /// Delete at most one document that matches the query.
    pub fn delete_one(&self, query: ConstraintDocument) {}

    /// Delete all documents that match the query.
    pub fn delete_many(&self, query: ConstraintDocument) {}
}
