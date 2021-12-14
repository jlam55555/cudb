// Test cases for values.

use cudb::value::Value;

#[path = "./utils.rs"]
mod utils;

#[cfg(test)]
pub mod tests {
    use super::*;
    use cudb::document::Document;
    use std::collections::HashMap;

    // Test each variant in the enum.
    #[test]
    fn test_is_variant_equal_id() {
        let val_id_a = Value::Id("12345".to_string());
        let val_id_b = Value::Id("67890".to_string());
        let val_int = Value::Int32(1);

        assert!(val_id_a.is_variant_equal(&val_id_b));
        assert!(!val_id_a.is_variant_equal(&val_int));
    }

    #[test]
    fn test_is_variant_equal_int32() {
        let val_int_a = Value::Int32(32);
        let val_int_b = Value::Int32(5);
        let val_str = Value::String("abc".to_string());

        assert!(val_int_a.is_variant_equal(&val_int_b));
        assert!(!val_int_a.is_variant_equal(&val_str));
    }

    #[test]
    fn test_is_variant_equal_string() {
        let val_str_a = Value::String("abc".to_string());
        let val_str_b = Value::String("xyz".to_string());
        let val_id = Value::Id("12345".to_string());

        assert!(val_str_a.is_variant_equal(&val_str_b));
        assert!(!val_str_a.is_variant_equal(&val_id));
    }

    #[test]
    fn test_is_variant_equal_dict() {
        let val_dict_a = Value::Dict(Document::from(HashMap::from([(
            "key".to_string(),
            Value::Int32(1),
        )])));
        let val_dict_b = Value::Dict(Document::from(HashMap::from([
            ("abc".to_string(), Value::String("xyz".to_string())),
            ("temp".to_string(), Value::Id("12345".to_string())),
        ])));
        let val_array = Value::Array(Vec::from([
            Value::Int32(5),
            Value::String("abc".to_string()),
        ]));

        assert!(val_dict_a.is_variant_equal(&val_dict_b));
        assert!(!val_dict_a.is_variant_equal(&val_array));
    }

    #[test]
    fn test_is_variant_equal_array() {
        let val_array_a = Value::Array(Vec::from([
            Value::Int32(5),
            Value::String("abc".to_string()),
        ]));
        let val_array_b =
            Value::Array(Vec::from([Value::Id("67890".to_string()), Value::Int32(2)]));
        let val_str = Value::String("temp".to_string());

        assert!(val_array_a.is_variant_equal(&val_array_b));
        assert!(!val_array_a.is_variant_equal(&val_str));
    }

    // Test that update document functionality works.
    #[test]
    fn test_update_from_document() {
        let mut doc = Document::new();
        doc.insert(String::from("a"), Value::String(String::from("hello")));
        doc.insert(String::from("b"), Value::String(String::from("world")));
        let mut subdoc = Document::new();
        subdoc.insert(String::from("d"), Value::Int32(4));
        doc.insert(String::from("c"), Value::Dict(subdoc));

        let update_doc = Document::from(HashMap::from([
            (String::from("a"), Value::String(String::from("new value"))),
            (String::from("e"), Value::Int32(55)),
            (
                String::from("c"),
                Value::Dict(Document::from(HashMap::from([
                    (String::from("d"), Value::String(String::from("updated d"))),
                    (
                        String::from("f"),
                        Value::Dict(Document::from(HashMap::from([(
                            String::from("g"),
                            Value::Int32(-2),
                        )]))),
                    ),
                ]))),
            ),
        ]));

        doc.update_from(&update_doc);

        assert!(
            doc == Document::from(HashMap::from([
                (String::from("a"), Value::String(String::from("new value"))),
                (String::from("b"), Value::String(String::from("world"))),
                (String::from("e"), Value::Int32(55)),
                (
                    String::from("c"),
                    Value::Dict(Document::from(HashMap::from([
                        (String::from("d"), Value::String(String::from("updated d"))),
                        (
                            String::from("f"),
                            Value::Dict(Document::from(HashMap::from([(
                                String::from("g"),
                                Value::Int32(-2),
                            )]))),
                        ),
                    ]))),
                ),
            ]))
        );
    }
}
