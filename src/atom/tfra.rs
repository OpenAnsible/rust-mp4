//! Describes the Track Fragment Random Access Box (`Tfra`).

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref, retval};

/// The `Tfra` box contains a table that allows random access to the tracks in a file.
///
/// Each entry contains the location and the presentation time of the sync sample.
/// Note that not every sync sample in the track needs to be listed in the table.
///
/// The absence of this box does not mean that all the samples are sync samples.
/// Random access information in the ‘trun’, ‘traf’ and ‘trex’ shall be set
/// appropriately regardless of the presence of this box.
#[derive(Debug, Clone)]
pub struct Tfra {
    header: Header,
    sequence_number: u32,
}

impl Tfra {
    /// Parses the `Tfra` atom, returning `Self`, meaning that the `Tfra` atom is parsed
    /// and the sequence number is stored in the `sequence_number` field. Further, the offset
    /// of the file is moved ahead by the size of the `Tfra` atom.
    ///
    /// # Arguments
    ///
    /// * `f` - `Mp4File` to read from.
    /// * `header` - `Header` of the `Tfra` atom.
    ///
    /// # Returns
    ///
    /// * `Result<Self, &'static str>` - The parsed `Tfra` atom.
    ///
    /// # Errors
    ///
    /// * `Err` - If the sequence number cannot be read from the file.
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let_ok!(
            sequence_number,
            f.read_u32(),
            "Unable to read sequence number"
        );

        log::trace!("Mfro::parse() -- header = {header:?}, sequence_number = {sequence_number}");

        f.offset_inc(header.data_size);
        Ok(Self {
            header,
            sequence_number,
        })
    }

    retref!(header, Header);
    retval!(sequence_number, u32);
}
