//! A simple implementation of a mmapv1-like storage system.
//!
//! A storage pool supports both random-access lookup and scanning
//! (i.e., it is the filesystem analog of a collection). Since documents
//! are variable length, each block has a small fixed-sized header
//! indicating the size of the next document in bytes. Each block
//! is stored as a power of two for efficiency reasons.
use crate::document::*;
use byteorder::{ByteOrder, LittleEndian};
use std::fmt;
use std::os::unix::fs::FileExt;
use std::path::{Path, PathBuf};
use std::{
    fs,
    fs::{File, OpenOptions},
};

// TODO: de-fragment pool? and more advanced allocation methods

/// A wrapper around a `Document` which contains pool allocation information.
#[derive(Debug)]
pub struct TopLevelDocument {
    /// The last allocated memory space; updated on a call to `write()`
    blk: block::Block,

    /// The document associated with this document
    doc: Document,
}

impl TopLevelDocument {
    /// Getter for block; note that the block is immutable, i.e., the
    /// block for a topleveldocument can only be mutated internally.
    pub fn get_block(&self) -> &block::Block {
        &self.blk
    }

    /// Getter for document. The returned document reference is mutable.
    pub fn get_doc(&mut self) -> &mut Document {
        &mut self.doc
    }
}

/// Represents a memory block: offset and size.
pub mod block {
    /// Minimum block size including header, in bytes.
    /// Prevents excessive fragmentation.
    pub static MIN_BLOCK_SIZE: usize = 16;

    /// Maximum block size including header, in bytes.
    pub static MAX_BLOCK_SIZE: usize = 1024 * 1024;

    /// Header size, in bytes.
    pub static HEADER_SIZE: usize = 5;

    /// Indicates a memory offset from the beginning of the pool.
    pub type Offset = u64;

    /// Indicates a memory size.
    pub type Size = usize;

    /// Stores a memory block (for a single document).
    #[derive(Debug, Clone, Copy)]
    pub struct Block {
        /// Position of the header from the start of the pool.
        pub off: Offset,

        /// Length of the data in bytes (not including header).
        /// At most `cap-HEADER_SIZE`.
        pub len: Size,

        /// Capacity of the block in bytes. Is always a power of two,
        /// lying in the range [`MIN_BLOCK_SIZE`, `MAX_BLOCK_SIZE`].
        pub cap: Size,

        /// Whether the block is deleted (for soft deletion).
        pub del: bool,
    }

    /// Get the real allocation size (capacity) for a given size data
    /// (not including header). This is the smallest power of
    /// two (bytes) that will contain the data and header.
    ///
    /// Note that a document may be shrunken, in which case this may
    /// not be the smallest power of two that can fit the data.
    pub fn alloc_size(data_size: Size) -> Size {
        let block_size = data_size + HEADER_SIZE;
        if block_size < MIN_BLOCK_SIZE {
            MIN_BLOCK_SIZE
        } else if block_size > MAX_BLOCK_SIZE {
            panic!("document >1MB")
        } else {
            1 << ((block_size as f32).log2().ceil() as Size)
        }
    }
}

/// A linear collection of contiguous blocks.
///
/// Implements random-access
/// operations: insertion, update (including resizing/reallocation),
/// (soft) deletion. Also implements a linear scanning.
/// Currently uses a very simple (bad) allocation scheme.
#[derive(Debug)]
pub struct Pool {
    // TODO: implement a better allocation scheme
    // free_blocks: Vec<Vec<block::Offset>>,
    top: block::Offset,
    file: File,
    path: PathBuf,
}

impl Pool {
    /// Create a new memory pool from a file, creating the file if necessary.
    pub fn new(path: &Path) -> Pool {
        // Read file, panic if err
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .unwrap();

        // TODO: implement free_blocks and better allocation scheme
        Pool {
            top: file.metadata().unwrap().len(),
            file: file,
            path: PathBuf::from(&path),
        }
    }

    /// Close pool (closes open file).
    pub fn close(self) {
        // Nothing to do, self and self.file will go out of scope and
        // the file will be closed.
    }

    /// Delete pool (deletes file).
    pub fn drop(self) {
        let path = self.path.clone();
        self.close();

        fs::remove_file(path).unwrap();
    }

    /// Gets pool (file) size in bytes.
    pub fn get_size(&self) -> usize {
        self.top as usize
    }

    // Helper function to read and parse (deserialize) a block header.
    // The format of a (5-byte) block header is:
    // - Highest-order bit: set if the block is deleted
    // - Next 7 bits: capacity of current block (2^n bytes)
    // - Next 32 bits: data size in bytes
    // TODO: validate header value
    fn fetch_block_header(&self, off: block::Offset) -> block::Block {
        let mut buf = [0u8; 5];
        self.file.read_exact_at(&mut buf, off).unwrap();

        let header_val = LittleEndian::read_u32(&buf[1..]);
        block::Block {
            del: (buf[0] & (1 << 7)) != 0,
            cap: 1 << (buf[0] & !(1 << 7)),
            len: header_val as usize,
            off: off,
        }
    }

    // Helper function to read block data. See `fetch_block_header` for
    // details about the format of the block header.
    fn fetch_block_data(&self, blk: &block::Block, buf: &mut [u8]) {
        self.file
            .read_exact_at(buf, blk.off + block::HEADER_SIZE as u64)
            .unwrap();
    }

    // Helper function to serialize and write a block header.
    fn write_block_header(&mut self, blk: &block::Block) {
        // Prepare header value
        let mut buf = [0u8; 5];
        let header_val = blk.len;
        buf[0] |= (blk.cap as f64).log2().round() as u8;

        if blk.del {
            buf[0] |= 1 << 7;
        }
        LittleEndian::write_u32(&mut buf[1..], header_val as u32);

        // Write header value
        self.file.write_all_at(&buf, blk.off).unwrap();
    }

    // Helper function to write a document and block header. Assumes
    // block and buf are already correct.
    fn write_block_data(&mut self, blk: &block::Block, buf: &[u8]) {
        self.write_block_header(&blk);
        self.file
            .write_all_at(buf, blk.off + block::HEADER_SIZE as u64)
            .unwrap();
    }

    // Given an existing block and a new length, allocate a new block if necessary.
    // Returns the resultant block, whether it is reallocated or not.
    fn alloc_block(&mut self, mut block: block::Block, new_len: usize) -> block::Block {
        // If we need to allocate a new block
        if block::alloc_size(new_len) > block.cap {
            // Mark old block as deleted
            block.del = true;
            self.write_block_header(&block);

            // Allocate new block
            self.alloc_new_block(new_len)
        } else {
            // Don't need to allocate new block, simply update new length
            block.len = new_len;
            block
        }
    }

    // Helper function to allocate and return a new block with length `len`
    fn alloc_new_block(&mut self, len: usize) -> block::Block {
        let blk = block::Block {
            off: self.top,
            len: len,
            cap: block::alloc_size(len),
            del: false,
        };
        self.top += blk.cap as block::Offset;
        self.file.set_len(self.top).unwrap();

        blk
    }

    /// Fetch a top level document from a block offset, reading the header.
    pub fn fetch_block_at_offset(&self, off: block::Offset) -> TopLevelDocument {
        self.fetch(&self.fetch_block_header(off))
    }

    /// Fetch a top level document from a block address, ignoring the header.
    pub fn fetch(&self, blk: &block::Block) -> TopLevelDocument {
        let mut buf = vec![0u8; blk.len];
        self.fetch_block_data(&blk, &mut buf);

        let doc: Document = bincode::deserialize(&buf).unwrap();
        TopLevelDocument {
            blk: *blk,
            doc: doc,
        }
    }

    /// Delete a block.
    // TODO: perform input validation (i.e., make sure this is a valid block.)
    pub fn delete(&mut self, mut blk: block::Block) {
        blk.del = true;
        self.write_block_header(&blk);
    }

    /// Update document (including_header), return new document.
    /// Note that this may require resizing, which may update the document.
    pub fn write(&mut self, tldoc: &mut TopLevelDocument) {
        let buf = bincode::serialize(&tldoc.get_doc()).unwrap();
        tldoc.blk = self.alloc_block(tldoc.blk, buf.len());
        self.write_block_data(&tldoc.blk, &buf);
    }

    /// Write a new document (including header), and return the TopLevelDocument.
    pub fn write_new(&mut self, doc: Document) -> TopLevelDocument {
        let buf = bincode::serialize(&doc).unwrap();
        let tldoc = TopLevelDocument {
            blk: self.alloc_new_block(buf.len()),
            doc: doc,
        };
        self.write_block_data(&tldoc.blk, &buf);

        tldoc
    }

    /// Linearly scan and retrieve all documents from the pool.
    // TODO: convert the result into a stream for efficiency
    pub fn scan(&self) -> Vec<TopLevelDocument> {
        let mut cur_pos: block::Offset = 0;
        let mut tldocs = Vec::new();

        while cur_pos < self.top {
            let block = self.fetch_block_header(cur_pos);
            if !block.del {
                tldocs.push(self.fetch(&block));
            }
            cur_pos += block.cap as u64;
        }

        tldocs
    }
}

/// Allow pretty-printing of pool.
impl fmt::Display for Pool {
    /// Prints a pool with all of its allocated blocks, for debugging purposes.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Storage pool @{} ({} bytes) {{",
            self.path.display().to_string(),
            self.top
        )
        .unwrap();

        // Perform a scan over blocks, print them all (including deleted ones).
        let mut cur_pos: block::Offset = 0;
        while cur_pos < self.top {
            let block = self.fetch_block_header(cur_pos);
            writeln!(
                f,
                "\toffset: {}\tlength: {}\tcapacity: {}{}",
                block.off,
                block.len,
                block.cap,
                if block.del { "\t(DELETED)" } else { "" },
            )
            .unwrap();
            cur_pos += block.cap as u64;
        }

        writeln!(f, "}}")
    }
}
