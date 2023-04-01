use super::{Header, Mp4File};

#[derive(Debug, Clone)]
pub struct Unrecognized {
    header: Header,
}

impl Unrecognized {
    pub fn parse(f: &mut Mp4File, header: Header) -> Self {
        let curr_offset = f.offset();

        let _seek_res = f.seek(curr_offset + header.data_size);
        let _offset = f.offset_inc(header.data_size);

        Self { header }
    }

    pub const fn header(&self) -> &Header {
        &self.header
    }
}
