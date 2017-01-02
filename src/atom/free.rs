
use super::{Mp4File, Kind, Header, Atom};

/**

aligned(8) class FreeSpaceBox extends Box(free_type) {
    unsigned int(8) data[];
}

**/

#[derive(Debug, Clone)]
pub struct Free {
    header: Header,

}

impl Free {
    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str>{
        let curr_offset = f.offset();
        f.seek(curr_offset+header.data_size);
        f.offset_inc(header.data_size);
        Ok(Free{
            header: header,
        })
    }
}