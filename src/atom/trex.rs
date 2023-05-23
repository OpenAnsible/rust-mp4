//!

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{generic_parse, retref};

/// Sets up default values used by the movie fragments.
///
/// By setting defaults in this way, space and complexity can be saved in each Track Fragment Box.
///
/// This Atom is currently not implemented in this crate.
///
// TODO: Implement the Trex atom.
#[derive(Debug, Clone)]
pub struct Trex {
    /// The header of the atom.
    pub header: Header,
}

impl Trex {
    generic_parse!(Trex);
    retref!(header, Header);
}
