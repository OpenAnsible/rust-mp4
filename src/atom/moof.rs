use super::{Atom, Header, Mp4File};
use crate::retref;

#[derive(Debug, Clone)]
pub struct Moof {
    header: Header,
    children: Vec<Atom>,
}

impl Moof {
    pub fn parse(f: &mut Mp4File, header: Header) -> Self {
        let children: Vec<Atom> = Atom::parse_children(f);

        log::trace!("Moof::parse() -- header = {header:?}");

        Self { header, children }
    }

    retref!(header, Header);
    retref!(children, Vec<Atom>);
}
