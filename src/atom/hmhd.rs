//!

use crate::atom::header::Header;
use crate::let_ok;
use crate::mp4file::Mp4File;

#[derive(Debug, Clone)]
pub struct Hmhd {
    pub header: Header,
    pub max_pdu_size: u16,
    pub avg_pdu_size: u16,
    pub max_bitrate: u32,
    pub avg_bitrate: u32,
}

impl Hmhd {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let_ok!(max_pdu_size, f.read_u16(), "Unable to read max PDU size.");
        let_ok!(
            avg_pdu_size,
            f.read_u16(),
            "Unable to read average PDU size."
        );
        let_ok!(max_bitrate, f.read_u32(), "Unable to read max bitrate.");
        let_ok!(avg_bitrate, f.read_u32(), "Unable to read average bitrate.");

        // reserved
        let _throwaway = f.read_u32().unwrap_or_default();

        f.offset_inc(16);

        Ok(Self {
            header,
            max_pdu_size,
            avg_pdu_size,
            max_bitrate,
            avg_bitrate,
        })
    }
}
