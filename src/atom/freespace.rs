use super::{Header, Mp4File};

/*

skip
    udta
        cprt
        tsel
        strk
            stri
            strd

Ref: "free skip.md"

*/

#[derive(Debug, Clone)]
pub struct Skip {
    header: Header,
}

impl Skip {
    pub fn parse(f: &mut Mp4File, header: Header) -> Self {
        let curr_offset = f.offset();
        f.seek(curr_offset + header.data_size);
        f.offset_inc(header.data_size);
        Self { header }
    }

    pub fn header_ref(&self) -> &Header {
        &self.header
    }

    pub fn header(&self) -> Header {
        self.header.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Free {
    header: Header,
}

impl Free {
    pub fn parse(f: &mut Mp4File, header: Header) -> Self {
        let curr_offset = f.offset();
        f.seek(curr_offset + header.data_size);
        f.offset_inc(header.data_size);
        Self { header }
    }

    pub fn header_ref(&self) -> &Header {
        &self.header
    }

    pub fn header(&self) -> Header {
        self.header.clone()
    }
}
