use super::{Atom, Header, Mp4File};
use crate::retref;

#[derive(Debug, Clone)]
pub struct Traf {
    header: Header,
    children: Vec<Atom>,
}

impl Traf {
    pub fn parse(f: &mut Mp4File, header: Header) -> Self {
        let children: Vec<Atom> = Atom::parse_children(f);

        log::trace!("Mfhd::parse() -- header = {header:?}, children = {children:?}");

        Self { header, children }
    }

    retref!(header, Header);
    retref!(children, Vec<Atom>);
}
