//!

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref, retval};

/// Provides a table of sample counts and durations that can be used to calculate the total number of frames in the track.
#[derive(Debug, Clone)]
pub struct Stts {
    /// The header of the atom.
    pub header: Header,

    /// The number of entries in the table.
    pub entry_count: u32,

    /// The table of entries.
    pub entries: Vec<SttsEntry>,
}

impl Stts {
    /// Parses a Stts atom from the given file. The header is already parsed and passed in.
    /// The file is advanced to the end of the atom.
    /// In practice, the function reads the data from the atom and returns it as part of the Stts struct.
    /// The Stts atom is required to be present in a valid MP4 file.
    /// It is a full atom, so it has a version and flags.
    /// The version determines the size of the `sample_count` and `sample_delta` fields, which are 32-bit or 64-bit.
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let_ok!(entry_count, f.read_u32(), "Unable to read entry count.");
        let mut entries = Vec::new();

        for _entry in 0..entry_count {
            let_ok!(sample_count, f.read_u32(), "Unable to read sample count.");
            let_ok!(sample_delta, f.read_u32(), "Unable to read sample delta.");
            entries.push(SttsEntry {
                sample_count,
                sample_delta,
            });
        }

        f.offset_inc(header.data_size);
        Ok(Self {
            header,
            entry_count,
            entries,
        })
    }

    retref!(header, Header);
    retval!(entry_count, u32);
    retref!(entries, Vec<SttsEntry>);
}

/// Provides a mapping from the presentation time of a sample to the byte offset into the data stream.
#[derive(Debug, Clone)]
pub struct SttsEntry {
    /// The number of consecutive samples that have the same duration.
    pub sample_count: u32,

    /// The delta of these samples in the track's timescale.
    pub sample_delta: u32,
}
