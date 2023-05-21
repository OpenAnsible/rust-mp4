//! FEC Reservoir Box

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, read_version, retref, retval};

/// The FEC reservoir box associates the source file identified in the file partition box (’fpar’) with FEC
/// reservoirs stored as additional items.
///
/// It contains a list that starts with the first FEC reservoir associated
/// with the first source block of the source file and continues sequentially through the source blocks of the
/// source file. Version 1 of FECReservoirBox should only be used when support for large item_ID
/// values and entry_count (exceeding 65535) is required or expected to be required.
///
/// - Box Type:  ‘fecr’
/// - Container:  Partition Entry ([Paen](crate::atom::paen::Paen)])
/// - Mandatory: No
/// - Quantity:  Zero or One
#[derive(Debug, Clone)]
pub struct Fecr {
    /// The header of the atom.
    pub header: Header,

    /// The number of entries in the following table.
    pub entry_count: u32,

    /// The entries in the table.
    pub entries: Vec<FecrEntry>,
}

impl Fecr {
    /// Parses the `Fecr` atom, returning `Self`.
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        read_version!(entry_count, u32, f.read_u16(), f.read_u32(), header);

        let mut entries = Vec::with_capacity(entry_count as usize);
        for _ in 0..entry_count {
            entries.push(FecrEntry::parse(f, &header)?);
        }

        // Advance the file offset by the size of the data.
        f.offset_inc(header.data_size);

        Ok(Fecr {
            header,
            entry_count,
            entries,
        })
    }

    retref!(header, Header);
    retval!(entry_count, u32);
    retref!(entries, Vec<FecrEntry>);
}

/// An entry in the `Fecr` atom.
#[derive(Debug, Clone)]
pub struct FecrEntry {
    /// The ID of the item.
    pub item_id: u32,

    /// The offset of the item.
    pub symbol_count: u32,
}

impl FecrEntry {
    /// Parses a `FecrEntry` atom from the given file. The header is already parsed and passed in.
    fn parse(f: &mut Mp4File, header: &Header) -> Result<Self, &'static str> {
        read_version!(item_id, u32, f.read_u16(), f.read_u32(), header);
        let_ok!(symbol_count, f.read_u32(), "Unable to read symbol count.");

        Ok(FecrEntry {
            item_id,
            symbol_count,
        })
    }

    retval!(item_id, u32);
    retval!(symbol_count, u32);
}
