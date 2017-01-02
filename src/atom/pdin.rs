

use super::{Mp4File, Kind, Header, Atom};

/**
BoxTypes : `pdin`
Container: File
Mandatory: No
Quantity : Zero or One

8.1.3.2 Syntax

aligned(8) class ProgressiveDownloadInfoBox extends FullBox(‘pdin’, version = 0, 0) {
	for(i=0;;i++){ //to end of box
		unsigned int(32) rate;
		unsigned int(32) initial_delay;
	}
}

8.1.3.3 Semantics

`rate` is a download rate expressed in bytes/second
`initial_delay` is the suggested delay to use when playing the file, 
	such that if download continues at the given rate, 
	all data within the file will arrive in time for its use and playback should not need to stall.

**/

#[derive(Debug, Clone)]
pub struct Pdin {
    header: Header,
    rate: u32,
    initial_delay: u32
}

impl Pdin {
    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str>{
        let curr_offset = f.offset();
        let rate = f.read_u32().unwrap();
        let initial_delay = f.read_u32().unwrap();
        // f.seek(curr_offset+header.data_size);
        f.offset_inc(header.data_size);
        Ok(Pdin{
            header: header,
            rate: rate,
            initial_delay: initial_delay
        })
    }

}