//! FD Item Information

use crate::atom::atom::Atom;
use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref, retval};

/// FD Item Information
///
/// Defined in ISO/IEC 14496-12:2015 § 8.13.2
///
/// The FD item information box is optional, although it is mandatory for files using FD hint tracks. It
/// provides information on the partitioning of source files and how FD hint tracks are combined into FD
/// sessions. Each partition entry provides details on a particular file partitioning, FEC encoding and
/// associated File and FEC reservoirs. It is possible to provide multiple entries for one source file
/// (identified by its item ID) if alternative FEC encoding schemes or partitionings are used in the file. All
/// partition entries are implicitly numbered and the first entry has number 1.
///
/// - Box Type:  ‘fiin’
/// - Container:  Meta Box (‘meta’)
/// - Mandatory: No
/// - Quantity:  Zero or one
#[derive(Debug, Clone)]
pub struct Fiin {
    /// The header of the atom.
    pub header: Header,

    /// The number of entries in the `Fiin` atom.
    pub entry_count: u16,

    /// The entries in the `Fiin` atom.
    pub partition_entries: Vec<Atom>,

    /// The session information
    pub session_info: Option<Box<Atom>>,

    /// Group ID to Name
    pub group_id_to_name: Option<Box<Atom>>,
}

impl Fiin {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let curr_offset = f.offset();

        let_ok!(entry_count, f.read_u16(), "Unable to read entry count.");

        let mut partition_entries = Vec::with_capacity(entry_count as usize);
        for _ in 0..entry_count {
            partition_entries.push(Atom::parse(f)?);
        }

        // TODO: Figure out how to parse the rest of this atom.
        let session_info = None;
        let group_id_to_name = None;

        // Since we're skipping the rest of the atom, we need to seek to the end of the atom.
        let _seek_res = f.seek(curr_offset + header.data_size);
        let _offset = f.offset_inc(header.data_size);

        Ok(Fiin {
            header,
            entry_count,
            partition_entries,
            session_info,
            group_id_to_name,
        })
    }

    retref!(header, Header);
    retval!(entry_count, u16);
    retref!(partition_entries, Vec<Atom>);
    retref!(session_info, Option<Box<Atom>>);
    retref!(group_id_to_name, Option<Box<Atom>>);
}
