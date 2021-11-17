// Test cases for pools. See the README for instructions on running this file.

use cudb::document::Document;
use cudb::mmapv1::block::*;
use cudb::mmapv1::*;
use cudb::value::Value;
use std::path::Path;
use uuid::Uuid;

#[cfg(test)]
pub mod tests {
    use super::*;

    static DB_NAME: &str = "unit_tests.db";

    // Tests for `alloc_size`
    #[test]
    fn test_alloc_size() {
        assert_eq!(alloc_size(0), MIN_BLOCK_SIZE);
        assert_eq!(alloc_size(1), MIN_BLOCK_SIZE);
        assert_eq!(alloc_size(2), MIN_BLOCK_SIZE);
        assert_eq!(alloc_size(3), MIN_BLOCK_SIZE);
        assert_eq!(alloc_size(MIN_BLOCK_SIZE - HEADER_SIZE - 1), MIN_BLOCK_SIZE);
        assert_eq!(alloc_size(MIN_BLOCK_SIZE - HEADER_SIZE), MIN_BLOCK_SIZE);
        assert_eq!(
            alloc_size(MIN_BLOCK_SIZE - HEADER_SIZE + 1),
            2 * MIN_BLOCK_SIZE
        );
        assert_eq!(alloc_size(17), 32);
        assert_eq!(alloc_size(43), 64);
        assert_eq!(alloc_size(23123), 32768);
        assert_eq!(alloc_size(1000000), 1048576);
        assert_eq!(alloc_size(MAX_BLOCK_SIZE - HEADER_SIZE), MAX_BLOCK_SIZE);
    }

    // Testing that allocating past `MAX_BLOCK_SIZE` should panic.
    #[test]
    #[should_panic(expected = "document >1MB")]
    fn test_invalid_alloc_size() {
        alloc_size(MAX_BLOCK_SIZE - HEADER_SIZE + 1);
    }

    // Testing pool creation/deletion
    #[test]
    fn test_pool_new() {
        let pool_path = Path::new(DB_NAME);
        let p = Pool::new(&pool_path);

        // check that new pool exists
        if !pool_path.exists() {
            panic!("pool was not created");
        }

        // drop pool
        p.drop();

        // check that pool was dropped
        if pool_path.exists() {
            panic!("pool was not dropped");
        }
    }

    // Helper function to generate sample documents.
    fn sample_documents(n: u32) -> Vec<Document> {
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

    // Testing that storing records works as expected
    // (e.g., correctly allocating sequentially.)
    // NOTE: this depends on the allocation method, this
    // is currently testing the very basic sequential allocator.
    #[test]
    fn test_pool_alloc() {
        let mut p = Pool::new(&Path::new(DB_NAME));

        let doc = sample_documents(1)[0].clone();

        println!("Pool before insert: {:#?}", p);
        println!("Document: {:#?}", doc);

        let tldoc = p.write_new(doc);

        println!("Pool after insert: {:#?}", p);
        println!("TopLevelDocument: {:#?}", tldoc);

        p.drop();
    }

    // Testing inserting multiple items
    #[test]
    fn test_pool_alloc_multiple() {
        let mut p = Pool::new(&Path::new(DB_NAME));
        println!("Pool before multiple insert: {:#?}", p);

        for doc in sample_documents(16).iter() {
            let tldoc = p.write_new(doc.clone());
            println!("Inserting document: {:#?}", tldoc);
        }

        println!("Pool after multiple insert: {:#?}", p);
        p.drop();
    }

    // Testing that data persists between multiple connections to the database.
    #[test]
    fn test_pool_persistence() {
        let mut p = Pool::new(&Path::new(DB_NAME));
        for doc in sample_documents(16).iter() {
            let tldoc = p.write_new(doc.clone());
            println!("Inserting document: {:#?}", tldoc);
        }
        println!("Pool before closing: {:#?}", p);
        let prev_size = p.get_size();

        // close but don't drop
        p.close();

        // reopen pool
        let p = Pool::new(&Path::new(DB_NAME));
        println!("Pool after re-opening: {:#?}", p);
        let post_size = p.get_size();

        p.drop();

        assert_eq!(prev_size, post_size);
    }

    // Testing that records can be retrieved successfully from
    // database file.
    #[test]
    fn test_pool_fetch() {
        let mut p = Pool::new(&Path::new(DB_NAME));
        let docs = sample_documents(16);
        let mut blocks = Vec::new();

        for doc in docs.iter() {
            blocks.push(*p.write_new(doc.clone()).get_block());
        }

        // Check that returned documents are the same as the inserted ones.
        for (doc, block) in docs.iter().zip(blocks.iter()) {
            assert_eq!(p.fetch(block).get_doc(), doc);
        }

        p.drop();
    }

    // Testing that scanning a pool will return all of its records
    #[test]
    fn test_pool_scan() {
        let mut p = Pool::new(&Path::new(DB_NAME));
        let docs = sample_documents(16);

        for doc in docs.iter() {
            p.write_new(doc.clone());
        }

        // Check that returned documents are the same as the inserted ones.
        // Note that this assumes scanned elements are retrieved in insertion order,
        // which is true for the very simple allocator implemented, but not
        // otherwise necessarily true.
        assert_eq!(
            docs,
            p.scan()
                .iter_mut()
                .map(|tldoc| tldoc.get_doc().clone())
                .collect::<Vec<Document>>()
        );

        p.drop();
    }

    // Test document deletions.
    #[test]
    fn test_pool_delete() {
        let mut p = Pool::new(&Path::new(DB_NAME));
        let docs = sample_documents(6);
        let mut blocks = Vec::new();

        // Insertion/deletion pattern
        // 0 1 2
        // 0   2
        // 0   2 3
        // 0   2
        // 0   2   4 5
        // 0         5
        for doc in 0..3 {
            blocks.push(*p.write_new(docs[doc].clone()).get_block());
        }
        assert_eq!(p.scan().len(), 3);

        p.delete(blocks[1]);
        assert_eq!(p.scan().len(), 2);

        for doc in 3..4 {
            blocks.push(*p.write_new(docs[doc].clone()).get_block());
        }
        assert_eq!(p.scan().len(), 3);

        p.delete(blocks[3]);
        assert_eq!(p.scan().len(), 2);

        for doc in 4..6 {
            blocks.push(*p.write_new(docs[doc].clone()).get_block());
        }
        assert_eq!(p.scan().len(), 4);

        p.delete(blocks[2]);
        p.delete(blocks[4]);
        assert_eq!(p.scan().len(), 2);

        // Check that deletion is idempotent, i.e., re-deleting a block
        // has no effect
        p.delete(blocks[1]);
        p.delete(blocks[2]);
        p.delete(blocks[3]);
        p.delete(blocks[4]);
        assert_eq!(p.scan().len(), 2);

        p.drop();
    }

    // Test document resizing (both larger and smaller)
    #[test]
    fn test_pool_resize() {
        let mut p = Pool::new(&Path::new(DB_NAME));
        let docs = sample_documents(4);
        let mut tldocs = Vec::new();

        // Insert four documents
        // __0__ __1__ __2__ __3__
        // Make 1 smaller
        // __0__ _1_   __2__ __3__
        // Make 2 larger
        // __0__ _1_         __3__ _______2_______

        for doc in docs.iter() {
            tldocs.push(p.write_new(doc.clone()));
        }

        println!("TopLevelDocument 1: {:#?}", tldocs[1]);
        tldocs[1]
            .get_doc()
            .insert("y".to_string(), Value::String("".to_string()));
        p.write(&mut tldocs[1]);
        println!("Smaller TopLevelDocument 1: {:#?}", tldocs[1]);

        println!("TopLevelDocument 2: {:#?}", tldocs[2]);
        tldocs[2]
            .get_doc()
            .insert("y".to_string(), Value::String("a".repeat(1000)));
        p.write(&mut tldocs[2]);
        println!("Larger TopLevelDocument 2: {:#?}", tldocs[2]);

        // Make sure we still see four elements
        assert_eq!(p.scan().len(), 4);

        p.drop();
    }
}
