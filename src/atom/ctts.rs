//! Composition Time to Sample atom.

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, read_version, retref, retval};

/// Provides the offset between decoding time and composition time.
///
/// In version 0 of this box the decoding time must be less than the composition time, and the offsets are
/// expressed as unsigned numbers such that CT(n) = DT(n) + CTTS(n) where CTTS(n) is the (uncompressed) table
/// entry for sample n. In version 1 of this box, the composition timeline and the decoding timeline are still
/// derived from each other, but the offsets are signed. It is recommended that for the computed composition
/// timestamps, there is exactly one with the value 0 (zero).
///
/// For either version of the box, each sample must have a unique composition timestamp value, that is, the
/// timestamp for two samples shall never be the same.
///
/// It may be true that there is no frame to compose at time 0; the handling of this is unspecified (systems
/// might display the first frame for longer, or a suitable fill colour).
///
/// When version 1 of this box is used, the CompositionToDecodeBox may also be present in the sample
/// table to relate the composition and decoding timelines. When backwards‐compatibility or compatibility
/// with an unknown set of readers is desired, version 0 of this box should be used when possible. In either
/// version of this box, but particularly under version 0, if it is desired that the media start at track time 0,
/// and the first media sample does not have a composition time of 0, an edit list may be used to ‘shift’ the
/// media to time 0.
///
/// The composition time to sample table is optional and must only be present if DT and CT differ for any
/// samples.
///
/// Hint tracks do not use this box.
///
/// Example:
///
/// |Sample Count|Sample_offset|
/// |------------|-------------|
/// |1           |10           |
/// |1           |30           |
/// |2           |0            |
/// |1           |30           |
/// |2           |0            |
/// |1           |10           |
/// |1           |30           |
/// |2           |0            |
/// |1           |30           |
/// |2           |0            |
///
/// Box Type:  ‘ctts’
/// Container: Sample Table Box (‘stbl’)
/// Mandatory: No
/// Quantity:  Zero or one
#[derive(Debug, Clone)]
pub struct Ctts {
    /// The header of the atom.
    pub header: Header,

    /// The number of entries in the following table.
    pub entry_count: u32,

    /// The entries in the table.
    pub entries: Vec<CttsEntryOffset>,
}

impl Ctts {
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

        let_ok!(entry_count, f.read_u32(), "Unable to read entry count.");
        let mut entries = Vec::with_capacity(entry_count as usize);

        for _ in 0..entry_count {
            let entry = CttsEntryOffset::parse(f, &header)?;
            entries.push(entry);
        }

        // Advance the file offset by the size of the data.
        f.offset_inc(header.data_size);

        Ok(Self {
            header,
            entry_count,
            entries,
        })
    }

    retref!(header, Header);
    retval!(entry_count, u32);
    retref!(entries, Vec<CttsEntryOffset>);
}

/// Provides a mapping between counts of samples and their respective offset times.
#[derive(Debug, Clone)]
pub struct CttsEntryOffset {
    /// The number of consecutive samples that have the given offset.
    pub sample_count: u32,

    /// The offset for the given number of samples. If the version is 0, this is a `u32`. If the version is 1, this is an `i64`.
    /// Gives the offset between CT and DT, such that CT(n) = DT(n) + CTTS(n).
    pub sample_offset: i64,
}

impl CttsEntryOffset {
    /// Parses a `CttsEntryOffset` entry from the given file. The version is already parsed and passed in.
    fn parse(f: &mut Mp4File, header: &Header) -> Result<Self, &'static str> {
        let_ok!(sample_count, f.read_u32(), "Unable to read sample count.");
        read_version!(sample_offset, i64, f.read_u32(), f.read_i32(), header);

        Ok(Self {
            sample_count,
            sample_offset,
        })
    }

    retref!(sample_count, u32);
    retval!(sample_offset, i64);
}
