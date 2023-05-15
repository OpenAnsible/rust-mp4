//!

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{generic_parse, retref};

#[derive(Debug, Clone)]
pub struct Stdp {
    pub header: Header,
}

impl Stdp {
    generic_parse!(Stdp);
    retref!(header, Header);
}
