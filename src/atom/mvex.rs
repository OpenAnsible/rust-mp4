//! mvex

use super::{Atom, Header};
use crate::mp4file::Mp4File;

#[derive(Debug, Clone)]
pub struct Mvex {
    pub header: Header,
    pub children: Vec<Atom>,
}

impl Mvex {
    pub fn parse(f: &mut Mp4File, header: Header) -> Self {
        let children: Vec<Atom> = Atom::parse_children(f);
        Self { header, children }
    }
}
