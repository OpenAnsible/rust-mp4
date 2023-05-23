//! File Delivery Item Information Extension

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref, retval};

/// File Delivery Item Information Extension (`Fdel`) atom.
///
/// The `Fdel` atom is an extension to the `Ftyp` atom.
/// It contains information about the delivery of the file.
///
/// - Box Type: `Fdel`
/// - Container: File Type Box (`Ftyp`)
/// - Mandatory: No
/// - Quantity: Zero or one
#[derive(Debug, Clone)]
pub struct Fdel {
    /// The header of the atom.
    pub header: Header,

    /// A null‐terminated string in UTF‐8 characters containing the URI of the file
    /// as defined in HTTP/1.1 ([RFC 2616](https://www.rfc-editor.org/rfc/rfc2616.html)).
    pub content_location: String,

    /// A null‐terminated string in UTF‐8 characters containing an MD5 digest of the file.
    /// See HTTP/1.1 [RFC 2616](https://www.rfc-editor.org/rfc/rfc2616.html) and [RFC 1864](https://www.rfc-editor.org/rfc/rfc1864.html).
    pub content_md5: String,

    /// The total length (in bytes) of the (un‐encoded) file.
    pub content_length: u64,

    /// The total length (in bytes) of the (encoded) file. Note that transfer length
    /// is equal to content length if no content encoding is applied (see above).
    pub transfer_length: u64,

    /// A count of the number of entries in the following array.
    pub entry_count: u8,

    /// A file group to which the file item (source file) belongs.
    /// See 3GPP TS 26.346 for more details on file groups.
    pub group_ids: Vec<u32>,
}

impl Fdel {
    /// Parses an `Fdel` atom from the given `Mp4File`.
    ///
    /// # Arguments
    ///
    /// * `f` - The `Mp4File` to read the `Fdel` atom from.
    /// * `header` - The `Header` of the `Fdel` atom.
    ///
    /// # Returns
    ///
    /// * An `Fdel` atom if parsing was successful, otherwise an error.
    ///
    /// # Errors
    ///
    /// * If there was an error reading from the `Mp4File`.
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let_ok!(
            content_location,
            f.read_null_terminated_string(),
            "Unable to read content location."
        );
        let_ok!(
            content_md5,
            f.read_null_terminated_string(),
            "Unable to read content MD5."
        );

        let_ok!(
            content_length,
            f.read_u64(),
            "Unable to read content length."
        );
        let_ok!(
            transfer_length,
            f.read_u64(),
            "Unable to read transfer length."
        );

        let_ok!(entry_count, f.read_u8(), "Unable to read entry count.");

        let mut group_ids = Vec::with_capacity(entry_count as usize);
        for _ in 0..entry_count {
            let_ok!(group_id, f.read_u32(), "Unable to read group ID.");
            group_ids.push(group_id);
        }

        // Advance the file offset by the size of the data.
        f.offset_inc(header.data_size);

        Ok(Self {
            header,
            content_location,
            content_md5,
            content_length,
            transfer_length,
            entry_count,
            group_ids,
        })
    }

    // Included for completeness.
    retref!(header, Header);
    retref!(content_location, String);
    retref!(content_md5, String);
    retval!(content_length, u64);
    retval!(transfer_length, u64);
    retval!(entry_count, u8);
    retref!(group_ids, Vec<u32>);
}
