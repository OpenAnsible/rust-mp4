//!

use crate::atom::entry::Entry;
use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref, retval};

#[derive(Debug, Clone)]
pub struct Stsc {
    pub header: Header,
    pub entry_count: u32,
    pub entries: Vec<Entry>,
}

impl Stsc {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let_ok!(entry_count, f.read_u32(), "Unable to read entry count.");
        let mut entries: Vec<Entry> = Vec::new();
        for _entry in 0..entry_count {
            let_ok!(first_chunk, f.read_u32(), "Unable to read first chunk.");
            let_ok!(
                samples_per_chunk,
                f.read_u32(),
                "Unable to read samples per chunk."
            );
            let_ok!(
                sample_description_index,
                f.read_u32(),
                "Unable to read sample description index."
            );

            let entry = Entry {
                first_chunk,
                samples_per_chunk,
                sample_description_index,
            };
            entries.push(entry);
        }

        f.offset_inc(header.data_size);
        Ok(Self {
            header,
            entry_count,
            entries,
        })
    }

    retref!(header, Header);
    retval!(entry_count, u32);
    retref!(entries, Vec<Entry>);
}
