use crate::{let_ok, retref};

use super::{Header, Mp4File};

#[derive(Debug, Clone)]
pub struct Ignore {
    header: Header,
}

impl Ignore {
    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str> {
        let curr_offset = f.offset();

        let_ok!(
            _offset,
            f.seek(curr_offset + header.data_size),
            "Unable to seek file."
        );

        f.offset_inc(header.data_size);
        Ok(Self { header })
    }

    retref!(header, Header);
}
