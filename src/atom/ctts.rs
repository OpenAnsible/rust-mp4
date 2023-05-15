//!

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, let_some};

#[derive(Debug, Clone)]
pub struct Ctts {
    pub header: Header,
    pub entry_count: u32,
    pub entries: Vec<CttsEntryOffset>,
}

impl Ctts {
    #[allow(clippy::cast_possible_wrap)]
    /// Parses a `Ctts` atom from the given file. The header is already parsed and passed in.
    /// The file is advanced to the end of the atom.
    ///
    /// In practice, the function reads the data from the atom and returns it as part of the `Ctts` struct.
    /// The `Ctts` atom is required to be present in a valid MP4 file.
    /// It is a full atom, so it has a version and flags.
    ///
    /// # Arguments
    ///
    /// * `f` - The file to read from.
    /// * `header` - The header of the atom.
    ///
    /// # Returns
    ///
    /// * A `Result` containing either the `Ctts` struct or an error message.
    ///
    /// # Errors
    ///
    /// * If the version isn't found, an error is returned.
    /// * If the entry count isn't found, an error is returned.
    /// * If the sample count isn't found, an error is returned.
    /// * If the sample offset isn't found, an error is returned.
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let_some!(version, header.version, "No header version found.");

        let_ok!(entry_count, f.read_u32(), "Unable to read entry count.");
        let mut entries = Vec::new();

        for _ in 0..entry_count {
            let_ok!(sample_count, f.read_u32(), "Unable to read sample count.");
            let sample_offset = if version == 0u8 {
                let_ok!(so, f.read_u32(), "Unable to read sample offset (u32).");
                so as i32
            } else {
                let_ok!(so, f.read_i32(), "Unable to read sample offset (i32).");
                so
            };

            entries.push(CttsEntryOffset {
                sample_count,
                sample_offset,
            });
        }

        f.offset_inc(header.data_size);
        Ok(Self {
            header,
            entry_count,
            entries,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CttsEntryOffset {
    pub sample_count: u32,
    pub sample_offset: i32,
}
