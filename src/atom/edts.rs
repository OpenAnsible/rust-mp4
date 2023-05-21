//! Edit Atom (`Edts`) and its children.

use crate::atom::atom::Atom;
use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{generic_parse_children, retref};

/// An Edit Box maps the presentation time‐line to the media time‐line as it is stored in the file. The Edit
/// Box is a container for the edit lists.
///
/// The Edit Box is optional. In the absence of this box, there is an implicit one‐to‐one mapping of these
/// time‐lines, and the presentation of a track starts at the beginning of the presentation. An empty edit is
/// used to offset the start time of a track.
///
/// - Box Type:  ‘edts’
/// - Container: Track Box (‘trak’)
/// - Mandatory: No
/// - Quantity:  Zero or one
#[derive(Debug, Clone)]
pub struct Edts {
    /// The header of the atom.
    pub header: Header,

    /// A list of atoms contained in this atom. The following atoms may be found within the `Edts` atom:
    ///
    /// - [Elst](crate::atom::elst::Elst)
    pub children: Vec<Atom>,
}

impl Edts {
    generic_parse_children!(Edts);

    retref!(header, Header);
    retref!(children, Vec<Atom>);
}
