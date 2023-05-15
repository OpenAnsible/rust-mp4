//!

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{generic_parse, retref};

#[derive(Debug, Clone)]
pub struct Stsd {
    pub header: Header,
}

impl Stsd {
    generic_parse!(Stsd);
    retref!(header, Header);
}
