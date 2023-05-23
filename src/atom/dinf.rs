//! User Data Box (`Dinf')

use crate::atom::atom::Atom;
use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{generic_parse_children, retref};

/// Contains objects that declar use information about the containing box and its data (presentation or track).
///
/// - Box Type:  ‘dinf’
/// - Container: Media Information Box ([Minf](crate::atom::minf::Minf)) or Meta Box ([Meta](crate::atom::meta::Meta))
/// - Mandatory: Yes (required within ‘minf’ box) and No (optional within ‘meta’ box)
/// - Quantity:  Exactly one
#[derive(Debug, Clone)]
pub struct Dinf {
    /// The header of the atom.
    pub header: Header,

    /// A list of atoms contained in this atom. The following atoms may be found within the `Dinf` atom:
    ///
    /// - [Dref](crate::atom::dref::Dref)
    pub children: Vec<Atom>,
}

impl Dinf {
    generic_parse_children!(Udta);
    retref!(header, Header);
    retref!(children, Vec<Atom>);
}
