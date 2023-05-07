//! Contains the `Xml` atom, which is used to store XML data.

use super::{Header, Mp4File};
use crate::{let_ok, retref};

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
            let_ok!(byte, f.read_u8(), "Unable to read byte.");
            xml_bytes.push(byte);
        }

        let Ok(xml) = String::from_utf8(xml_bytes) else {
            return Err("Unable to parse XML from utf8")
        };

        log::trace!("Xml::parse() -- header = {header:?}, xml = {xml}");

        f.offset_inc(header.data_size);
        Ok(Self { header, xml })
    }

    retref!(header, Header);
    retref!(xml, String);
}
