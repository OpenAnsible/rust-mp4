// Metadata container

use crate::atom::atom::Atom;
use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{generic_parse_children, retref};

// This is the top-level atom for the file. It contains all other atoms. It is a container atom, so it
// has no data of its own. The atom is defined in ISO/IEC 14496-12:2015 § 8.1.1.
#[derive(Debug, Clone)]
pub struct Moov {
    /// The header of the atom.
    header: Header,

    /// The children of the atom, which are the atoms contained in the moov atom.
    children: Vec<Atom>,
}

impl Moov {
    generic_parse_children!(Moov);

    retref!(header, Header);
    retref!(children, Vec<Atom>);
}
