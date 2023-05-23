use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{generic_parse, retref};

/// Represents the UUID atom, which is used to store user-defined data, as per ISO/IEC 14496-12:2015 § 8.16.1.
/// This atom is not recognized by this library, so it is just parsed and discarded.
#[derive(Debug, Clone)]
pub struct Uuid {
    /// The header of the atom.
    header: Header,
}

impl Uuid {
    generic_parse!(Uuid);
    retref!(header, Header);
}
