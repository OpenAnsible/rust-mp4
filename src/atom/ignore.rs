//! `Ignore` is a free space atom that is used to skip over data that is not needed.
//!
//! This is identical to the `Skip` and `Free` atoms for all intents and purposes.
//! The `Free` and `Skip` atoms are defined in the spec (ISO/IEC 14496-12:2015 § 8.1.2),
//! but the `Ignore` atom is not.
use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{generic_parse, retref};

/// Ignore is a free space atom that is used to skip over data that is not needed, similar to `Free` and `Skip`.
/// The `Free` and `Skip` atoms are defined in the spec (ISO/IEC 14496-12:2015 § 8.1.2), but the `Ignore` atom is not.
#[derive(Debug, Clone)]
pub struct Ignore {
    /// Header of the `Ignore` atom.
    pub header: Header,
}

impl Ignore {
    generic_parse!(Ignore);
    retref!(header, Header);
}
