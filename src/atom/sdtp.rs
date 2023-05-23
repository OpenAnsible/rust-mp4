//! `sdtp` atom tree

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{generic_parse, retref};

#[derive(Debug, Clone)]
pub struct Sdtp {
    pub header: Header,
}

impl Sdtp {
    generic_parse!(Sdtp);
    retref!(header, Header);
}
