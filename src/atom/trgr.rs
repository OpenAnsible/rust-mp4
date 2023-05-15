//!

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{generic_parse, retref};

#[derive(Debug, Clone)]
pub struct Trgr {
    pub header: Header,
}

impl Trgr {
    generic_parse!(Trgr);
    retref!(header, Header);
}
