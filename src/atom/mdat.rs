//! The Mdat atom contains the actual media data, i.e., the audio and video frames.
//!
//! -- ISO/IEC 14496-12:2015 § 8.1.1
//!
//! This is atom is required to be at the end of the file (unless this object is after all the media data).

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref};

/// This box contains the media data. In video tracks, this box would contain video frames.
///
/// A presentation may contain zero or more Media Data Boxes.
/// The actual media data follows the type field; its structure is described by the metadata
/// (see particularly the sample table, subclause 8.5, and the item location box, subclause 8.11.3).
///
/// In large presentations, it may be desirable to have more data in this box than a 32-bit size would permit.
/// In this case, the large variant of the size field, above in subclause 4.2, is used.
///
/// There may be any number of these boxes in the file (including zero,
/// if all the media data is in other files).
/// The metadata refers to media data by its absolute offset within the file
/// (see subclause 8.7.5, the Chunk Offset Box); so Media Data Box headers
/// and free space may easily be skipped, and files without any box structure
/// may also be referenced and used.
///
/// - Box Type: `mdat`
/// - Container: File
/// - Mandatory: No
/// - Quantity: Zero or more
///
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Mdat {
    /// Header of the `Mdat` atom.
    pub header: Header,

    /// The actual media data. This is a `Vec` of bytes, and is not parsed. This is the raw data, so
    /// the user is responsible for parsing it into something useful. Right now it is _not_ read,
    /// as this is too time and resource-consuming. But the code is in the parse function if needed.
    pub data: Vec<u8>,
}

impl Mdat {
    /// Parse an atom from the file. The actual data will be read into the `data` field.
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let curr_offset = f.offset();

        // This snippet can read the data into a `Vec` of bytes, but it's not useful unless we
        // can find a way to do it much faster. There has to be some kind of a crate or something
        // that can do it much faster than this, especially considering that we know how large the
        // data is.
        // let mut data = Vec::with_capacity(header.data_size as usize);
        // for _ in 0..header.data_size {
        //     let byte_res = f.read_u8();

        //     if byte_res.is_err() {
        //         log::error!("Mdat::parse() -- Unable to read byte from file.");
        //         return Err("Unable to read Mdat data from file.");
        //     }

        //     let byte = byte_res.unwrap_or_default();
        //     data.push(byte);
        // }

        // If you decide to read the data, you should comment out this statement.
        let_ok!(
            _offset,
            f.seek(curr_offset + header.data_size),
            "Unable to seek file."
        );

        f.offset_inc(header.data_size);
        log::trace!("$id::parse() -- header = {header:?}");

        Ok(Self {
            header,
            data: Vec::new(),
        })
    }

    retref!(header, Header);
    retref!(data, Vec<u8>);
}
