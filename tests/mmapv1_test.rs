use cudb::mmapv1::block::*;
use cudb::mmapv1::*;
use std::path::Path;

#[cfg(test)]
pub mod tests {
    use super::*;

    // Tests for `alloc_size`
    #[test]
    fn test_alloc_size() {
        assert_eq!(alloc_size(0), MIN_BLOCK_SIZE);
        assert_eq!(alloc_size(1), MIN_BLOCK_SIZE);
        assert_eq!(alloc_size(2), MIN_BLOCK_SIZE);
        assert_eq!(alloc_size(3), MIN_BLOCK_SIZE);
        assert_eq!(alloc_size(MIN_BLOCK_SIZE), MIN_BLOCK_SIZE);
        assert_eq!(alloc_size(17), 32);
        assert_eq!(alloc_size(43), 64);
        assert_eq!(alloc_size(23123), 32768);
        assert_eq!(alloc_size(1000000), 1048576);
        assert_eq!(alloc_size(MAX_BLOCK_SIZE), MAX_BLOCK_SIZE);
    }

    // Testing that allocating past `MAX_BLOCK_SIZE` should panic.
    #[test]
    #[should_panic(expected = "document >1MB")]
    fn test_invalid_alloc_size() {
        alloc_size(MAX_BLOCK_SIZE + 1);
    }

    // Testing pool creation/deletion
    #[test]
    fn test_pool_new() {
        let pool_path = "hello.db";
        let p = Pool::new(&pool_path.to_string());

        // check that new pool exists
        if !Path::new(pool_path).exists() {
            panic!("pool was not created");
        }

        // delete pool
        p.delete();

        // check that pool was deleted
        if Path::new(pool_path).exists() {
            panic!("pool was not deleted");
        }
    }
}
