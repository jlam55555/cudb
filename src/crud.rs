use crate::db::*;
use crate::document::*;
use crate::query::*;

// TODO: most of these should return Result<T,E> types
impl Collection {
    // create
    pub fn insert_one(&self, doc: Document) {}
    pub fn insert_many(&self, docs: Vec<Document>) {}

    // read/query
    pub fn find_one(&self, query: Query) -> Document {
        Document::new()
    }
    pub fn find_many(&self, query: Query) -> Vec<Document> {
        Vec::new()
    }

    // update
    pub fn update_one(&self, query: ConstraintDocument, update: UpdateDocument) {}
    pub fn update_many(&self, query: ConstraintDocument, update: UpdateDocument) {}
    pub fn replace_one(&self, query: ConstraintDocument, replace: Document) {}

    // delete
    pub fn delete_one(&self, query: ConstraintDocument) {}
    pub fn delete_many(&self, query: ConstraintDocument) {}
}
