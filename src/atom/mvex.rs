//! mvex

use crate::atom::atom::Atom;
use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{generic_parse_children, retref};

#[derive(Debug, Clone)]
pub struct Mvex {
    pub header: Header,
    pub children: Vec<Atom>,
}

impl Mvex {
    generic_parse_children!(Mvex);
    retref!(header, Header);
    retref!(children, Vec<Atom>);
}
