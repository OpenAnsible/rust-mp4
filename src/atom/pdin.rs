use super::{Header, Mp4File};
use crate::{let_ok, retref, retval};

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

    retref!(header, Header);
    retval!(rate, u32);
    retval!(initial_delay, u32);
}
