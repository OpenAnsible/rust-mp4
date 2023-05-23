//!

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{generic_parse, retref};

#[derive(Debug, Clone)]
pub struct Stss {
    pub header: Header,
}

impl Stss {
    generic_parse!(Stss);
    retref!(header, Header);
}
