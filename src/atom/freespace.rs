use crate::retref;

use super::{Header, Mp4File};

#[derive(Debug, Clone)]
pub struct Skip {
    header: Header,
}

impl Skip {
    pub fn parse(f: &mut Mp4File, header: Header) -> Self {
        let curr_offset = f.offset();
        let _throwaway = f.seek(curr_offset + header.data_size);
        let _offset = f.offset_inc(header.data_size);
        Self { header }
    }

    retref!(header, Header);
}

#[derive(Debug, Clone)]
pub struct Free {
    header: Header,
}

impl Free {
    pub fn parse(f: &mut Mp4File, header: Header) -> Self {
        let curr_offset = f.offset();
        let _throwaway = f.seek(curr_offset + header.data_size);
        let _offset = f.offset_inc(header.data_size);
        Self { header }
    }

    retref!(header, Header);
}
