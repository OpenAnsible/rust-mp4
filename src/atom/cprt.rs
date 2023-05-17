//! Copyright Box (`Cprt`)
//!
//! - Box Type:  ‘cprt’
//! - Container: User data box (‘udta’)
//! - Mandatory: No
//! - Quantity:  Zero or more
//!
//! The Copyright box contains a copyright declaration which applies to the entire presentation, when
//! contained within the Movie Box, or, when contained in a track, to that entire track. There may be
//! multiple copyright boxes using different language codes.

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref};
use bitreader::BitReader;

#[derive(Debug, Clone)]
pub struct Cprt {
    /// The header of the atom.
    header: Header,

    /// Declares the language code for the following text. See ISO 639‐2/T for the set of three
    /// character codes. Each character is packed as the difference between its ASCII value and 0x60.
    /// The code is confined to being three lower‐case letters, so these values are strictly positive.
    language: String,

    /// A null‐terminated string in either UTF‐8 or UTF‐16 characters, giving a copyright notice.
    /// If UTF‐16 is used, the string shall start with the BYTE ORDER MARK (`0xFEFF`), to distinguish it
    /// from a UTF‐8 string. This mark does not form part of the final string. Note that this field is
    /// the raw text, so you may need to do any necessary decoding to get the actual text.
    notice: String,
}

impl Cprt {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let_ok!(bytes, f.read_u16(), "Unable to read language");
        let lang = u16::to_be_bytes(bytes);

        let mut br = BitReader::new(&lang);
        let _pad = br.read_u8(1).unwrap_or_default();
        let l1 = br.read_u8(5).unwrap_or_default();
        let l2 = br.read_u8(5).unwrap_or_default();
        let l3 = br.read_u8(5).unwrap_or_default();

        let language = format!(
            "{}{}{}",
            (l1 + 0x60) as char,
            (l2 + 0x60) as char,
            (l3 + 0x60) as char
        );

        let mut notice = String::new();
        let mut byte = f.read_u8().unwrap_or_default();
        while byte != 0 {
            notice.push(byte as char);
            byte = f.read_u8().unwrap_or_default();
        }

        f.offset_inc(header.data_size);

        Ok(Self {
            header,
            language,
            notice,
        })
    }

    // These are here for completeness, since the struct is public.
    retref!(header, Header);
    retref!(language, String);
    retref!(notice, String);
}
