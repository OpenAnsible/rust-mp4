//!

use super::Header;
use crate::mp4file::Mp4File;

/// Sets up default values used by the movie fragments.
///
/// By setting defaults in this way, space and complexity can be saved in each Track Fragment Box.
///
/// This Atom is currently not implemented in this crate.
///
// TODO: Implement the Trex atom.
#[derive(Debug, Clone)]
pub struct Trex {
    /// The header of the atom.
    pub header: Header,
}

impl Trex {
    /// Parses a `Trex` atom from the given file. The header is already parsed and passed in.
    ///
    /// The file is advanced to the end of the atom. This atom is currently not implemented in this crate, so in practice
    /// the function just advances the file to the end of the atom.
    ///
    /// # Arguments
    ///
    /// * `f` - The file to read from.
    /// * `header` - The header of the atom.
    ///
    /// # Returns
    ///
    /// * A `Result` containing either the `Trex` struct or an error message.
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Self {
        header.parse_version(f);
        header.parse_flags(f);

        let curr_offset = f.offset();
        let _seek_res = f.seek(curr_offset + header.data_size);
        let _offset = f.offset_inc(header.data_size);

        Self { header }
    }
}
