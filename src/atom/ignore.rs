//! `Ignore` is a free space atom that is used to skip over data that is not needed.
//!
//! This is identical to the `Skip` and `Free` atoms for all intents and purposes.
//! The `Free` and `Skip` atoms are defined in the spec (ISO/IEC 14496-12:2015 § 8.1.2),
//! but the `Ignore` atom is not.
use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref};

/// Ignore is a free space atom that is used to skip over data that is not needed, similar to `Free` and `Skip`.
/// The `Free` and `Skip` atoms are defined in the spec (ISO/IEC 14496-12:2015 § 8.1.2), but the `Ignore` atom is not.
#[derive(Debug, Clone)]
pub struct Ignore {
    /// Header of the `Ignore` atom.
    header: Header,
}

impl Ignore {
    /// Parse a Ignore atom from the file. This will skip over the data in the file.
    ///
    /// # Arguments
    ///
    /// * `f` - The file to read from.
    /// * `header` - The header of the atom.
    ///
    /// # Returns
    ///
    /// * `Self` - The parsed atom, which in this case basically means we move the offset ahead.
    ///
    /// # Errors
    ///
    /// * `Unable to seek file.` - If the file cannot be seeked.
    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str> {
        let curr_offset = f.offset();

        let_ok!(
            _offset,
            f.seek(curr_offset + header.data_size),
            "Unable to seek file."
        );

        f.offset_inc(header.data_size);
        log::trace!("Ignore::parse() -- header = {header:?}");

        Ok(Self { header })
    }

    // Get the header of the atom.
    retref!(header, Header);
}
