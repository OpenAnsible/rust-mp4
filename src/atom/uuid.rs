use crate::retref;

use super::{Header, Mp4File};

/// Represents the UUID atom, which is used to store user-defined data, as per ISO/IEC 14496-12:2015 § 8.16.1.
/// This atom is not recognized by this library, so it is just parsed and discarded.
#[derive(Debug, Clone)]
pub struct Uuid {
    /// The header of the atom.
    header: Header,
}

impl Uuid {
    /// Parses a Uuid atom from the given file. The header is already parsed and passed in. The file is advanced to the end of the atom.
    /// In practice, this function just reads the data and discards it before advancing the file and returning.
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Self {
        header.parse_usertype(f);

        let curr_offset = f.offset();
        let _seek_res = f.seek(curr_offset + header.data_size);
        let _offset = f.offset_inc(header.data_size);

        Self { header }
    }

    retref!(header, Header);
}
