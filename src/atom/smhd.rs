//!

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref, retval};

#[derive(Debug, Clone)]
pub struct Smhd {
    pub header: Header,
    pub balance: f64, // fixed-point 8.8 number
}

impl Smhd {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let_ok!(balance, f.read_fixed_point(8, 8), "Unable to read balance.");

        // reserved
        let _throwaway = f.read_u16().unwrap_or_default();

        f.offset_inc(4);

        Ok(Self { header, balance })
    }

    retref!(header, Header);
    retval!(balance, f64);
}
