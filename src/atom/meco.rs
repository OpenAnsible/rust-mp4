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
    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str> {
        let children: Vec<Atom> = Atom::parse_children(f);
        Ok(Self { header, children })
    }
}

#[allow(clippy::doc_markdown)]
/**


**/

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

        // let curr_offset = f.offset();
        // f.seek(curr_offset+header.data_size);
        let first_metabox_handler_type = f.read_u32().unwrap_or(0);
        let second_metabox_handler_type = f.read_u32().unwrap_or(0);
        let metabox_relation = f.read_u8().unwrap_or(1);

        f.offset_inc(header.data_size);
        Ok(Self {
            header,
            first_metabox_handler_type,
            second_metabox_handler_type,
            metabox_relation,
        })
    }
}
