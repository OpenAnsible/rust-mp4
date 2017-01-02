/**

skip
    udta
        cprt
        tsel
        strk
            stri
            strd

aligned(8) class FreeSpaceBox extends Box(free_type) {
    unsigned int(8) data[];
}

**/

use super::{Mp4File, Kind, Header, Atom};

#[derive(Debug, Clone)]
pub struct Skip {
    header: Header,
    // children: Atom,
}

impl Skip {
    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str>{
        let curr_offset = f.offset();
        f.seek(curr_offset+header.data_size);
        f.offset_inc(header.data_size);
        Ok(Skip{
            header: header,
        })
    }
}