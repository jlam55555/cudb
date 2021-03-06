use cudb::db::Collection;
use cudb::document::Document;
use cudb::index::FieldSpec;
use cudb::query::{Constraint, Query};
use cudb::value::Value;

use rand::Rng;

use std::collections::HashMap;
use std::time::Instant;

fn populate_collection(col: &mut Collection, n: i32) {
    println!("Populating database...");
    let start = Instant::now();

    let mut docs = Vec::new();
    let mut rng = rand::thread_rng();
    for i in 0..n {
        docs.push(Document::from(HashMap::from([
            (
                String::from("randid"),
                Value::Int32(rng.gen::<i32>() % 1000),
            ),
            (String::from("insertion_index"), Value::Int32(i)),
        ])));
    }
    col.insert_many(docs);

    println!("Elapsed: {}ms", start.elapsed().as_millis());
}

fn print_collection_size(col: &Collection) {
    println!("Counting documents...");
    let start = Instant::now();

    println!("Number of documents: {}", col.find_all().len());

    println!("Elapsed: {}ms", start.elapsed().as_millis());
}

fn create_index(col: &mut Collection) {
    println!("Creating index...");
    let start = Instant::now();

    col.declare_index(vec![FieldSpec::new(
        vec![String::from("randid")],
        Value::Int32(0),
    )]);

    println!("Elapsed: {}ms", start.elapsed().as_millis());
}

fn query_lt_100(col: &Collection) {
    println!("Querying elements 0 < x < 100...");
    let start = Instant::now();

    let query = Query {
        constraints: HashMap::from([
            (
                vec![String::from("randid")],
                Constraint::And(
                    Box::new(Constraint::LessThan(Value::Int32(100))),
                    Box::new(Constraint::GreaterThan(Value::Int32(0))),
                ),
            ),
            (
                vec![String::from("insertion_index")],
                Constraint::And(
                    Box::new(Constraint::LessThan(Value::Int32(100))),
                    Box::new(Constraint::GreaterThan(Value::Int32(0))),
                ),
            ),
        ]),
        projection: HashMap::new(),
        order: None,
    };

    let query_results = col.find_many(query);
    println!("Number of results: {}", query_results.len());

    for x in query_results {
        println!("{}", x);
    }

    println!("Elapsed: {}ms", start.elapsed().as_millis());
}

fn main() {
    // Remove old collection
    Collection::from("my_database.db").drop();

    // Create and populate collection
    let mut col = Collection::from("my_database.db");
    populate_collection(&mut col, 100000);
    print_collection_size(&col);

    // Query without index
    query_lt_100(&col);

    // Query with index
    create_index(&mut col);
    query_lt_100(&col);

    // Close and write indices
    col.close();
}
