//! Media Box (`Mdia`) atom and its children.

use crate::atom::atom::Atom;
use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{generic_parse_children, retref};

/// Media Box (`Mdia`) atom.
///
/// - Box Type:  `mdia`
/// - Container: Track Box ([Trak](crate::atom::trak::Trak)))
/// - Mandatory: Yes
/// - Quantity:  Exactly one
///
/// The media declaration container contains all the objects that declare information about the media data within a track.
#[derive(Debug, Clone)]
pub struct Mdia {
    /// The header of the atom.
    pub header: Header,

    /// A list of atoms contained in this atom. The following atoms may be found within the `Mdia` atom:
    ///
    /// - [Mdhd](crate::atom::mdhd::Mdhd)
    /// - [Hdlr](crate::atom::hdlr::Hdlr)
    /// - [Minf](crate::atom::minf::Minf) (with children)
    pub children: Vec<Atom>,
}

impl Mdia {
    generic_parse_children!(Mdia);
    retref!(header, Header);
    retref!(children, Vec<Atom>);
}
