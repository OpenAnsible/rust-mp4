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

use super::{Header, Mp4File};
use crate::retref;

/// Creates a generic function for parsing a 'free space' atom (i.e., `Skip` or `Free`).
/// This is a macro because it is used by both `Skip` and `Free` in exactly the same way.
///
/// # Arguments
///
/// * `$id` - The name of the struct that is being parsed. This is only used in the log message.
macro_rules! parse {
    ($id:ident) => {
        /// Parse an atom from the file. This will skip over the data in the file.
        ///
        /// # Arguments
        ///
        /// * `f` - The file to read from.
        /// * `header` - The header of the atom.
        ///
        /// # Returns
        ///
        /// * `Self` - The parsed atom, which in this case basically means we move the offset ahead.
        pub fn parse(f: &mut Mp4File, header: Header) -> Self {
            let curr_offset = f.offset();
            let _throwaway = f.seek(curr_offset + header.data_size);
            let _offset = f.offset_inc(header.data_size);
            log::trace!("$id::parse() -- header = {header:?}");
            Self { header }
        }
    };
}

/// `Skip` is a free space atom that is used to skip over data that is not needed.
/// It is identical to the `Free` atom for all intents and purposes.
#[derive(Debug, Clone)]
pub struct Skip {
    /// The header of the atom.
    header: Header,
}

impl Skip {
    parse!(Skip);
    retref!(header, Header);
}

/// `Free` is a free space atom that is used to skip over data that is not needed.
/// It is identical to the `Skip` atom for all intents and purposes.
#[derive(Debug, Clone)]
pub struct Free {
    /// The header of the atom.
    header: Header,
}

impl Free {
    parse!(Free);
    retref!(header, Header);
}
