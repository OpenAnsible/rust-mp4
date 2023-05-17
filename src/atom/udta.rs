//! User Data Box (`Udta`)
//!
//! - Box Type:  `Udta`
//! - Container: Movie Box (‘moov’), Track Box (‘trak’), Movie Fragment Box ([Moof](crate::atom::moof::Moof)) or Track Fragment Box ([Traf](crate::atom::traf::Traf))
//! - Mandatory: No
//! - Quantity:  Zero or one
//!
//! This  box  contains  objects  that  declare  user  information  about  t he  containing  box  and  its  data (presentation or track).
//!
//! The User Data Box is a container box for informative user‐data. This user data is formatted as a set of
//! boxes with more specific box types, which declare more precisely their content.
//!
//! The handling of user‐data in movie fragments is described under [Assp](crate::atom::assp::Assp).

use crate::atom::atom::Atom;
use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{generic_parse_children, retref};

/// Contains objects that declar use information about the containing box and its data (presentation or track).
#[derive(Debug, Clone)]
pub struct Udta {
    /// The header of the atom.
    header: Header,

    /// A list of atoms contained in this atom. The following atoms may be found within the `Udta` atom:
    ///
    /// - [Cprt](crate::atom::cprt::Cprt)
    /// - [Tsel](crate::atom::tsel::Tsel)
    /// - [Strk](crate::atom::strk::Strk)
    children: Vec<Atom>,
}

impl Udta {
    generic_parse_children!(Udta);
    retref!(header, Header);
    retref!(children, Vec<Atom>);
}
