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
    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str> {
        let children: Vec<Atom> = Atom::parse_children(f);
        Ok(Self { header, children })
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn children(&self) -> &Vec<Atom> {
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

        let sequence_number: u32 = f.read_u32().unwrap();
        f.offset_inc(header.data_size);
        Ok(Self {
            header,
            sequence_number,
        })
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn sequence_number(&self) -> u32 {
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
        // let curr_offset = f.offset();
        // f.seek(curr_offset+header.data_size);
        let size: u32 = f.read_u32().unwrap();
        f.offset_inc(header.data_size);
        Ok(Self { header, size })
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn size(&self) -> u32 {
        self.size
    }
}
