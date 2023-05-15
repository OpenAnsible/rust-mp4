//! Defines the `Mfro` atom and its parsing logic.

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref, retval};

/// `Mfro` is a movie fragment random access offset atom. It is used to specify the offset between
/// the end of the `moof` atom and the beginning of the `mdat` atom. This is used to help readers
/// find the `moof` atom.
///
/// The atom is defined in ISO/IEC 14496-12 § 8.8.11
///
/// The Movie Fragment Random Access Offset Box provides a copy of the length field from
/// the enclosing Movie Fragment Random Access Box. It is placed last within that box,
/// so that the size field is also last in the enclosing Movie Fragment Random Access Box.
/// When the Movie Fragment Random Access Box is also last in the file this permits its easy location.
/// The size field here must be correct.
/// However, neither the presence of the Movie Fragment Random Access Box,
/// nor its placement last in the file, are assured.
///
/// - Box Type: `Mfro`
/// - Container: Movie Fragment Random Access Box (`Mfra`)
/// - Mandatory: No
/// - Quantity: Exactly one
#[derive(Debug, Clone)]
pub struct Mfro {
    /// Header of the `Mfro` atom.
    header: Header,

    /// The number of bytes of the enclosing ‘Mfra’ box.
    /// This field is placed at the last of the enclosing box to assist readers
    /// scanning from the end of the file in finding the ‘Mfra’ box.
    size: u32,
}

impl Mfro {
    /// Parses the `Mfro` atom, returning `Self`. This means that the `Mfro` atom is parsed
    /// and the size of the `Mfra` atom is stored in the `size` field. Further, the offset
    /// of the file is moved ahead by the size of the `Mfro` atom.
    ///
    /// # Arguments
    ///
    /// * `f` - `Mp4File` to read from.
    /// * `header` - `Header` of the `Mfro` atom.
    ///
    /// # Returns
    ///
    /// * `Result<Self, &'static str>` - The parsed `Mfro` atom.
    ///
    /// # Errors
    ///
    /// * `Err` - If the size cannot be read from the file.
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let_ok!(size, f.read_u32(), "Unable to read size.");

        f.offset_inc(header.data_size);

        log::trace!("Mfro::parse() -- header = {header:?}, size = {size}");

        Ok(Self { header, size })
    }

    retref!(header, Header);
    retval!(size, u32);
}
