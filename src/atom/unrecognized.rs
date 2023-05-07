use crate::retref;

use super::{Header, Mp4File};

/// Represents an atom that is not recognized by this library.
#[derive(Debug, Clone)]
pub struct Unrecognized {
    /// The header of the atom.
    header: Header,
}

impl Unrecognized {
    /// Parses an Unrecognized atom from the given file. The header is already parsed and passed in. The file is advanced to the end of the atom.
    /// In practice, this function just reads the data and discards it before advancing the file and returning.
    ///
    /// Parameters:
    ///
    /// * `f` - The file to read from.
    /// * `header` - The header of the atom.
    ///
    /// Returns:
    ///
    /// * `Self` - The parsed atom, which in this case basically means we move the offset ahead.
    ///
    pub fn parse(f: &mut Mp4File, header: Header) -> Self {
        let curr_offset = f.offset();

        let _seek_res = f.seek(curr_offset + header.data_size);
        let _offset = f.offset_inc(header.data_size);

        log::trace!("unrecognized::parse() -- header = {header:?}");

        Self { header }
    }

    retref!(header, Header);
}
