//! Bit Rate

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref, retval};

/// Bit Rate atom as defined in ISO/IEC 14496-12:2015 § 8.5.5.2.
#[derive(Debug, Clone)]
pub struct Btrt {
    /// The header of the atom.
    pub header: Header,

    /// Size of the decoding buffer for the elementary stream in bytes.
    pub buffer_size_db: u32,

    /// Maximum rate in bits/second over any window of one second.
    pub max_bitrate: u32,

    /// Average rate in bits/second over the entire presentation.
    pub avg_bitrate: u32,
}

impl Btrt {
    /// Parses a `Btrt` atom from the data in the file.
    ///
    /// # Arguments
    ///
    /// * `f` - The file to read from.
    /// * `header` - The header of the atom.
    ///
    /// # Returns
    ///
    /// * `Result<Self, &'static str>` - The parsed atom.
    ///
    /// # Errors
    ///
    /// - `Unable to read buffer size.` if the buffer size could not be read.
    /// - `Unable to read max bitrate.` if the max bitrate could not be read.
    /// - `Unable to read avg bitrate.` if the avg bitrate could not be read.
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let_ok!(buffer_size_db, f.read_u32(), "Unable to read buffer size.");
        let_ok!(max_bitrate, f.read_u32(), "Unable to read max bitrate.");
        let_ok!(avg_bitrate, f.read_u32(), "Unable to read avg bitrate.");

        f.offset_inc(header.data_size);

        Ok(Self {
            header,
            buffer_size_db,
            max_bitrate,
            avg_bitrate,
        })
    }

    retref!(header, Header);
    retval!(buffer_size_db, u32);
    retval!(max_bitrate, u32);
    retval!(avg_bitrate, u32);
}
