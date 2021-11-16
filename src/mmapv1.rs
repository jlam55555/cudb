// A simple implementation of the mmapv1
// storage system; each document is assigned a
// storage block position

use crate::document::*;

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
    fn get_doc(tldoc: TopLevelDocument) -> Document {
        tldoc.doc
    }
}

// Represents a memory block: offset and size.
// Size is an exponent of 2.
pub mod block {
    pub type Offset = u64;
    pub type Size = u8;
    pub enum Segment {
        Alloc(Offset, Size),
        Unalloc,
    }
}

pub struct Pool {
    free_blocks: Vec<Vec<block::Offset>>,
    top: block::Offset,
}

impl Pool {
    // Fetch a top level document from a block address.
    pub fn fetch(seg: block::Segment) -> TopLevelDocument {
        TopLevelDocument {
            blk: block::Segment::Unalloc,
            doc: Document::new(),
        }
    }

    // Update document, return new document.
    // Note that this may require resizing, which will
    //   update the document
    pub fn write(tldoc: TopLevelDocument) -> TopLevelDocument {
        tldoc
    }

    // Write a new document, and return the TopLevelDocument.
    pub fn write_new_doc(doc: Document) -> TopLevelDocument {
        TopLevelDocument {
            blk: block::Segment::Unalloc,
            doc: doc,
        }
    }
}
