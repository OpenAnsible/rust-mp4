//! Movie Extends Header

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::read_version;

/// The Movie Extends Header is optional, and provides the overall duration, including fragments, of a fragmented movie.
/// If this box is not present, the overall duration must be computed by examining each fragment.
///
/// - Box Type: `Mehd`
/// - Container:  Movie Extends Box ([Mvex](crate::atom::mvex::Mvex))])
/// - Mandatory: No
/// - Quantity:  Zero or one
#[derive(Debug, Clone)]
pub struct Mehd {
    /// The header of the atom.
    pub header: Header,

    /// Declares length of the presentation of the whole movie including fragments (in the
    /// timescale indicated in the Movie Header Box). The value of this field corresponds
    /// to the duration of the longest track, including movie fragments. If an MP4 file is
    /// created in real‐time, such as used in live streaming, it is not likely that the
    /// `fragment_duration` is known in advance and this box may be omitted.
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

        read_version!(fragment_duration, u64, f.read_u64(), f.read_u32(), header);

        f.offset_inc(header.data_size);

        Ok(Self {
            header,
            fragment_duration,
        })
    }
}
