//!

use crate::atom::atom::Atom;
use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{generic_parse_children, retref};

#[derive(Debug, Clone)]
pub struct Traf {
    header: Header,
    children: Vec<Atom>,
}

impl Traf {
    generic_parse_children!(Traf);

    retref!(header, Header);
    retref!(children, Vec<Atom>);
}
