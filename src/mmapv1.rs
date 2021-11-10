// A simple implementation of the mmapv1
// storage system; each document is assigned a
// storage block position

// TODO: how to associate storage pools with files?
// TODO: de-fragment pool?

// Represents an mmap-ed file position.
type Position = u64;

struct Pool {
    free_blocks: Vec<Vec<Position>>,
    top: Position,
}

struct Block {
    pos: Position,
    size: i8, // size in bytes (2^size)
    data: i8, // TODO: create raw data buffer
}

// TODO: unsafe rust to read raw data into document
