//! Mehd atom definition and parsing.

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, let_some};

/// The Movie Extends Header is optional, and provides the overall duration, including fragments, of a fragmented movie.
/// If this box is not present, the overall duration must be computed by examining each fragment.
#[derive(Debug, Clone)]
pub struct Mehd {
    pub header: Header,
    pub fragment_duration: u64,
}

impl Mehd {
    /// Parses a `Mehd` atom from the given file. The header is already parsed and passed in.
    ///
    /// The file is advanced to the end of the atom.
    ///
    /// In practice, the function reads the data from the atom and returns it as part of the `Mehd` struct.
    ///
    /// # Arguments
    ///
    /// * `f` - The file to read from.
    /// * `header` - The header of the atom.
    ///
    /// # Returns
    ///
    /// * A `Result` containing either the `Mehd` struct or an error message.
    ///
    /// # Errors
    ///
    /// * If the header version is not set.
    /// * If the fragment duration cannot be read.
    /// * If the file cannot be advanced to the end of the atom.
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let_some!(version, header.version, "No header version set.");
        let fragment_duration: u64 = if version == 1u8 {
            let_ok!(fd, f.read_u64(), "Unable to read fragment duration (u64).");
            fd
        } else {
            let_ok!(fd, f.read_u32(), "Unable to read fragment duration (u32).");
            u64::from(fd)
        };

        f.offset_inc(header.data_size);

        Ok(Self {
            header,
            fragment_duration,
        })
    }
}
