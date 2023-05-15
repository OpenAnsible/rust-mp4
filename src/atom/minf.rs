//!

use crate::atom::atom::Atom;
use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{generic_parse_children, retref};

#[derive(Debug, Clone)]
pub struct Minf {
    pub header: Header,
    pub children: Vec<Atom>, // Box Types: ‘vmhd’, ‘smhd’, ’hmhd’, ‘nmhd’
}

impl Minf {
    generic_parse_children!(Minf);
    retref!(header, Header);
    retref!(children, Vec<Atom>);
}
