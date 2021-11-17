#[cfg(test)]
pub mod tests {
    use cudb::mmapv1::block::*;

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

    #[test]
    #[should_panic(expected = "document >1MB")]
    fn test_invalid_alloc_size() {
        alloc_size(MAX_BLOCK_SIZE + 1);
    }
}
