mod document;

fn main() {
    let mut d = document::Document::new();

    d.insert("Hello".to_string(),
             document::Value::Id("12312312541243123".to_string()));
}
