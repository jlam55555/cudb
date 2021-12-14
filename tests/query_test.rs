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
    docs[0].insert(String::from("b"), Value::Int32(5));
    let mut subdoc = Document::new();
    subdoc.insert(String::from("d"), Value::String(String::from("hello")));
    subdoc.insert(String::from("e"), Value::String(String::from("world")));
    docs[0].insert(String::from("c"), Value::Dict(subdoc));

    docs[1].insert(String::from("a"), Value::Int32(5));
    docs[1].insert(String::from("b"), Value::Int32(4));

    docs[2].insert(String::from("a"), Value::Int32(2));
    docs[2].insert(String::from("b"), Value::Int32(3));

    docs[3].insert(String::from("a"), Value::Int32(-1));
    docs[3].insert(String::from("b"), Value::Int32(2));
    let mut subdoc = Document::new();
    subdoc.insert(String::from("d"), Value::String(String::from("jon")));
    subdoc.insert(String::from("e"), Value::String(String::from("derek")));
    docs[3].insert(String::from("c"), Value::Dict(subdoc));

    docs[4].insert(String::from("a"), Value::Int32(4));
    docs[4].insert(String::from("b"), Value::Int32(1));

    docs[5].insert(String::from("a"), Value::Int32(4));
    docs[5].insert(String::from("b"), Value::Int32(0));

    docs[6].insert(String::from("a"), Value::Int32(4));
    let mut subdoc = Document::new();
    subdoc.insert(String::from("d"), Value::String(String::from("hello")));
    subdoc.insert(String::from("e"), Value::String(String::from("derek")));
    docs[6].insert(String::from("c"), Value::Dict(subdoc));

    docs[7].insert(String::from("a"), Value::Int32(4));

    docs[8].insert(String::from("a"), Value::Int32(4));

    docs[9].insert(String::from("a"), Value::Int32(4));

    docs
}

// Predicate to check if two vectors of documents are equal.
// Note that documents are unhashable, so can't use a set.
fn are_doc_sets_equal(v1: &Vec<Document>, v2: &Vec<Document>) -> bool {
    assert!(v1.len() == v2.len());

    // Keep track of seen indices
    let mut seen = HashSet::new();

    for d1 in v1 {
        let mut found = false;
        for (i, d2) in v2.iter().enumerate() {
            if d1 == d2 && !seen.contains(&i) {
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
        Collection::from(utils::DB_NAME).drop();
        let mut col = Collection::from(utils::DB_NAME);

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

        assert!(are_doc_sets_equal(&query_results, &true_results));

        col.drop();
    }

    // Query doesn't return default values on missing fields.
    #[test]
    fn test_find_many_no_return_default() {
        Collection::from(utils::DB_NAME).drop();
        let mut col = Collection::from(utils::DB_NAME);

        for doc in fixture() {
            col.get_mut_pool().write_new(doc);
        }

        col.declare_index(vec![FieldSpec::new(
            vec![String::from("b")],
            Value::Int32(3),
        )]);

        let query = Query {
            constraints: HashMap::from([(
                vec![String::from("b")],
                Constraint::LessThan(Value::Int32(4)),
            )]),
            projection: HashMap::new(),
            order: None,
        };

        let query_results = col.find_many(query);

        let true_results = fixture()
            .iter()
            .filter(|doc| match doc.get(&vec![String::from("b")]) {
                Some(value) => value < Value::Int32(4),
                None => false,
            })
            .map(|doc| doc.clone())
            .collect::<Vec<Document>>();

        assert!(are_doc_sets_equal(&query_results, &true_results));

        col.drop();
    }

    // Query for inclusive inequality on single-index field.
    #[test]
    fn test_find_many_inclusive() {
        Collection::from(utils::DB_NAME).drop();
        let mut col = Collection::from(utils::DB_NAME);

        for doc in fixture() {
            col.get_mut_pool().write_new(doc);
        }

        col.declare_index(vec![FieldSpec::new(
            vec![String::from("b")],
            Value::Int32(3),
        )]);

        let query = Query {
            constraints: HashMap::from([(
                vec![String::from("b")],
                Constraint::Or(
                    Box::new(Constraint::LessThan(Value::Int32(4))),
                    Box::new(Constraint::Equals(Value::Int32(4))),
                ),
            )]),
            projection: HashMap::new(),
            order: None,
        };

        let query_results = col.find_many(query);

        let true_results = fixture()
            .iter()
            .filter(|doc| match doc.get(&vec![String::from("b")]) {
                Some(value) => value <= Value::Int32(4),
                None => false,
            })
            .map(|doc| doc.clone())
            .collect::<Vec<Document>>();

        assert!(are_doc_sets_equal(&query_results, &true_results));

        col.drop();
    }

    // Query works when there is no matching index.
    #[test]
    fn test_find_many_scan() {
        // Perform the same as the above, and check that the results match.
        Collection::from(utils::DB_NAME).drop();

        // Query with index
        let mut col = Collection::from(utils::DB_NAME);
        for doc in fixture() {
            col.get_mut_pool().write_new(doc);
        }
        col.declare_index(vec![FieldSpec::new(
            vec![String::from("b")],
            Value::Int32(3),
        )]);
        let query = Query {
            constraints: HashMap::from([(
                vec![String::from("b")],
                Constraint::Or(
                    Box::new(Constraint::LessThan(Value::Int32(4))),
                    Box::new(Constraint::Equals(Value::Int32(4))),
                ),
            )]),
            projection: HashMap::new(),
            order: None,
        };
        let query1_results = col.find_many(query);
        col.drop();

        // Query without index
        let mut col = Collection::from(utils::DB_NAME);
        for doc in fixture() {
            col.get_mut_pool().write_new(doc);
        }
        let query = Query {
            constraints: HashMap::from([(
                vec![String::from("b")],
                Constraint::Or(
                    Box::new(Constraint::LessThan(Value::Int32(4))),
                    Box::new(Constraint::Equals(Value::Int32(4))),
                ),
            )]),
            projection: HashMap::new(),
            order: None,
        };
        let query2_results = col.find_many(query);
        col.drop();

        assert!(are_doc_sets_equal(&query1_results, &query2_results));
    }

    // Test query with constraints over multiple fields matching the index.
    #[test]
    fn test_find_many_multi_field() {
        Collection::from(utils::DB_NAME).drop();

        let mut col = Collection::from(utils::DB_NAME);
        for doc in fixture() {
            col.get_mut_pool().write_new(doc);
        }
        col.declare_index(vec![FieldSpec::new(
            vec![String::from("b")],
            Value::Int32(3),
        )]);
        let query = Query {
            constraints: HashMap::from([
                (
                    vec![String::from("a")],
                    Constraint::And(
                        Box::new(Constraint::GreaterThan(Value::Int32(1))),
                        Box::new(Constraint::Or(
                            Box::new(Constraint::LessThan(Value::Int32(4))),
                            Box::new(Constraint::Equals(Value::Int32(4))),
                        )),
                    ),
                ),
                (
                    vec![String::from("b")],
                    Constraint::Or(
                        Box::new(Constraint::LessThan(Value::Int32(4))),
                        Box::new(Constraint::Equals(Value::Int32(4))),
                    ),
                ),
            ]),
            projection: HashMap::new(),
            order: None,
        };
        let query_results = col.find_many(query);
        col.drop();

        let true_results = fixture()
            .iter()
            .filter(|doc| {
                match (
                    doc.get(&vec![String::from("a")]),
                    doc.get(&vec![String::from("b")]),
                ) {
                    (Some(a_value), Some(b_value)) => {
                        (a_value > Value::Int32(1) && a_value <= Value::Int32(4))
                            && b_value <= Value::Int32(4)
                    }
                    (_, None) | (None, _) => false,
                }
            })
            .map(|doc| doc.clone())
            .collect::<Vec<Document>>();

        assert!(are_doc_sets_equal(&query_results, &true_results));
    }

    // Test query with constraints on subdocument.
    #[test]
    fn test_find_many_subdoc() {
        Collection::from(utils::DB_NAME).drop();

        let mut col = Collection::from(utils::DB_NAME);
        for doc in fixture() {
            col.get_mut_pool().write_new(doc);
        }
        col.declare_index(vec![FieldSpec::new(
            vec![String::from("c"), String::from("d")],
            Value::String(String::from("")),
        )]);
        let query = Query {
            constraints: HashMap::from([(
                vec![String::from("c"), String::from("d")],
                Constraint::Equals(Value::String(String::from("hello"))),
            )]),
            projection: HashMap::new(),
            order: None,
        };
        let query_results = col.find_many(query);
        col.drop();

        let true_results = fixture()
            .iter()
            .filter(
                |doc| match doc.get(&vec![String::from("c"), String::from("d")]) {
                    Some(value) => value == Value::String(String::from("hello")),
                    None => false,
                },
            )
            .map(|doc| doc.clone())
            .collect::<Vec<Document>>();

        assert!(are_doc_sets_equal(&query_results, &true_results));
    }
}
