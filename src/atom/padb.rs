//!

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref, retval};

#[derive(Debug, Clone)]
pub struct Padb {
    pub header: Header,
    pub sample_count: u32,
}

impl Padb {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);
        let curr_offset = f.offset();

        let_ok!(sample_count, f.read_u32(), "Unable to read sample count.");

        let _seek_res = f.seek(curr_offset + header.data_size);
        let _offset = f.offset_inc(header.data_size);

        Ok(Self {
            header,
            sample_count,
        })
    }

    retref!(header, Header);
    retval!(sample_count, u32);
}
