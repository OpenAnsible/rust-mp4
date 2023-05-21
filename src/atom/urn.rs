//! The Uniform Resource Name (`Urn`) atom.

//! The Urn atom is a data entry atom that contains a Uniform Resource Name (URN) as defined in [RFC 2141](https://tools.ietf.org/html/rfc2141).
//! The atom is defined in ISO/IEC 14496-12:2015 §8.7.2

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref};

/// Contains a URN entry
#[derive(Debug, Clone)]
pub struct Urn {
    /// The header of the atom.
    header: Header,

    /// The name of the URN.
    name: String,

    /// The location of the media data.
    location: String,
}

impl Urn {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let curr_offset = f.offset();

        let_ok!(
            name,
            f.read_null_terminated_string(),
            "Unable to read name."
        );
        let_ok!(
            location,
            f.read_null_terminated_string(),
            "Unable to read location."
        );

        let _seek_res = f.seek(curr_offset + header.data_size);
        let _offset = f.offset_inc(header.data_size);

        Ok(Self {
            header,
            name,
            location,
        })
    }

    retref!(header, Header);
    retref!(name, String);
    retref!(location, String);
}
