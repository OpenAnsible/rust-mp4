//! Scheme Type

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref, retval};

/// Identifies the protection or restriction scheme.
///
/// - Box Types: ‘schm’
/// - Container:  Protection Scheme Information Box (‘sinf’), Restricted Scheme Information Box (‘rinf’), or SRTP Process box (‘srpp‘)
/// - Mandatory: No
/// - Quantity:  Zero or one in ‘sinf’, depending on the protection structure; Exactly one in ‘rinf’ and ‘srpp’
#[derive(Debug, Clone)]
pub struct Schm {
    /// The header of the atom.
    pub header: Header,

    /// A four-character code that identifies the protection or restriction scheme.
    pub scheme_type: String,

    /// A 32-bit integer that specifies the version of the scheme (used to create the contents of the scheme-specific information).
    pub scheme_version: u32,

    /// Allows for the option of directing the user to a web‐page if they do not have the
    /// scheme installed on their system. It is an absolute URI formed as a null‐terminated string in
    /// UTF‐8 characters.
    pub scheme_info: String,
}

impl Schm {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let_ok!(
            scheme_type,
            f.read_4_char_string(),
            "Unable to read scheme type."
        );

        let_ok!(
            scheme_version,
            f.read_u32(),
            "Unable to read scheme version."
        );

        let scheme_info = if header.flags_to_u32() & 0x000001 == 0x000001 {
            let_ok!(
                si,
                f.read_null_terminated_string(),
                "Unable to read scheme info."
            );
            si
        } else {
            String::new()
        };

        Ok(Self {
            header,
            scheme_type,
            scheme_version,
            scheme_info,
        })
    }

    retref!(header, Header);
    retref!(scheme_type, String);
    retval!(scheme_version, u32);
    retref!(scheme_info, String);
}
