use crate::let_ok;

use super::{Header, Mp4File};

#[derive(Debug, Clone)]
pub struct Pdin {
    header: Header,
    rate: u32,
    initial_delay: u32,
}

impl Pdin {
    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str> {
        let_ok!(rate, f.read_u32(), "Unable to read rate.");
        let_ok!(initial_delay, f.read_u32(), "Unable to read initial delay.");

        f.offset_inc(header.data_size);

        Ok(Self {
            header,
            rate,
            initial_delay,
        })
    }

    pub const fn header(&self) -> &Header {
        &self.header
    }

    pub const fn rate(&self) -> u32 {
        self.rate
    }

    pub const fn initial_delay(&self) -> u32 {
        self.initial_delay
    }
}
