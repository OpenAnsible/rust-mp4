//! Contains the 'Url' atom.

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref};

/// Contains a URL entry
///
/// Box Type:  ‘url ‘
/// Container: Data Information Box ([Dref](crate::atom::dref::Dref)])
/// Mandatory: Yes (at least one of ‘url ‘ or ‘urn ‘ shall be present)
/// Quantity: One or more
#[derive(Debug, Clone)]
pub struct Url {
    /// The header of the atom.
    header: Header,

    /// The location of the media data.
    location: String,
}

impl Url {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let curr_offset = f.offset();

        let_ok!(
            location,
            f.read_null_terminated_string(),
            "Unable to read location."
        );

        let _seek_res = f.seek(curr_offset + header.data_size);
        let _offset = f.offset_inc(header.data_size);

        Ok(Self { header, location })
    }

    retref!(header, Header);
    retref!(location, String);
}
