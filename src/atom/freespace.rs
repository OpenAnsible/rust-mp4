

use super::{Mp4File, Kind, Header, Atom};

/**

skip
    udta
        cprt
        tsel
        strk
            stri
            strd

BoxTypes : ‘free’,‘skip’
Container: File or other box
Mandatory: No
Quantity : Zero or more

The contents of a free-space box are irrelevant and may be ignored, 
or the object deleted, without affecting the presentation. (Care should be exercised 
when deleting the object, as this may invalidate the offsets used in the sample table, 
unless this object is after all the media data).

8.1.2.2 Syntax

aligned(8) class FreeSpaceBox extends Box(free_type) {
    unsigned int(8) data[];
}

8.1.2.3 Semantics

`free_type` may be ‘free’ or ‘skip’.

**/

#[derive(Debug, Clone)]
pub struct Skip {
    header: Header
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

#[derive(Debug, Clone)]
pub struct Free {
    header: Header
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