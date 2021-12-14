// Test cases for the query operation.

use cudb::db::Collection;
use cudb::document::Document;
use cudb::index::FieldSpec;
use cudb::query::{Constraint, Query};
use cudb::value::Value;

use std::collections::{HashMap, HashSet};

#[path = "./utils.rs"]
mod utils;

// Test fixture for query tests
fn fixture() -> Vec<Document> {
    let mut docs = vec![Document::new(); 10];

    docs[0].insert(String::from("a"), Value::Int32(0));

    docs[1].insert(String::from("a"), Value::Int32(5));

    docs[2].insert(String::from("a"), Value::Int32(2));

    docs[3].insert(String::from("a"), Value::Int32(-1));

    docs[4].insert(String::from("a"), Value::Int32(4));

    docs[5].insert(String::from("a"), Value::Int32(4));

    docs[6].insert(String::from("a"), Value::Int32(4));

    docs[7].insert(String::from("a"), Value::Int32(4));

    docs[8].insert(String::from("a"), Value::Int32(4));

    docs[9].insert(String::from("a"), Value::Int32(4));

    docs
}

// Predicate to check if two vectors of documents are equal.
// Note that documents are unhashable, so can't use a set.
fn are_doc_sets_equal(v1: Vec<Document>, v2: Vec<Document>) -> bool {
    assert!(v1.len() == v2.len());

    // Keep track of seen indices
    let mut seen = HashSet::new();

    for d1 in v1 {
        let mut found = false;
        for (i, d2) in v2.iter().enumerate() {
            if d1 == d2.clone() && !seen.contains(&i) {
                seen.insert(i);
                found = true;
                break;
            }
        }
        if !found {
            return false;
        }
    }

    true
}

#[cfg(test)]
pub mod tests {
    use super::*;

    // Query chooses maximal matching index.
    // TODO

    // Query doesn't choose index that doesn't match type.
    // TODO

    // Query for equality on single-index field.
    #[test]
    fn test_find_many_single_index() {
        let mut col = Collection::from(utils::DB_NAME);

        // Visually check that the documents and indices are created correctly
        for doc in fixture() {
            col.get_mut_pool().write_new(doc);
        }

        col.declare_index(vec![FieldSpec::new(
            vec![String::from("a")],
            Value::Int32(-12345),
        )]);

        // Create query
        let query = Query {
            constraints: HashMap::from([(
                vec![String::from("a")],
                Constraint::LessThan(Value::Int32(2)),
            )]),
            projection: HashMap::new(),
            order: None,
        };

        let query_results = col.find_many(query);

        let true_results = fixture()
            .iter()
            .filter(|doc| doc.get(&vec![String::from("a")]).unwrap() < Value::Int32(2))
            .map(|doc| doc.clone())
            .collect::<Vec<Document>>();

        assert!(are_doc_sets_equal(query_results, true_results));

        col.drop();
    }

    // Query doesn't return default values on missing fields.
    // TODO

    // Query for inclusive inequality on single-index field.
    // TODO

    // Query for exclusive inequality on single-index field.
    // TODO

    // Query works when there is no matching index.
    // TODO

    // ... multi fields

    // ... multi-documents
}
