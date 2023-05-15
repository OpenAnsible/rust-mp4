//!

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref, retval};

#[derive(Debug, Clone)]
pub struct Stsz {
    pub header: Header,
    pub sample_size: u32,
    pub sample_count: u32,
    pub entry_size: Option<Vec<u32>>,
}

impl Stsz {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let_ok!(sample_size, f.read_u32(), "Unable to read sample size.");
        let_ok!(sample_count, f.read_u32(), "Unable to read sample count.");

        let entry_size = if sample_size == 0u32 {
            let mut es: Vec<u32> = Vec::new();
            for _sample in 0..sample_count {
                let_ok!(s, f.read_u32(), "Unable to read samples.");
                es.push(s);
            }
            Some(es)
        } else {
            None
        };

        f.offset_inc(header.data_size);

        Ok(Self {
            header,
            sample_size,
            sample_count,
            entry_size,
        })
    }

    retref!(header, Header);
    retval!(sample_size, u32);
    retval!(sample_count, u32);
    retref!(entry_size, Option<Vec<u32>>);
}
