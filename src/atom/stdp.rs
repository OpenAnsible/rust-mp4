//!

use super::{Header, Mp4File};

#[derive(Debug, Clone)]
pub struct Stdp {
    pub header: Header,
}

impl Stdp {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Self {
        header.parse_version(f);
        header.parse_flags(f);

        let curr_offset = f.offset();
        let _seek_res = f.seek(curr_offset + header.data_size);
        let _offset = f.offset_inc(header.data_size);

        Self { header }
    }
}
