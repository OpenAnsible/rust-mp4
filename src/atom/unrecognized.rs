use super::{Header, Mp4File};

#[derive(Debug, Clone)]
pub struct Unrecognized {
    header: Header,
}

impl Unrecognized {
    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str> {
        let curr_offset = f.offset();
        f.seek(curr_offset + header.data_size);
        f.offset_inc(header.data_size);
        Ok(Self { header })
    }
}
