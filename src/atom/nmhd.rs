//!

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{generic_parse, retref};

#[derive(Debug, Clone)]
pub struct Nmhd {
    pub header: Header,
}

impl Nmhd {
    generic_parse!(Nmhd);
    retref!(header, Header);
}
