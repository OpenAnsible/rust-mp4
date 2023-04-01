use crate::let_ok;

use super::{Atom, Header, Mp4File};
/*
meco
    mere
*/

#[derive(Debug, Clone)]
pub struct Meco {
    header: Header,
    children: Vec<Atom>,
}

impl Meco {
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
pub struct Mere {
    header: Header,
    first_metabox_handler_type: u32,
    second_metabox_handler_type: u32,
    metabox_relation: u8,
}

impl Mere {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let_ok!(
            first_metabox_handler_type,
            f.read_u32(),
            "Unable to determine first metabox handler type."
        );

        let_ok!(
            second_metabox_handler_type,
            f.read_u32(),
            "Unable to determine second metabox handler type."
        );

        let_ok!(
            metabox_relation,
            f.read_u8(),
            "Unable to determine metabox relation."
        );

        let _offset = f.offset_inc(header.data_size);

        Ok(Self {
            header,
            first_metabox_handler_type,
            second_metabox_handler_type,
            metabox_relation,
        })
    }

    pub const fn header(&self) -> &Header {
        &self.header
    }

    pub const fn first_metabox_handler_type(&self) -> u32 {
        self.first_metabox_handler_type
    }

    pub const fn second_metabox_handler_type(&self) -> u32 {
        self.second_metabox_handler_type
    }

    pub const fn metabox_relation(&self) -> u8 {
        self.metabox_relation
    }
}
