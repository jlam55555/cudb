// A simple implementation of the mmapv1
// storage system; each document is assigned a
// storage block position

use crate::document::*;
use std::fs::File;
use std::os::unix::fs::FileExt;
use std::path::Path;

// TODO: how to associate storage pools with files?
// TODO: de-fragment pool?

pub struct TopLevelDocument {
    // The last allocated memory space; updated on a call to `write()`
    blk: block::Segment,

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
    pub static MIN_BLOCK_SIZE: usize = 16;

    pub type Offset = u64;
    pub type Size = usize;
    pub struct Segment {
        pub off: Offset,
        pub len: Size,
    }

    // Get the real allocation size for a given size
    // This is the smallest power of two (bytes) that will
    // contain the size.
    pub fn alloc_size(len: Size) -> Size {
        if len < MIN_BLOCK_SIZE {
            MIN_BLOCK_SIZE
        } else {
            1 << ((len as f32).log2().ceil() as Size)
        }
    }
}

pub struct Pool {
    free_blocks: Vec<Vec<block::Offset>>,
    top: block::Offset,
    file: File,
}

impl Pool {
    // Create a new memory pool from a file.
    pub fn new(path: &Path) -> Pool {
        // Read file, panic if err
        let file = match File::open(&path) {
            Ok(file) => file,
            Err(e) => panic!("cannot open file {}: {}", path.display(), e),
        };

        // TODO: implement free_blocks
        Pool {
            free_blocks: Vec::new(),
            top: 0,
            file: file,
        }
    }

    // Fetch a top level document from a block address.
    pub fn fetch(&mut self, seg: block::Segment) -> TopLevelDocument {
        let mut buf = vec![0u8; seg.len];
        let size_read = self.file.read_at(&mut buf, seg.off).unwrap();
        if size_read != seg.len {
            panic!("short read on document fetch")
        }

        let doc: Document = bincode::deserialize(&buf).unwrap();
        TopLevelDocument { blk: seg, doc: doc }
    }

    // Update document, return new document.
    // Note that this may require resizing, which will
    //   update the document
    // TODO: refactor this and `write_new_doc` to be DRYer
    pub fn write(&mut self, mut tldoc: TopLevelDocument) -> TopLevelDocument {
        let buf = bincode::serialize(&tldoc.get_doc()).unwrap();

        let old_len = tldoc.blk.len;
        tldoc.blk.len = buf.len();

        // if new document is too large, reallocate it
        if block::alloc_size(tldoc.blk.len) > block::alloc_size(old_len) {
            tldoc.blk.off = self.top;
            self.top += tldoc.blk.len as u64;
        }

        let size_written = self.file.write_at(&buf, tldoc.blk.off).unwrap();
        if size_written != tldoc.blk.len {
            panic!("short write on document fetch");
        }

        tldoc
    }

    // Write a new document, and return the TopLevelDocument.
    pub fn write_new_doc(&mut self, doc: Document) -> TopLevelDocument {
        let buf = bincode::serialize(&doc).unwrap();

        let seg = block::Segment {
            off: self.top,
            len: buf.len(),
        };

        // update file position
        self.top += seg.len as u64;

        let tldoc = TopLevelDocument { blk: seg, doc: doc };

        let size_written = self.file.write_at(&buf, tldoc.blk.off).unwrap();
        if size_written != tldoc.blk.len {
            panic!("short write on document fetch");
        }

        tldoc
    }
}
