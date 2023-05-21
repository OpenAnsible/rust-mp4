//! Scheme Information

use crate::atom::atom::Atom;
use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::retref;

/// A container Box that is only interpreted by the scheme being used. Any information the
/// encryption or restriction system needs is stored here. The content of this box is a series
/// of boxes whose type and format are defined by the scheme declared in the Scheme Type Box.
///
/// - Box Types: ‘schi’
/// - Container: Protection Scheme Information Box ([Sinf](crate::atom::sinf::Sinf)),
/// Restricted Scheme Information Box ([Rinf](crate::atom::rinf::Rinf)),
/// or SRTP Process box ([Srpp](crate::atom::srpp::Srpp))]).
/// - Mandatory: No
/// - Quantity:  Zero or one
#[derive(Debug, Clone)]
pub struct Schi {
    /// The header of the atom.
    pub header: Header,

    /// A list of atoms contained in this atom. The following atoms may be found within the `Schi` atom:
    ///
    /// - [Tenc](crate::atom::tenc::Tenc)
    /// - [Link](crate::atom::link::Link)
    pub scheme_specific_data: Vec<Atom>,
}

impl Schi {
    /// Parse an atom from the file.
    ///
    /// # Arguments
    ///
    /// * `f` - The file to read from.
    /// * `header` - The header of the atom.
    ///
    /// # Returns
    ///
    /// * `Self` - The parsed atom, which in this case basically means we move the offset ahead.
    pub fn parse(f: &mut Mp4File, header: Header) -> Self {
        let scheme_specific_data: Vec<Atom> = Atom::parse_children(f);
        log::trace!("Schi::parse() -- scheme_specific_data = {scheme_specific_data:?}");
        Self {
            header,
            scheme_specific_data,
        }
    }

    retref!(header, Header);
    retref!(scheme_specific_data, Vec<Atom>);
}
