//! Data Reference Box (`Dref`)
//!
//! Contains  a  table  of  data  references  ( normally  URLs)  that  declare  the
//! location(s) of the media data used within the presentation.

use crate::atom::atom::Atom;
use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref};

/// Contains a table of data references (normally URLs) that declare the location(s) of the media data used within the presentation.
///
/// The data reference index in the sample description ties entries in this table to the samples in the track.
/// A track may be split over several sources in this way.
/// If the flag is set indicating that the data is in the same file as this box, then no string (not even an empty
/// one) shall be supplied in the entry field.
///
/// The `entry_count` in the DataReferenceBox shall be 1 or greater; each `DataEntryBox` within the
/// `DataReferenceBox` shall be either a `DataEntryUrnBox` or a `DataEntryUrlBox`.
///
/// _NOTE: Though the count is 32 bits, the number of items is usually much fewer, and is restricted by the fact that
/// the reference index in the sample table is only 16 bits._
///
/// When a file that has data entries with the flag set indicating that the media data is in the same file, is
/// split into segments for transport, the value of this flag does not change, as the file is (logically)
/// reassembled after the transport operation.
///
/// - Box Types: ‘dref’
/// - Container: Data Information Box ([Dinf](crate::atom::dinf::Dinf))
/// - Mandatory: Yes
/// - Quantity: Exactly one
#[derive(Debug, Clone)]
pub struct Dref {
    /// The header of the atom.
    pub header: Header,

    /// The number of entries in the following table.
    pub entry_count: u32,

    /// A list of atoms contained in this atom. The following atoms may be found within the `Dref` atom:
    ///
    /// - [Url](crate::atom::url::Url)
    /// - [Urn](crate::atom::urn::Urn)
    pub children: Vec<Atom>,
}

impl Dref {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let_ok!(entry_count, f.read_u32(), "Unable to read entry count.");
        let mut children = Vec::with_capacity(entry_count as usize);

        for _ in 0..entry_count {
            let child = Atom::parse(f)?;
            children.push(child);
        }

        // Advance the file offset by the size of the data.
        f.offset_inc(header.data_size);

        Ok(Self {
            header,
            entry_count,
            children,
        })
    }

    retref!(header, Header);
    retref!(entry_count, u32);
    retref!(children, Vec<Atom>);
}
