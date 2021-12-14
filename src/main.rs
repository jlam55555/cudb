use cudb::db::Collection;
use cudb::document::Document;
use cudb::index::FieldSpec;
use cudb::query::{Constraint, Query};
use cudb::value::Value;

use rand::Rng;

use std::collections::HashMap;
use std::time::Instant;

#[allow(dead_code)]
fn populate_collection(col: &mut Collection, n: i32) {
    println!("Populating database...");
    let start = Instant::now();

    let mut docs = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        docs.push(Document::from(HashMap::from([(
            String::from("randid"),
            Value::Int32(rng.gen::<i32>() % 1000),
        )])));
    }
    col.insert_many(docs);

    println!("Elapsed: {}ms", start.elapsed().as_millis());
}

#[allow(dead_code)]
fn print_collection_size(col: &Collection) {
    println!("Counting documents...");
    let start = Instant::now();

    println!("Number of documents: {}", col.find_all().len());

    println!("Elapsed: {}ms", start.elapsed().as_millis());
}

#[allow(dead_code)]
fn drop_collection(col: Collection) {
    println!("Dropping collection...");
    let start = Instant::now();

    col.drop();

    println!("Elapsed: {}ms", start.elapsed().as_millis());
}

#[allow(dead_code)]
fn create_index(col: &mut Collection) {
    println!("Creating index...");
    let start = Instant::now();

    col.declare_index(vec![FieldSpec::new(
        vec![String::from("randid")],
        Value::Int32(0),
    )]);

    println!("Elapsed: {}ms", start.elapsed().as_millis());
}

#[allow(dead_code)]
fn query_lt_100(col: &Collection) {
    println!("Querying elements 0 < x < 100...");
    let start = Instant::now();

    let query = Query {
        constraints: HashMap::from([(
            vec![String::from("randid")],
            Constraint::And(
                Box::new(Constraint::LessThan(Value::Int32(100))),
                Box::new(Constraint::GreaterThan(Value::Int32(0))),
            ),
        )]),
        projection: HashMap::new(),
        order: None,
    };

    let query_results = col.find_many(query);
    println!("Number of results: {}", query_results.len());

    println!("Elapsed: {}ms", start.elapsed().as_millis());
}

#[allow(unused_variables, unused_mut)]
fn main() {
    // Create/open collection
    let mut col = Collection::from("my_database.db");

    // drop_collection(col);

    // populate_collection(&mut col, 100000);
    // print_collection_size(&col);

    // create_index(&mut col);
    // query_lt_100(&col);

    col.close();
}
