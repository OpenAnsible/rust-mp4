//! Media Fragment Random Access (`Mfra`) atom.

use super::{Atom, Header, Mp4File};
use crate::retref;

/// Represents a Media Fragment Random Access (`Mfra`) atom in an MP4 file.
/// This atom is used to store information about the location of media fragments in the file.
///
/// The atom is defined in ISO/IEC 14496-12 § 8.8.9
///
/// The Movie Fragment Random Access Box (‘mfra’) provides a table which may assist readers in
/// finding sync samples in a file using movie fragments. It contains a track fragment
/// random access box for each track for which information is provided (which may not be all tracks).
/// It is usually placed at or near the end of the file; the last box within the
/// Movie Fragment Random Access Box provides a copy of the length field from the
/// Movie Fragment Random Access Box. Readers may attempt to find this box by examining
/// the last 32 bits of the file, or scanning backwards from the end of the file for a
/// Movie Fragment Random Access Offset Box and using the size information in it,
/// to see if that locates the beginning of a Movie Fragment Random Access Box.
///
/// This box provides only a hint as to where sync samples are; the movie fragments themselves are definitive.
/// It is recommended that readers take care in both locating and using this box as
/// modifications to the file after it was created may render either the pointers,
/// or the declaration of sync samples, incorrect.
///
/// - Box Type: `Mfra`
/// - Container: File
/// - Mandatory: No
/// - Quantity: Zero or one
///
///
#[derive(Debug, Clone)]
pub struct Mfra {
    /// Header of the `Mfra` atom.
    header: Header,

    /// Children of the `Mfra` atom.
    children: Vec<Atom>,
}

impl Mfra {
    /// Parses the `Mfra` atom, returning `Self`. In practice, this function just parses the children of the atom.
    ///
    /// # Arguments
    ///
    /// * `f` - `Mp4File` to read from.
    /// * `header` - `Header` of the `Mfra` atom.
    ///
    /// # Returns
    ///
    /// * `Self` - The parsed `Mfra` atom.
    pub fn parse(f: &mut Mp4File, header: Header) -> Self {
        let children: Vec<Atom> = Atom::parse_children(f);

        log::trace!("Mfro::parse() -- header = {header:?}, children = {children:?}");

        Self { header, children }
    }

    retref!(header, Header);
    retref!(children, Vec<Atom>);
}
