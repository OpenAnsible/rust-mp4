//!

use crate::atom::header::Header;
use crate::let_ok;
use crate::mp4file::Mp4File;

#[derive(Debug, Clone)]
pub struct Hdlr {
    pub header: Header,
    pub handler_type: String,
    pub name: String,
}

impl Hdlr {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let_ok!(b1, f.read_u8(), "Unable to read handler type byte 1");
        let_ok!(b2, f.read_u8(), "Unable to read handler type byte 2");
        let_ok!(b3, f.read_u8(), "Unable to read handler type byte 3");
        let_ok!(b4, f.read_u8(), "Unable to read handler type byte 4");
        let handler_type_bytes: [u8; 4] = [b1, b2, b3, b4];

        let_ok!(
            handler_type,
            String::from_utf8(handler_type_bytes.to_vec()),
            "Unable to get the handler type string from bytes."
        );

        // reserved
        let _throwaway = f.read_u32().unwrap_or_default();
        let _throwaway = f.read_u32().unwrap_or_default();
        let _throwaway = f.read_u32().unwrap_or_default();

        let name_length = header.data_size - 20;
        let mut name_bytes = Vec::new();
        for _bytes in 0..name_length {
            let_ok!(byte, f.read_u8(), "Unable to read name bytes");
            name_bytes.push(byte);
        }

        let_ok!(
            name,
            String::from_utf8(name_bytes),
            "Unable to convert bytes to name."
        );

        f.offset_inc(header.data_size);
        Ok(Self {
            header,
            handler_type,
            name,
        })
    }
}
