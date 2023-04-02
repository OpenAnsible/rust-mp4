use super::{Atom, Header, Mp4File};
use crate::{let_ok, retref, retval};

#[derive(Debug, Clone)]
pub struct Mfra {
    header: Header,
    children: Vec<Atom>,
}

impl Mfra {
    pub fn parse(f: &mut Mp4File, header: Header) -> Self {
        let children: Vec<Atom> = Atom::parse_children(f);
        Self { header, children }
    }

    retref!(header, Header);
    retref!(children, Vec<Atom>);
}

#[derive(Debug, Clone)]
pub struct Tfra {
    header: Header,
    sequence_number: u32,
}

impl Tfra {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let_ok!(
            sequence_number,
            f.read_u32(),
            "Unable to read sequence number"
        );

        f.offset_inc(header.data_size);
        Ok(Self {
            header,
            sequence_number,
        })
    }

    retref!(header, Header);
    retval!(sequence_number, u32);
}

#[derive(Debug, Clone)]
pub struct Mfro {
    header: Header,
    size: u32,
}

impl Mfro {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let_ok!(size, f.read_u32(), "Unable to read size.");

        f.offset_inc(header.data_size);
        Ok(Self { header, size })
    }

    retref!(header, Header);
    retval!(size, u32);
}
