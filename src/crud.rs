//! User-facing API for collection-level CRUD operations.

use crate::db::Collection;
use crate::document::Document;
use crate::index::{FieldSpec, IndexSchema};
use crate::mmapv1::TopLevelDocument;
use crate::query::{ConstraintDocument, ConstraintDocumentTrait, Query};

use std::collections::HashMap;

// TODO: most of these should return Result<T,E> types
impl Collection {
    /// Check if a Constraint Document is valid.
    /// If valid, return an Index Schema, where the default values represent the Value variant for that field.
    /// Otherwise, return None.
    fn check_constraints(&self, constraints: &ConstraintDocument) -> Option<IndexSchema> {
        // Get the value type for each Constraint
        // If a Constraint does not have a value (e.g. is invalid), filter it out
        // Convert each Constraint into a Field Spec
        let field_specs: Vec<FieldSpec> = constraints
            .iter()
            .map(|x| (x.0, x.1.get_value_type()))
            .filter(|x| x.1.is_some())
            .map(|x| FieldSpec::new(x.0.clone(), x.1.unwrap()))
            .collect();

        // If a constraint was invalid, it will have been removed, so the lengths will be different
        if field_specs.len() != constraints.len() {
            return Option::None;
        }

        Option::from(IndexSchema::new(field_specs))
    }

    /// Insert one document.
    pub fn insert_one(&mut self, doc: Document) {
        // We try to insert the document to all of the indices first, before we actually write it
        let tldoc = self.get_pool().get_next_offset(doc);
        self.add_document_to_indices(&tldoc);
        self.get_mut_pool().write_new(tldoc.get_doc().clone());
    }

    /// Insert a vector of documents.
    pub fn insert_many(&mut self, docs: Vec<Document>) {
        docs.into_iter().for_each(|doc| self.insert_one(doc));
    }

    // Query helper: transforms a query into a TopLevelDocument iterator.
    fn query(&self, query: Query) -> impl std::iter::Iterator<Item = TopLevelDocument> {
        let constraint_schema = match self.check_constraints(&query.constraints) {
            Some(value) => value,
            None => panic!("invalid constraint"),
        };

        // Get best matching index schema, or an empty index schema otherwise
        let index_schema = match self.get_best_index_schema(&constraint_schema) {
            Some(index_schema) => index_schema.clone(),
            None => IndexSchema::new(vec![]),
        };

        // TODO: remove
        // // Get remaining fields that are not part of the index
        // let remaining_constraints = query.constraints.remove_index_fields(&index_schema);

        // Fetch documents that match index
        // TODO: both of these steps should use an iterator, if possible.
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

        // Perform linear scan to match remaining constraints
        tldocs
            .into_iter()
            .filter(move |tldoc| query.constraints.matches_document(tldoc.get_doc()))
    }

    /// Fetch at most one document matching the query.
    pub fn find_one(&mut self, query: Query) -> Option<Document> {
        // Get first element of iter. Filter is a lazy stream operator so this
        // should short-circuit and be efficient.
        self.query(query)
            .map(|tldoc| tldoc.get_doc().clone())
            .next()
    }

    /// Fetch a vector of documents matching the query.
    pub fn find_many(&self, query: Query) -> Vec<Document> {
        // Linearly scan docs to find matching ones.
        self.query(query)
            .map(|tldoc| tldoc.get_doc().clone())
            .collect()
    }

    /// Fetch all documents from collection.
    pub fn find_all(&self) -> Vec<Document> {
        self.find_many(Query {
            constraints: HashMap::new(),
            projection: HashMap::new(),
            order: None,
        })
    }

    /// Update at most one document that matches the query.
    pub fn update_one(&mut self, query: Query, update: Document) {
        match self.query(query).next() {
            Some(mut tldoc) => {
                tldoc.get_mut_doc().update_from(&update);
                self.get_mut_pool().write(&mut tldoc);
            }
            None => (),
        }
    }

    /// Update all documents matching the query.
    pub fn update_many(&mut self, query: ConstraintDocument, update: Document) {
        self.query(Query {
            constraints: query,
            projection: HashMap::new(),
            order: None,
        })
        .for_each(|mut tldoc| {
            tldoc.get_mut_doc().update_from(&update);
            self.get_mut_pool().write(&mut tldoc);
        })
    }

    /// Delete at most one document that matches the query.
    pub fn delete_one(&mut self, query: Query) {
        match self.query(query).next() {
            Some(tldoc) => self.get_mut_pool().delete(tldoc.get_block().clone()),
            None => (),
        }
    }

    /// Delete all documents that match the query.
    pub fn delete_many(&mut self, query: ConstraintDocument) {
        self.query(Query {
            constraints: query,
            projection: HashMap::new(),
            order: None,
        })
        .for_each(|tldoc| self.get_mut_pool().delete(tldoc.get_block().clone()));
    }
}
