//! Chunk Offset table (`co64` - 64-bit) gives the index of each chunk into the containing file. Ref. [Stco](crate::atom::stco).
//!
//! There are two variants, permitting the use of 32‐bit or 64‐bit offsets.
//! The latter is useful when managing very large presentations.
//! Exactly _one_ of these variants will occur in any single instance of a sample table.
//!
//! Offsets are file offsets, not the offset into any box within the file (e.g. Media Data Box). This permits
//! referring to media data in files without any box structure. It does also mean that care must be taken
//! when constructing a self‐contained ISO file with its metadata (Movie Box) at the front, as the size of the
//! Movie Box will affect the chunk offsets to the media data.
//!
//! - Box Type: `stco` or `co64`
//! - Container: Sample Table Box ([Stbl](crate::atom::stbl))
//! - Mandatory: Yes
//! - Quantity: Exactly one variant must be present

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref, retval};

/// Chunk Offset table (`co64` - 64-bit) gives the index of each chunk into the containing file. Ref. [Stco](crate::atom::stco).
///
/// There are two variants, permitting the use of 32‐bit or 64‐bit offsets.
/// The latter is useful when managing very large presentations.
/// Exactly _one_ of these variants will occur in any single instance of a sample table.
///
/// Offsets are file offsets, not the offset into any box within the file (e.g. Media Data Box). This permits
/// referring to media data in files without any box structure. It does also mean that care must be taken
/// when constructing a self‐contained ISO file with its metadata (Movie Box) at the front, as the size of the
/// Movie Box will affect the chunk offsets to the media data.
#[derive(Debug, Clone)]
pub struct Co64 {
    /// Header of the `Co64` atom. `version = 0.0`
    pub header: Header,

    /// The number of entries in the following vector.
    pub entry_count: u32,

    /// The table of entries.
    pub chunks: Vec<u64>,
}

impl Co64 {
    /// Parses a `Co64` atom from the given file. The header is already parsed and passed in.
    /// The header is updated with version and flags information from the file.
    ///
    /// In practice, the function reads the data from the atom and returns it as part of the `Co64` struct.
    /// The `Co64` atom is required to be present in a valid MP4 file.
    ///
    /// # Arguments
    ///
    /// * `f` - The file to read from.
    /// * `mut header` - The header of the atom. Will be updated with version and flags information.
    ///
    /// # Returns
    ///
    /// * A `Result` containing either the `Co64` struct or an error message.
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let_ok!(entry_count, f.read_u32(), "Unable to read entry count.");
        let mut chunks: Vec<u64> = Vec::with_capacity(entry_count as usize);

        for _entry in 0..entry_count {
            let_ok!(chunk, f.read_u64(), "Unable to read chunk.");
            chunks.push(chunk);
        }

        // Advance the file offset by the size of the data.
        let _offset = f.offset_inc(header.data_size);

        Ok(Self {
            header,
            entry_count,
            chunks,
        })
    }

    // These are defined for consistency more than anything, since the struct entries are public anyway.
    retref!(header, Header);
    retval!(entry_count, u32);
    retref!(chunks, Vec<u64>);
}
