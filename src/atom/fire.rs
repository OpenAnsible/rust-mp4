//! File Reservoir

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref, retval};

/// Associates the source file identified in the file partition box ([Fpar](crate::atom::fpar::Fpar)) with File
/// reservoirs stored as additional items. It contains a list that starts with the first File reservoir associated
/// with the first source block of the source file and continues sequentially through the source blocks of the
/// source file. Version 1 of `FileReservoirBox` should only be used when support for large `item_ID`
/// or `entry_count` values (exceeding 65535) is required or expected to be required.
///
/// - Box Type:  ‘fire’
/// - Container:  Partition Entry (‘paen’)
/// - Mandatory: No
/// - Quantity:  Zero or One
#[derive(Debug, Clone)]
pub struct Fire {
    /// The header of the atom.
    header: Header,

    /// The number of entries in the following table.
    entry_count: u32,

    /// The entries in the `Fire` atom.
    entries: Vec<FireEntry>,
}

impl Fire {
    /// Parses a `Fire` atom from the given `Mp4File`.
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let version = header.version().unwrap_or(0);

        let entry_count = if version == 0 {
            let_ok!(ec, f.read_u16(), "Unable to read entry count.");
            ec as u32
        } else {
            let_ok!(ec, f.read_u32(), "Unable to read entry count.");
            ec
        };

        let mut entries = Vec::with_capacity(entry_count as usize);
        for _ in 0..entry_count {
            let entry = FireEntry::parse(f, version)?;
            entries.push(entry);
        }

        // Advance the file offset by the size of the data.
        f.offset_inc(header.data_size);

        Ok(Self {
            header,
            entry_count,
            entries,
        })
    }

    retref!(header, Header);
    retval!(entry_count, u32);
    retref!(entries, Vec<FireEntry>);
}

/// Contains the information needed to associate a source block with a File reservoir.
#[derive(Debug, Clone)]
pub struct FireEntry {
    /// The ID of the source block.
    pub item_id: u32,

    /// The ID of the File reservoir.
    pub symbol_count: u32,
}

impl FireEntry {
    /// Parses a `FireEntry` atom from the given `Mp4File`.
    fn parse(f: &mut Mp4File, version: u8) -> Result<Self, &'static str> {
        let item_id = if version == 0 {
            let_ok!(id, f.read_u16(), "Unable to read item ID.");
            id as u32
        } else {
            let_ok!(id, f.read_u32(), "Unable to read item ID.");
            id
        };

        let_ok!(symbol_count, f.read_u32(), "Unable to read symbol count.");

        Ok(Self {
            item_id,
            symbol_count,
        })
    }

    retval!(item_id, u32);
    retval!(symbol_count, u32);
}
