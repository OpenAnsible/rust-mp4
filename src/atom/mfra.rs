use crate::let_ok;

/**

mfra
    tfra
    mfro

**/
use super::{Atom, Header, Mp4File};

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

    pub const fn header(&self) -> &Header {
        &self.header
    }

    pub const fn children(&self) -> &Vec<Atom> {
        &self.children
    }
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

    pub const fn header(&self) -> &Header {
        &self.header
    }

    pub const fn sequence_number(&self) -> u32 {
        self.sequence_number
    }
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

    pub const fn header(&self) -> &Header {
        &self.header
    }

    pub const fn size(&self) -> u32 {
        self.size
    }
}
