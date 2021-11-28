//! B-tree indexing.

// use compare::{Compare, natural};
use crate::query::FieldPath;

// TODO: implementing indices/B-trees
/// Store the fields used for an index.
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct IndexSchema {
    fields: Vec<FieldPath>,
}

impl IndexSchema {
    pub fn new(fields: Vec<FieldPath>) -> IndexSchema {
        IndexSchema {
            fields: fields,
        }
    }

    pub fn get_fields(&self) -> Vec<FieldPath> {
        self.fields.clone()
    }
}

pub mod index_instance_module {
    use crate::document::Document;
    use crate::index::IndexSchema;
    use crate::value::Value;

    /// Store the values for the fields for a particular document.
    pub struct IndexInstance {
        values: Vec<Value>,
    }

    impl IndexInstance {
        fn new(values: Vec<Value>) -> IndexInstance {
            IndexInstance {
                values: values,
            }
        }
    }

    /// Create IndexInstances for multiple documents given an IndexSchema.
    pub struct IndexInstanceFactory {
        index: IndexSchema
    }

    // ToDo: Support default values here?
    impl IndexInstanceFactory {
        pub fn new(index: IndexSchema) -> IndexInstanceFactory {
            IndexInstanceFactory {
                index: index,
            }
        }

        /// Create an IndexInstance from the provided document.
        pub fn create_index_instance(&self, doc: Document) -> IndexInstance {
            // Extract the values from the document
            let mut values = Vec::new();
            for ind in self.index.get_fields() {
                values.push(doc.get(ind));
            }

            IndexInstance::new(values)
        }
    }
}
