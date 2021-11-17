// A simple implementation of the storage system based on a single file;
// each document is assigned a storage block (offset, size).
//
// A storage pool supports both random-access lookup and scanning
// (i.e., it is the filesystem analog of a collection). Since documents
// are variable length, each block has a small fixed-sized header
// indicating the size of the next document in bytes. Each block
// is stored as a power of two for efficiency reasons.

use crate::document::*;
use byteorder::{ByteOrder, LittleEndian};
use std::os::unix::fs::FileExt;
use std::path::{Path, PathBuf};
use std::{
    fs,
    fs::{File, OpenOptions},
};

// TODO: de-fragment pool? and more advanced allocation methods

#[derive(Debug)]
pub struct TopLevelDocument {
    // The last allocated memory space; updated on a call to `write()`
    blk: block::Block,

    // The document associated with this document
    doc: Document,
}

impl TopLevelDocument {
    // Getter for document
    fn get_doc(&self) -> &Document {
        &self.doc
    }
}

// Represents a memory block: offset and size.
pub mod block {
    // Block size includes header.
    pub static MIN_BLOCK_SIZE: usize = 16;
    pub static MAX_BLOCK_SIZE: usize = 1024 * 1024;
    pub static HEADER_SIZE: usize = 4;

    // Offset from start of pool.
    pub type Offset = u64;

    // Size of data (not including header).
    pub type Size = usize;

    // Stores a memory block indicating a document.
    // `Offset` is the position of the header from the start of the pool.
    // `Size` is the size of the data (not including the header).
    // The size of the block will be `alloc_size(block)` bytes, including
    // the header.
    #[derive(Debug, Clone)]
    pub struct Block {
        pub off: Offset,
        pub len: Size,
        pub del: bool,
    }

    // Get the real allocation size for a given size data
    // (not including header). This is the smallest power of
    // two (bytes) that will contain the data and header.
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

#[derive(Debug)]
pub struct Pool {
    free_blocks: Vec<Vec<block::Offset>>,
    top: block::Offset,
    file: File,
    path: PathBuf,
}

impl Pool {
    // Create a new memory pool from a file, creating the file if necessary.
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
            free_blocks: Vec::new(),
            top: file.metadata().unwrap().len(),
            file: file,
            path: PathBuf::from(&path),
        }
    }

    // Close pool (closes open file).
    pub fn close(self) {
        // Nothing to do, self and self.file will go out of scope and
        // the file will be closed.
    }

    // Delete pool (deletes file).
    pub fn delete(self) {
        let path = self.path.clone();
        self.close();

        fs::remove_file(path).unwrap();
    }

    // Helper function to read and parse (deserialize) a block header.
    // The format of a (4-byte) block header is:
    // - Highest-order bit: set if the block is deleted
    // - Low-order 31 bits: data size in bytes
    // TODO: validate header value
    fn fetch_block_header(&mut self, off: block::Offset) -> block::Block {
        let mut buf = [0u8; 4];
        let size_read = self.file.read_at(&mut buf, off).unwrap();
        if size_read != block::HEADER_SIZE {
            panic!("short read on document fetch")
        }

        let header_val = LittleEndian::read_u32(&buf);
        block::Block {
            del: (header_val & (1 << 31)) != 0,
            len: (header_val & !(1 << 31)) as usize,
            off: off,
        }
    }

    // Helper function to read block data. See `fetch_block_header` for
    // details about the format of the block header.
    fn fetch_block_data(&mut self, blk: &block::Block, buf: &mut [u8]) {
        let size_read = self
            .file
            .read_at(buf, blk.off + block::HEADER_SIZE as u64)
            .unwrap();
        if size_read != blk.len {
            panic!("short read on document fetch")
        }
    }

    // Helper function to serialize and write a block header.
    fn write_block_header(&mut self, blk: &block::Block) {
        // Prepare header value
        let mut buf = [0u8; 4];
        let mut header_val = blk.len;
        if blk.del {
            header_val |= 1 << 31;
        }
        LittleEndian::write_u32(&mut buf, header_val as u32);

        // Write header value
        let size_written = self.file.write_at(&buf, blk.off).unwrap();
        if size_written != block::HEADER_SIZE {
            panic!("short write");
        }
    }

    // Helper function to write a document and block header. Assumes
    // block and buf are already correct.
    fn write_block_data(&mut self, blk: &block::Block, buf: &[u8]) {
        self.write_block_header(&blk);

        let size_written = self
            .file
            .write_at(buf, blk.off + block::HEADER_SIZE as u64)
            .unwrap();
        if size_written != blk.len {
            panic!("short write");
        }
    }

    // Given an existing block and a new length, allocate a new block if necessary.
    // Returns the resultant block, whether it is reallocated or not.
    fn alloc_block(&mut self, mut block: block::Block, new_len: usize) -> block::Block {
        // If we need to allocate a new block
        if block::alloc_size(new_len) > block::alloc_size(block.len) {
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
        self.top += block::alloc_size(len) as u64;
        block::Block {
            off: self.top,
            len: len,
            del: false,
        }
    }

    // Fetch a top level document from a block address, ignoring the header.
    pub fn fetch(&mut self, blk: &block::Block) -> TopLevelDocument {
        let mut buf = vec![0u8; blk.len];
        self.fetch_block_data(&blk, &mut buf);

        let doc: Document = bincode::deserialize(&buf).unwrap();
        TopLevelDocument {
            blk: blk.clone(),
            doc: doc,
        }
    }

    // Update document (including_header), return new document.
    // Note that this may require resizing, which will
    //   update the document
    pub fn write(&mut self, mut tldoc: TopLevelDocument) -> TopLevelDocument {
        let buf = bincode::serialize(&tldoc.get_doc()).unwrap();
        tldoc.blk = self.alloc_block(tldoc.blk, buf.len());
        self.write_block_data(&tldoc.blk, &buf);

        tldoc
    }

    // Write a new document (including header), and return the TopLevelDocument.
    pub fn write_new_doc(&mut self, doc: Document) -> TopLevelDocument {
        let buf = bincode::serialize(&doc).unwrap();
        let tldoc = TopLevelDocument {
            blk: self.alloc_new_block(buf.len()),
            doc: doc,
        };
        self.write_block_data(&tldoc.blk, &buf);

        tldoc
    }

    // Linearly scan and retrieve all documents from the pool.
    // TODO: convert the result into a stream for efficiency
    pub fn scan(&mut self) -> Vec<TopLevelDocument> {
        let mut cur_pos: block::Offset = 0;
        let mut tldocs = Vec::new();

        while cur_pos < self.top {
            let block = self.fetch_block_header(cur_pos);
            tldocs.push(self.fetch(&block));
            cur_pos += block::alloc_size(block.len) as u64;
        }

        tldocs
    }
}
