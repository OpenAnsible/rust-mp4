//! The Mdat atom contains the actual media data, i.e., the audio and video frames.
//!
//! -- ISO/IEC 14496-12:2015 § 8.1.1
//!
//! This is atom is required to be at the end of the file (unless this object is after all the media data).

use super::{Header, Mp4File};
use crate::{let_ok, retref};

/// This box contains the media data. In video tracks, this box would contain video frames.
///
/// A presentation may contain zero or more Media Data Boxes.
/// The actual media data follows the type field; its structure is described by the metadata
/// (see particularly the sample table, subclause 8.5, and the item location box, subclause 8.11.3).
///
/// In large presentations, it may be desirable to have more data in this box than a 32-bit size would permit.
/// In this case, the large variant of the size field, above in subclause 4.2, is used.
///
/// There may be any number of these boxes in the file (including zero,
/// if all the media data is in other files).
/// The metadata refers to media data by its absolute offset within the file
/// (see subclause 8.7.5, the Chunk Offset Box); so Media Data Box headers
/// and free space may easily be skipped, and files without any box structure
/// may also be referenced and used.
///
/// - Box Type: `mdat`
/// - Container: File
/// - Mandatory: No
/// - Quantity: Zero or more
///
#[derive(Debug, Clone)]
pub struct Mdat {
    /// Header of the `Mdat` atom.
    header: Header,
}

impl Mdat {
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

        log::trace!("Mdat::parse() -- header = {header:?}");

        Ok(Self { header })
    }

    retref!(header, Header);
}
