use cudb::document::Document;
use cudb::value::Value;
use uuid::Uuid;

#[allow(dead_code)]
pub static DB_NAME: &str = "unit_tests.db";

// Helper function to generate sample documents.
#[allow(dead_code)]
pub fn sample_documents(n: u32) -> Vec<Document> {
    let mut docs = Vec::new();

    // Generate n very-non-random documents
    for i in 0..n {
        let mut doc = Document::new();
        doc.insert(
            "a".repeat((i + 3) as usize),
            Value::Id(Uuid::new_v4().to_string()),
        );
        doc.insert("key".to_string(), Value::Int32((i as i32) * 3 - 42));
        doc.insert("y".to_string(), Value::String("Hi world".to_string()));
        docs.push(doc);
    }

    docs
}
