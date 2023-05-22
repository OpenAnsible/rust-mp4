//! Group ID to Name

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref, retval};

/// Associates file group names to file group IDs used in the version 1 item
/// information entries in the item information box ([Iinf](crate::atom::iinf::Iinf)).
///
/// - Box Type:  ‘gitn’
/// - Container: FD Information Box ([Fiin](crate::atom::fiin::Fiin))])
/// - Mandatory: No
/// - Quantity:  Zero or One
#[derive(Debug, Clone)]
pub struct Gitn {
    /// The header of the atom.
    pub header: Header,

    /// The number of entries list
    pub entry_count: u16,

    /// The entries in the `Gitn` atom.
    pub group_entries: Vec<GroupEntry>,
}

impl Gitn {
    /// Parses a `Gitn` atom from the given `Mp4File`.
    ///
    /// # Arguments
    ///
    /// * `f` - The `Mp4File` to read the `Gitn` atom from.
    /// * `header` - The `Header` of the `Gitn` atom.
    ///
    /// # Returns
    ///
    /// * A `Gitn` atom if parsing was successful, otherwise an error.
    ///
    /// # Errors
    ///
    /// * If there was an error reading from the `Mp4File`.
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let_ok!(entry_count, f.read_u16(), "Unable to read entry count.");

        let mut group_entries = Vec::with_capacity(entry_count as usize);
        for _ in 0..entry_count {
            let entry = GroupEntry::parse(f)?;
            group_entries.push(entry);
        }

        // Advance the file offset by the size of the data.
        f.offset_inc(header.data_size);

        Ok(Self {
            header,
            entry_count,
            group_entries,
        })
    }

    retref!(header, Header);
    retval!(entry_count, u16);
    retref!(group_entries, Vec<GroupEntry>);
}

#[derive(Debug, Clone)]
pub struct GroupEntry {
    /// The file group ID
    pub group_id: u32,

    /// A null‐terminated string in UTF‐8 characters containing a file group name
    pub group_name: String,
}

impl GroupEntry {
    /// Parses a `GroupEntry` from the given `Mp4File`.
    ///
    /// # Arguments
    ///
    /// * `f` - The `Mp4File` to read the `GroupEntry` from.
    /// * `header` - The `Header` of the `GroupEntry`.
    ///
    /// # Returns
    ///
    /// - A `GroupEntry` if parsing was successful, otherwise an error.
    ///
    /// # Errors
    ///
    /// * If there was an error reading from the `Mp4File`.
    pub fn parse(f: &mut Mp4File) -> Result<Self, &'static str> {
        let_ok!(group_id, f.read_u32(), "Unable to read group ID.");

        let_ok!(
            group_name,
            f.read_null_terminated_string(),
            "Unable to read group name."
        );

        Ok(Self {
            group_id,
            group_name,
        })
    }

    retval!(group_id, u32);
    retref!(group_name, String);
}
