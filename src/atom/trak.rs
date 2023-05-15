//!

use crate::atom::atom::Atom;
use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{generic_parse_children, retref};

/// Represents the Trak atom, which contains the track information, as per ISO/IEC 14496-12:2015 § 8.3.1.
/// This atom is required to be present in a valid MP4 file. It is a container atom, so it has no data of its own.
/// Known children: Tkhd, Mdia
/// TODO: Implement the rest of the children: Edts, Tref, Udta, Meta
#[derive(Debug, Clone)]
pub struct Trak {
    pub header: Header,
    pub children: Vec<Atom>,
}

impl Trak {
    generic_parse_children!(Trak);
    retref!(header, Header);
    retref!(children, Vec<Atom>);
}
