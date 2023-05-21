//! Contains the `Skip` and `Free` atoms, which are used to skip over data that is not needed.
//!
//! You can find the spec for these atoms in ISO/IEC 14496-12:2015 § 8.1.2.
//! These atoms are not recognized by this library, so they are just parsed and discarded.
//!
//! From the specification:
//!
//! _The contents of a free-space box are irrelevant and may be ignored, or the object deleted, without affecting the presentation.
//! (Care should be exercised when deleting the object, as this may invalidate the offsets used in the sample table,
//! unless this object is after all the media data)._
//!
//! -- ISO/IEC 14496-12:2015 § 8.1.2

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{generic_parse, retref};

/// `Skip` is a free space atom that is used to skip over data that is not needed.
/// It is identical to the `Free` atom for all intents and purposes.
#[derive(Debug, Clone)]
pub struct Skip {
    /// The header of the atom.
    pub header: Header,
}

impl Skip {
    generic_parse!(Skip);
    retref!(header, Header);
}

/// `Free` is a free space atom that is used to skip over data that is not needed.
/// It is identical to the `Skip` atom for all intents and purposes.
#[derive(Debug, Clone)]
pub struct Free {
    /// The header of the atom.
    pub header: Header,
}

impl Free {
    generic_parse!(Free);
    retref!(header, Header);
}
