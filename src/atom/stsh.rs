//!

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{generic_parse, retref};

#[derive(Debug, Clone)]
pub struct Stsh {
    pub header: Header,
}

impl Stsh {
    generic_parse!(Stsh);
    retref!(header, Header);
}
