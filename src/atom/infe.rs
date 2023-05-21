//! Information Entry

use crate::atom::atom::Atom;
use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref, retval};

/// Represent an information entry.
#[derive(Debug, Clone)]
pub struct Infe {
    /// The header of the atom.
    pub header: Header,

    /// The item ID. u16 in version 0 & 1, u32 in version 2 and later.
    pub item_id: u32,

    /// The item protection index. Found in versions 2 and later.
    pub item_protection_index: u16,

    /// The item type as a 4-char string. Found in versions 2 and later.
    pub item_type: String,

    /// The item name. Found in versions 0 and 1.
    pub item_name: String,

    /// The content type. Found in versions 0 and 1.
    pub content_type: String,

    /// The content encoding. Found in versions 0 and 1.
    pub content_encoding: String,

    /// The extension type as a 4-char string. Found in version 1.
    pub extension_type: String,

    /// Item Info Extension
    pub item_info_extension: Option<Box<Atom>>,

    /// Item URI Type
    pub item_uri_type: String,
}

impl Infe {
    /// Parses an `Infe` atom from the given `Mp4File`.
    ///
    /// # Arguments
    ///
    /// * `f` - The `Mp4File` to read the `Infe` atom from.
    /// * `header` - The `Header` of the `Infe` atom.
    ///
    /// # Returns
    ///
    /// * An `Infe` atom if parsing was successful, otherwise an error.
    ///
    /// # Errors
    ///
    /// * If there was an error reading from the `Mp4File`.
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let version = header.version().unwrap_or(0);

        // Initialize the fields to their default values.
        let mut item_id = 0;
        let mut item_protection_index = 0;
        let mut item_name = String::new();
        let mut content_type = String::new();
        let mut content_encoding = String::new();
        let mut extension_type = String::new();
        let mut item_type = String::new();
        let mut item_info_extension = None;
        let mut item_uri_type = String::new();

        if version == 0 || version == 1 {
            let_ok!(iid, f.read_u16(), "Unable to read item ID.");
            item_id = iid as u32;

            let_ok!(ipi, f.read_u16(), "Unable to read item protection index.");
            item_protection_index = ipi;

            let_ok!(
                iname,
                f.read_null_terminated_string(),
                "Unable to read item name."
            );
            item_name = iname;

            let_ok!(
                ct,
                f.read_null_terminated_string(),
                "Unable to read content type."
            );
            content_type = ct;

            let_ok!(
                ce,
                f.read_null_terminated_string(),
                "Unable to read content encoding."
            );
            content_encoding = ce;
        }

        if version == 1 {
            let_ok!(et, f.read_4_char_string(), "Unable to read extension type.");
            extension_type = et;

            log::trace!("Infe::parse() -- extension_type = {extension_type}");

            let iie = Box::new(Atom::parse(f)?);
            item_info_extension = Some(iie);
        }

        if version >= 2 {
            item_id = if version == 2 {
                let_ok!(item_id, f.read_u16(), "Unable to read item ID.");
                item_id as u32
            } else {
                let_ok!(item_id, f.read_u32(), "Unable to read item ID.");
                item_id
            };

            let_ok!(ipe, f.read_u16(), "Unable to read item protection index.");
            item_protection_index = ipe;

            let_ok!(it, f.read_4_char_string(), "Unable to read item type.");
            item_type = it;

            let_ok!(
                itn,
                f.read_null_terminated_string(),
                "Unable to read item name."
            );
            item_name = itn;

            if item_type == String::from("mime") {
                let_ok!(
                    ct,
                    f.read_null_terminated_string(),
                    "Unable to read content type."
                );
                content_type = ct;

                let_ok!(
                    ce,
                    f.read_null_terminated_string(),
                    "Unable to read content encoding."
                );
                content_encoding = ce;
            } else if item_type == String::from("uri ") {
                let_ok!(
                    iut,
                    f.read_null_terminated_string(),
                    "Unable to read item URI type."
                );
                item_uri_type = iut;
            }
        }

        // Advance the file offset by the size of the data.
        f.offset_inc(header.data_size);

        Ok(Self {
            header,
            item_id,
            item_protection_index,
            item_type,
            item_name,
            content_type,
            content_encoding,
            extension_type,
            item_info_extension,
            item_uri_type,
        })
    }

    // These are here for completeness, since the struct is public.
    retref!(header, Header);
    retval!(item_id, u32);
    retval!(item_protection_index, u16);
    retref!(item_type, String);
    retref!(item_name, String);
    retref!(content_type, String);
    retref!(content_encoding, String);
    retref!(extension_type, String);
    retref!(item_info_extension, Option<Box<Atom>>);
    retref!(item_uri_type, String);
}
