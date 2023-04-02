use super::{Header, Mp4File};
use crate::{let_ok, retref};
use std::string::String;

#[derive(Debug, Clone)]
pub struct Meta {
    header: Header,
}

impl Meta {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let curr_offset = f.offset();
        let Ok(_offset) = f.seek(curr_offset + header.data_size) else {
            return Err("Unable to see file.")
        };
        f.offset_inc(header.data_size);

        Ok(Self { header })
    }

    retref!(header, Header);
}

#[derive(Debug, Clone)]
pub struct Xml {
    header: Header,
    xml: String,
}

impl Xml {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let mut xml_bytes: Vec<u8> = Vec::new();
        for _ in 0..header.data_size {
            let Ok(byte) = f.read_u8() else {
                return Err("Unable to read byte.")
            };
            xml_bytes.push(byte);
        }

        let Ok(xml) = String::from_utf8(xml_bytes) else {
            return Err("Unable to parse XML from utf8")
        };

        f.offset_inc(header.data_size);
        Ok(Self { header, xml })
    }

    retref!(header, Header);
    retref!(xml, String);
}

#[derive(Debug, Clone)]
pub struct Bxml {
    header: Header,
    data: Vec<u8>,
}

impl Bxml {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let mut data: Vec<u8> = Vec::new();
        for _ in 0..header.data_size {
            let_ok!(byte, f.read_u8(), "Unable to read byte.");
            data.push(byte);
        }

        f.offset_inc(header.data_size);
        Ok(Self { header, data })
    }

    retref!(header, Header);
    retref!(data, Vec<u8>);
}
