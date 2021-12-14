use std::collections::HashMap;
use rand::Rng;
use cudb::db::Collection;
use cudb::document::Document;
use cudb::query::{Constraint, ConstraintDocument, Query};
use cudb::value::Value;

fn gen_dob(month: i32, day: i32, year: i32) -> Document {
    Document::from(
        HashMap::from([
            (String::from("Month"), Value::Int32(month)),
            (String::from("Day"), Value::Int32(day)),
            (String::from("Year"), Value::Int32(year)),
        ])
    )
}

fn gen_gpa(num: i32) -> Vec<Document> {
    let mut students = Vec::new();
    for _ in 1..num {
        let mut rng = rand::thread_rng();
        let dob = gen_dob(
            (rng.gen::<u8>() % 12) as i32 + 1,
            (rng.gen::<u8>() % 28) as i32 + 1,
            (rng.gen::<u8>() % 30) as i32 + 1990,
        );

        let new_student = Document::from(
            HashMap::from([
                (String::from("name"), Value::String(String::from("Bob"))),
                (String::from("gpa"), Value::Int32((rng.gen::<u16>() % 400) as i32)),
                (String::from("dob"), Value::Dict(dob)),
            ])
        );
        students.push(new_student);
    }

    students
}

fn main() {
    // Remove old collection
    Collection::from("my_database.db").drop();

    // Create and populate collection
    let mut col = Collection::from("my_database.db");

    let students = gen_gpa(10);
    col.insert_many(students);

    let constraint = ConstraintDocument::from([
        (
            vec![String::from("dob"), String::from("Month")],
            Constraint::Equals(Value::Int32(11)),
        )
    ]);

    let updated_dob = gen_dob(11, 22, 200);
    let updated_doc = Document::from(
        HashMap::from([
            (String::from("dob"), Value::Dict(updated_dob)),
        ])
    );

    for doc in col.find_all() {
        println!("{}", doc);
    }
    println!();

    for doc in col.find_many(Query {
        constraints: constraint.clone(),
        projection: HashMap::new(),
        order: None,
    }) {
        println!("{}", doc);
    }

    println!();
    col.update_many(constraint, updated_doc);
    for doc in col.find_all() {
        println!("{}", doc);
    }
    println!();

    // for doc in col.find_many(Query {
    //     constraints: delete_constraint,
    //     projection: HashMap::new(),
    //     order: None,
    // }) {
    //     println!("{}", doc);
    // }

    // Delete
    println!("Deleting students with a GPA under 3.0...");
    let delete_constraint = ConstraintDocument::from([
        (
            vec![String::from("gpa")],
            Constraint::LessThan(Value::Int32(300)),
        )
    ]);
    col.delete_many( delete_constraint.clone());
    for doc in col.find_all() {
        println!("{}", doc.get(&vec![String::from("gpa")]).unwrap());
    }


}