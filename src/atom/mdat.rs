use super::{Atom, Header, Kind, Mp4File};

/**
Box Type: ‘mdat’
Container: File
Mandatory: No
Quantity: Zero or more

**/

#[derive(Debug, Clone)]
pub struct Mdat {
    header: Header,
}

impl Mdat {
    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str> {
        let curr_offset = f.offset();
        f.seek(curr_offset + header.data_size);
        f.offset_inc(header.data_size);
        Ok(Mdat { header: header })
    }
    // pub fn read(&self, buf: &mut [u8]) -> Result<usize>{

    // }
    // pub fn read_to_end(&self, buf: &mut Vec<u8>) -> Result<usize>{

    // }
}
