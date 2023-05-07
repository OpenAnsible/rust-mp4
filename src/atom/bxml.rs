//! Contains the `Bxml` atom, which is used to store binary XML data.

use super::{Header, Mp4File};
use crate::{let_ok, retref};

/// `Bxml` is used to store binary XML data. This is used in the `Meta` atom.
///
/// This box is defined in ISO/IEC 14496-12:2012 § 8.11.2.
///
/// When the primary data is in XML format and it is desired that the XML be stored directly in the meta-box,
/// one of these forms may be used. The Binary XML Box may only be used when there is a single
/// well-defined binarization of the XML for that defined format as identified by the handler.
///
/// Within an XML box the data is in UTF-8 format unless the data starts with
/// a byte-order-mark (BOM), which indicates that the data is in UTF-16 format.
///
/// The data is stored as a `Vec<u8>`, so it is up to the user to parse and interpret the data.
///
/// - Box Type: `Bxml`
/// - Container:`Meta`
/// - Mandatory: No
/// - Quantity: Zero or one
#[derive(Debug, Clone)]
pub struct Bxml {
    /// Header of the `Bxml` atom.
    header: Header,

    /// The binary XML data.
    data: Vec<u8>,
}

impl Bxml {
    /// Parse a `Bxml` atom from the file. This will read the data from the file and store it in a
    /// `Vec<u8>`. It is up to the user to interpret the data from there.
    ///
    /// # Arguments
    ///
    /// * `f` - The file to read from.
    /// * `header` - The header of the atom.
    ///
    /// # Returns
    ///
    /// * `Result<Self, &'static str>` - The parsed atom with the data stored in a `Vec<u8>`.
    ///
    /// # Errors
    ///
    /// * `Err` - If the file cannot be seeked.
    /// * `Err` - If the data cannot be read from the file.
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let mut data: Vec<u8> = Vec::new();
        for _ in 0..header.data_size {
            let_ok!(byte, f.read_u8(), "Unable to read byte.");
            data.push(byte);
        }

        f.offset_inc(header.data_size);

        log::trace!("Bxml::parse() -- header = {header:?}, data = {data:?}");

        Ok(Self { header, data })
    }

    retref!(header, Header);
    retref!(data, Vec<u8>);
}
