//! Original Format

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref, retval};

/// Original Format atom as defined in ISO/IEC 14496-12:2015 § 8.12.2.
///
/// Contains the four‐character‐code of the original un‐transformed sample description.
///
///
/// - Box Types:  ‘frma’
/// - Container: Protection Scheme Information Box ([Sinf](crate::atom::sinf::Sinf)),
///  Restricted Scheme Information Box ([Rinf](crate::atom::rinf::Rinf)),
/// or Complete Track Information Box ([Cinf](crate::atom::cinf::Cinf))
/// - Mandatory: Yes when used in a protected sample entry, in a restricted sample entry, or in a sample entry for an incomplete track.
/// - Quantity:  Exactly one.
#[derive(Debug, Clone)]
pub struct Frma {
    /// The header of the atom.
    pub header: Header,

    /// The four‐character‐code of the original un‐transformed sample entry (e.g. ‘mp4v’
    /// if the stream contains protected or restricted MPEG-4 visual material).
    pub data_format: u32,

    /// The original format of the media as a string.
    pub data_format_str: String,
}

impl Frma {
    /// Parses an `Frma` atom from the given MP4 file.
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
    /// - `Result<Self, &'static str>` - The parsed `Frma` atom.
    ///
    /// # Errors
    ///
    /// - If any of the fields cannot be read from the file.
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let_ok!(data_format, f.read_u32(), "Unable to read data format.");

        let data_format_str = String::from_utf8_lossy(&data_format.to_be_bytes()).to_string();

        // Advance the file offset by the size of the data.
        f.offset_inc(header.data_size);

        Ok(Self {
            header,
            data_format,
            data_format_str,
        })
    }

    // These are here for completeness, since the struct is public.
    retref!(header, Header);
    retval!(data_format, u32);
    retref!(data_format_str, String);
}
