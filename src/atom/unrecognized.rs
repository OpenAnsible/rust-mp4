use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{generic_parse, retref};

/// Represents an atom that is not recognized by this library.
#[derive(Debug, Clone)]
pub struct Unrecognized {
    /// The header of the atom.
    header: Header,
}

impl Unrecognized {
    generic_parse!(Unrecognized);
    retref!(header, Header);
}
