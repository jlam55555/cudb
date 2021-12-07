//! User-facing API for collection-level CRUD operations.

use crate::db::*;
use crate::document::*;
use crate::query::*;

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
    pub fn find_one(&self, query: Query) -> Option<Document> {
        Some(Document::new())
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
