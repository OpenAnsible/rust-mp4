use super::{Header, Mp4File};

#[derive(Debug, Clone)]
pub struct Uuid {
    header: Header,
}

impl Uuid {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_usertype(f);

        let curr_offset = f.offset();
        let _ = f.seek(curr_offset + header.data_size);
        f.offset_inc(header.data_size);
        Ok(Self { header })
    }

    pub fn header_ref(&self) -> &Header {
        &self.header
    }

    pub fn header(&self) -> Header {
        self.header.clone()
    }
}
