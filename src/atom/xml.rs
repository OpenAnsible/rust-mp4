//! XML (`Xml`) stores XML data. Ref. [Bxml](crate::atom::bxml).

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref};

#[derive(Debug, Clone)]
pub struct Xml {
    header: Header,
    xml: String,
}

impl Xml {
    /// Parse an `Xml` atom from the file. This will read the data from the file and store it in a
    /// `String`, which is a UTF-8 string. It is up to the user to interpret the data from there.
    ///
    /// # Arguments
    ///
    /// * `f` - The file to read from.
    /// * `header` - The header of the atom.
    ///
    /// # Returns
    ///
    /// - `Result<Self, &'static str>` - The parsed atom with the data stored in a `String`.
    ///
    /// # Errors
    ///
    /// - `Err` - If the file cannot be seeked.
    /// - `Err` - If the data cannot be read from the file.
    /// - `Err` - If the data cannot be parsed from utf8.
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let mut xml_bytes: Vec<u8> = Vec::new();
        for _ in 0..header.data_size {
            let_ok!(byte, f.read_u8(), "Unable to read byte.");
            xml_bytes.push(byte);
        }

        let Ok(xml) = String::from_utf8(xml_bytes) else {
            return Err("Unable to parse XML from utf8")
        };

        log::trace!("Xml::parse() -- header = {header:?}, xml = {xml}");

        f.offset_inc(header.data_size);
        Ok(Self { header, xml })
    }

    retref!(header, Header);
    retref!(xml, String);
}
