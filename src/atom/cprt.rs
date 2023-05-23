//! Copyright Box (`Cprt`)

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref};

/// The Copyright box contains a copyright declaration which applies to the entire presentation, when
/// contained within the Movie Box, or, when contained in a track, to that entire track. There may be
/// multiple copyright boxes using different language codes.
///
/// - Box Type:  `Cprt`
/// - Container: User data box ([Udta](crate::atom::udta::Udta))
/// - Mandatory: No
/// - Quantity:  Zero or more
#[derive(Debug, Clone)]
pub struct Cprt {
    /// The header of the atom.
    pub header: Header,

    /// Declares the language code for the following text. See ISO 639‐2/T for the set of three
    /// character codes. Each character is packed as the difference between its ASCII value and 0x60.
    /// The code is confined to being three lower‐case letters, so these values are strictly positive.
    pub language: String,

    /// A null‐terminated string in either UTF‐8 or UTF‐16 characters, giving a copyright notice.
    /// If UTF‐16 is used, the string shall start with the BYTE ORDER MARK (`0xFEFF`), to distinguish it
    /// from a UTF‐8 string. This mark does not form part of the final string. Note that this field is
    /// the raw text, so you may need to do any necessary decoding to get the actual text.
    pub notice: String,
}

impl Cprt {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        // 2 Bytes
        // pad: 1 Bit
        // language: 15 Bit;
        let_ok!(
            language,
            f.read_iso639_code(),
            "Unable to read language from ISO639 code."
        );

        let notice = f.read_null_terminated_string().unwrap_or_default();

        // Advance the file offset by the size of the data.
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
