//! Item Information

use crate::atom::atom::Atom;
use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref, retval};

/// Provides extra information about selected items, including symbolic (`file`) names.
///
/// It may optionally occur, but if it does, it must be interpreted, as item protection or content
/// encoding may have changed the format of the data in the item. If both content encoding and protection
/// are indicated for an item, a reader should first un‐protect the item, and then decode the item’s content
/// encoding. If more control is needed, an IPMP sequence code may be used.
///
/// This box contains an array of entries, and each entry is formatted as a box. This array is sorted by
/// increasing item_ID in the entry records.
///
/// Four versions of the item info entry are defined. Version 1 includes additional information to version 0
/// as specified by an extension type. For instance, it shall be used with extension type `fdel` for items
/// that are referenced by the file partition box ([Fpar](crate::atom::fpar::Fpar)), which is defined for source file partitionings and
/// applies to file delivery transmissions. Versions 2 and 3 provide an alternative structure in which
/// metadata item types are indicated by a 32-bit (typically 4-character) registered or defined code; two of
/// these codes are defined to indicate a MIME type or metadata typed by a URI. Version 2 supports 16-bit
/// item_ID values, whereas version 3 supports 32-bit item_ID values.
///
/// If  no  extension  is  desired,  the  box  may  terminate  without  the  extension_type  field  and  the
/// extension; if, in addition, content_encoding is not desired, that field also may be absent and the box
/// terminate before it. If an extension is desired without an explicit content_encoding, a single null
/// byte, signifying the empty string, must be supplied for the content_encoding, before the indication
/// of extension_type.
///
/// If file delivery item information is needed and a version 2 or 3 ItemInfoEntry is used, then the file
/// delivery information is stored as a separate item of type [Fdel](crate::atom::fdel::Fdel) that is also linked by an item reference
/// from the item, to the file delivery information, of type [Fdel](crate::atom::fdel::Fdel). There must be exactly one such reference if
/// file delivery information is needed.
///
/// It is possible that there are valid URI forms for MPEG-7 metadata (e.g. a schema URI with a fragment
/// identifying a particular element), and it may be possible that these structures could be used for MPEG‐7.
/// However, there is explicit support for MPEG‐=07 in ISO base media file format family files, and this explicit
/// support is preferred as it allows, among other things:
///
/// a. incremental update of the metadata (logically, I/P coding, in video terms) whereas this draft is I-frame only;
/// b. binarization and thus compaction;
/// c. the use of multiple schemas.
///
/// Therefore, the use of these structures for MPEG-7 is deprecated (and undocumented).
///
/// Information on URI forms for some metadata systems can be found in Annex G.
/// Version  1  of  ItemInfoBox  should  only  be  used  when  support  for  a  large  number  of
/// itemInfoEntries (exceeding 65535) is required or expected to be required.
///
/// - Box Type:  ‘iinf’
/// - Container: Meta Box ([Meta](crate::atom::meta::Meta))
/// - Mandatory: No
/// - Quantity:  Zero or one
#[derive(Debug, Clone)]
pub struct Iinf {
    /// The header of the atom.
    pub header: Header,

    /// The number of entries in the following table
    pub entry_count: u32,

    /// An array of item information entries.
    pub item_infos: Vec<Atom>,
}

impl Iinf {
    /// Parses an `Iinf` atom from the given `Mp4File`.
    ///
    /// # Arguments
    ///
    /// * `f` - The `Mp4File` to read the `Iinf` atom from.
    /// * `header` - The `Header` of the `Iinf` atom.
    ///
    /// # Returns
    ///
    /// * An `Iinf` atom if parsing was successful, otherwise an error.
    ///
    /// # Errors
    ///
    /// * If there was an error reading from the `Mp4File`.
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let entry_count = if header.version().unwrap_or(0) == 0 {
            let_ok!(ec, f.read_u16(), "Unable to read entry count.");
            ec as u32
        } else {
            let_ok!(ec, f.read_u32(), "Unable to read entry count.");
            ec
        };

        let mut item_infos = Vec::new();
        for _ in 0..entry_count {
            let item_info = Atom::parse(f)?;
            item_infos.push(item_info);
        }

        // Advance the file offset by the size of the data.
        f.offset_inc(header.data_size);

        Ok(Self {
            header,
            entry_count,
            item_infos,
        })
    }

    // These are here for completeness, since the struct is public.
    retref!(header, Header);
    retval!(entry_count, u32);
    retref!(item_infos, Vec<Atom>);
}
