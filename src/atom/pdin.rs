use super::{Header, Mp4File};

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

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn rate(&self) -> u32 {
        self.rate
    }

    pub fn initial_delay(&self) -> u32 {
        self.initial_delay
    }
}
