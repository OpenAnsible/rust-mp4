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

    pub fn header_ref(&self) -> &Header {
        &self.header
    }

    pub fn header(&self) -> Header {
        self.header.clone()
    }

    pub fn children_ref(&self) -> &Vec<Atom> {
        &self.children
    }

    pub fn children(&self) -> Vec<Atom> {
        self.children.clone()
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

    pub fn header_ref(&self) -> &Header {
        &self.header
    }

    pub fn header(&self) -> Header {
        self.header.clone()
    }

    pub fn sequence_number(&self) -> u32 {
        self.sequence_number
    }
}

/**
8.8.11.1 Definition
Box Type : ‘mfro’
Container: Movie Fragment Random Access Box (‘mfra’)
Mandatory: Yes
Quantity : Exactly one

The Movie Fragment Random Access Offset Box provides a copy of the length
field from the enclosing Movie Fragment Random Access Box. It is placed last
within that box, so that the size field is also last in the enclosing
Movie Fragment Random Access Box. When the Movie Fragment Random Access Box is
also last in the file this permits its easy location. The size field here must be correct.
However, neither the presence of the Movie Fragment Random Access Box, nor its placement
last in the file, are assured.

8.8.11.2 Syntax

aligned(8) class `MovieFragmentRandomAccessOffsetBox` extends FullBox(‘mfro’, version, 0) {
   unsigned int(32)  size;
}

8.8.11.3 Semantics
`size` is an integer gives the number of bytes of the enclosing ‘mfra’ box.
    This field is placed at the last of the enclosing box to assist readers scanning
    from the end of the file in finding the ‘mfra’ box.

**/

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

    pub fn header_ref(&self) -> &Header {
        &self.header
    }

    pub fn header(&self) -> Header {
        self.header.clone()
    }

    pub fn size(&self) -> u32 {
        self.size
    }
}
