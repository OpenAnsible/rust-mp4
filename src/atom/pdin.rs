use super::{Header, Mp4File};

/**
`BoxTypes` : `pdin`
Container: File
Mandatory: No
Quantity : Zero or One

8.1.3.2 Syntax

aligned(8) class `ProgressiveDownloadInfoBox` extends FullBox(‘pdin’, version = 0, 0) {
    for(i=0;;i++){ //to end of box
        unsigned int(32) rate;
        unsigned int(32) `initial_delay`;
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
    initial_delay: u32,
}

impl Pdin {
    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str> {
        let rate = f.read_u32().unwrap();
        let initial_delay = f.read_u32().unwrap();
        f.offset_inc(header.data_size);
        Ok(Self {
            header,
            rate,
            initial_delay,
        })
    }

    pub fn header_ref(&self) -> &Header {
        &self.header
    }

    pub fn header(&self) -> Header {
        self.header.clone()
    }

    pub fn rate(&self) -> u32 {
        self.rate
    }

    pub fn initial_delay(&self) -> u32 {
        self.initial_delay
    }
}
