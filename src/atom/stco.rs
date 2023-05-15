//!

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref, retval};

#[derive(Debug, Clone)]
pub struct Stco {
    pub header: Header,
    pub entry_count: u32,
    pub chunks: Vec<u32>,
}

impl Stco {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let_ok!(entry_count, f.read_u32(), "Unable to read entry count.");
        let mut chunks: Vec<u32> = Vec::new();

        for _entry in 0..entry_count {
            let_ok!(chunk, f.read_u32(), "Unable to read chunk.");
            chunks.push(chunk);
        }

        f.offset_inc(header.data_size);
        Ok(Self {
            header,
            entry_count,
            chunks,
        })
    }

    retref!(header, Header);
    retval!(entry_count, u32);
    retref!(chunks, Vec<u32>);
}
