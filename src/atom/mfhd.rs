//!

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref, retval};

#[derive(Debug, Clone)]
pub struct Mfhd {
    header: Header,
    sequence_number: u32,
}

impl Mfhd {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let_ok!(
            sequence_number,
            f.read_u32(),
            "Unable to read sequence number."
        );
        let _offset = f.offset_inc(header.data_size);

        log::trace!("Mfhd::parse() -- header = {header:?}, sequence_number = {sequence_number}");

        Ok(Self {
            header,
            sequence_number,
        })
    }

    retref!(header, Header);
    retval!(sequence_number, u32);
}
