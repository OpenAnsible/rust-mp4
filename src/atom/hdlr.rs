//! Handler Reference Atom

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref};

/// This box within a Media Box declares media type of the track, and thus the process by which the media‐
/// data in the track is presented. For example, a format for which the decoder delivers video would be
/// stored in a video track, identified by being handled by a video handler. The documentation of the
/// storage of a media format identifies the media type which that format uses.
///
/// This box when present within a Meta Box, declares the structure or format of the ’meta’ box contents.
///
/// There is a general handler for metadata streams of any type; the specific format is identified by the
/// sample entry, as for video or audio, for example.
///
/// Box Type:  ‘hdlr’
/// Container: Media Box (‘mdia’) or Meta Box (‘meta’)
/// Mandatory: Yes
/// Quantity:  Exactly one
#[derive(Debug, Clone)]
pub struct Hdlr {
    /// The header of the atom.
    pub header: Header,

    /// - When present in a media box, contains a value as defined in clause 12, or a value from a derived
    /// specification, or registration.
    /// - When present in a meta box, contains an appropriate value to indicate the format of the meta
    /// box contents. The value ‘null’ can be used in the primary meta box to indicate that it is
    /// merely being used to hold resources.
    pub handler_type: String,

    /// A null‐terminated string in UTF‐8 characters which gives a human‐readable name for the
    /// track type (for debugging and inspection purposes).
    pub name: String,
}

impl Hdlr {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        // Not used - so we just throw it away. Should always be 0.
        let _pre_defined = f.read_u32().unwrap_or(0);

        // Read the handler type.
        let mut handler_type = String::new();
        for _ in 0..4 {
            let_ok!(b, f.read_u8(), "Unable to read handler type byte.");
            handler_type.push(b as char);
        }

        // Reserved - not used - so we just throw it away. Should always be 0.
        for _ in 0..3 {
            let _reserved = f.read_u32().unwrap_or(0);
        }

        let_ok!(name, f.read_string(), "Unable to read name.");

        // Move the file offset by the size of the data.
        f.offset_inc(header.data_size);

        // Return safely
        Ok(Self {
            header,
            handler_type,
            name,
        })
    }

    // Included for completeness.
    retref!(header, Header);
    retref!(handler_type, String);
    retref!(name, String);
}
