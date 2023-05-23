//! Hint Media Header

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref, retval};

/// Hint tracks use the `HintMediaHeaderbox` in the media information box, as defined in ISO/IEC 14495-12:2015 section 8.4.5.
///
/// The hint media header contains general information, independent of the protocol, for hint tracks.
/// (A PDU is a Protocol Data Unit.)
///
/// - Box Types: `Hmhd`
/// - Container:  Media Information Box ([Minf](crate::atom::minf::Minf))
/// - Mandatory: Yes
/// - Quantity:  Exactly one specific media header shall be present
#[derive(Debug, Clone)]
pub struct Hmhd {
    /// The header of the atom.
    pub header: Header,

    /// The size in bytes of the largest PDU in this (hint) stream
    pub max_pdu_size: u16,

    /// The average size in bytes of the PDUs in this (hint) stream
    pub avg_pdu_size: u16,

    /// The maximum rate in bits/second over any window of one second
    pub max_bitrate: u32,

    /// The average rate in bits/second over the entire presentation
    pub avg_bitrate: u32,

    /// Reserved - not used. Should always be 0.
    pub reserved: u32,
}

impl Hmhd {
    /// Parses an `Hmhd` atom from the given MP4 file.
    ///
    /// The file is advanced to the end of the atom.
    ///
    /// # Arguments
    ///
    /// - `f` - The MP4 file to read from.
    /// - `header` - The header of the atom.
    ///
    /// # Returns
    ///
    /// - `Result<Self, &'static str>` - The parsed `Hmhd` atom.
    ///
    /// # Errors
    ///
    /// - If any of the fields cannot be read from the file.
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let_ok!(max_pdu_size, f.read_u16(), "Unable to read max PDU size.");
        let_ok!(
            avg_pdu_size,
            f.read_u16(),
            "Unable to read average PDU size."
        );
        let_ok!(max_bitrate, f.read_u32(), "Unable to read max bitrate.");
        let_ok!(avg_bitrate, f.read_u32(), "Unable to read average bitrate.");
        let_ok!(reserved, f.read_u32(), "Unable to read reserved.");

        // Advance the file offset by the size of the data.
        f.offset_inc(header.data_size);

        Ok(Self {
            header,
            max_pdu_size,
            avg_pdu_size,
            max_bitrate,
            avg_bitrate,
            reserved,
        })
    }

    // Included for completeness.
    retref!(header, Header);
    retval!(max_pdu_size, u16);
    retval!(avg_pdu_size, u16);
    retval!(max_bitrate, u32);
    retval!(avg_bitrate, u32);
}
