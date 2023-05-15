//! Contains descriptive or annotative metadata.

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::retref;

/// `Meta` contains descriptive or annotative metadata. It is used to provide information about
/// the media, or to associate other objects with the media.
///
/// The `Meta` box is required to contain a `Hdlr` box indicating the structure or format of the `Meta` box contents.
/// That metadata is located either within a box within this box (e.g. an `Xml` box), or is located by the item
/// identified by a primary item (`Pitm`) box.
///
/// All other contained boxes are specific to the format specified by the handler box.
///
/// The other boxes defined here may be defined as optional or mandatory for a given format.
/// If they are used, then they must take the form specified here.
/// These optional boxes include a data-information box, which documents other files in which metadata values
/// (e.g. pictures) are placed, and a item location box, which documents where in those files each item
/// is located (e.g. in the common case of multiple pictures stored in the same file).
/// At most one meta box may occur at each of the file level, movie level, or track level,
/// unless they are contained in an additional metadata container box (‘meco’).
///
/// If an Item Protection Box occurs, then some or all of the meta-data,
/// including possibly the primary resource,
/// may have been protected and be un-readable unless the protection system is taken into account.
///
/// - Box Type: `Meta`
/// - Container: File, Movie Box (`Moov`), Track Box (`Trak`), or Additional Metadata Container Box (`Meco`)
/// - Mandatory: No
/// - Quantity: Zero or one (in File, `Moov`, and `Trak`), One or more (in `Meco`)
///
/// This Atom is currently not implemented beyond parsing the header.
/// TODO: Implement the rest of this atom.
#[derive(Debug, Clone)]
pub struct Meta {
    /// Header of the `Meta` atom.
    header: Header,
}

impl Meta {
    /// Parses the `Meta` atom, returning `Self`. This function is currently not implemented beyond parsing the header.
    ///
    /// # Arguments
    ///
    /// * `f` - `Mp4File` to read from.
    /// * `header` - `Header` of the `Meta` atom.
    ///
    /// # Returns
    ///
    /// * `Self` - The parsed `Meta` atom.
    ///
    /// # Errors
    ///
    /// * `Err` - If the file cannot be seeked.
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let curr_offset = f.offset();
        let Ok(_offset) = f.seek(curr_offset + header.data_size) else {
            return Err("Unable to seek file.")
        };
        f.offset_inc(header.data_size);

        log::trace!("Meta::parse() -- header = {header:?}");

        Ok(Self { header })
    }

    retref!(header, Header);
}
