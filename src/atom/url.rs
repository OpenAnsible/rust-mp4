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

        // let curr_offset = f.offset();

        let location = if header.data_size() != 0 {
            let_ok!(
                loc,
                f.read_null_terminated_string(),
                "Unable to read location."
            );
            loc
        } else {
            String::new()
        };

        // Advance the file offset by the size of the data.
        let _offset = f.offset_inc(header.data_size);

        Ok(Self { header, location })
    }

    retref!(header, Header);
    retref!(location, String);
}
