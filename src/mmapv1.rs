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
    pub type Offset = u64;
    pub type Size = usize;
    pub struct Segment {
        pub off: Offset,
        pub len: Size,
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
    pub fn write(&self, mut tldoc: TopLevelDocument) -> TopLevelDocument {
        let mut buf = bincode::serialize(&tldoc.get_doc()).unwrap();

        tldoc.blk.len = buf.len();
        // TODO: if new length is too long, move it

        let size_written = self.file.write_at(&mut buf, tldoc.blk.off).unwrap();
        if size_written != tldoc.blk.len {
            panic!("short write on document fetch");
        }

        tldoc
    }

    // Write a new document, and return the TopLevelDocument.
    pub fn write_new_doc(&self, doc: Document) -> TopLevelDocument {
        let buf = bincode::serialize(&doc).unwrap();

        // TODO: find correct position to allocate document
        let seg = block::Segment {
            off: 0,
            len: buf.len(),
        };

        self.write(TopLevelDocument { blk: seg, doc: doc })
    }
}
