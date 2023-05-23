//!

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{generic_parse, retref};

#[derive(Debug, Clone)]
pub struct Tref {
    pub header: Header,
}

impl Tref {
    generic_parse!(Tref);
    retref!(header, Header);
}
