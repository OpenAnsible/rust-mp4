//!

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::retref;

#[derive(Debug, Clone)]
pub struct Nmhd {
    pub header: Header,
}

impl Nmhd {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Self {
        header.parse_version(f);
        header.parse_flags(f);

        Self { header }
    }

    retref!(header, Header);
}
