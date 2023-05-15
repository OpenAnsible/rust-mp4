//! Metabox Relation (`Mere`) atom.

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref, retval};

/// The metabox relation box indicates a relation between two meta boxes at the same level,
/// i.e., the top level of the file, the Movie Box, or Track Box.
///
/// The relation between two meta boxes is unspecified if there is no metabox relation
/// box for those meta boxes. Meta boxes are referenced by specifying their handler types.
///
/// - Box Type: `Mere`
/// - Container: Additional Metadata Container Atom (`Meco`)
/// - Mandatory: No
/// - Quantity: Zero or more
///
/// `first_metabox_handler_type` indicates the first meta box to be related.
///
/// `second_metabox_handler_type` indicates the second meta box to be related.
///
/// `metabox_relation` indicates the relation between the two meta boxes.
///
/// The following values are defined:
///
/// |Value|Description|
/// |:---:|-----------|
/// | `1` | The relationship between the boxes is unknown (which is the default when this box is not present)|
/// | `2` | The two boxes are semantically un-related (e.g., one is presentation, the other annotation)|
/// | `3` | The two boxes are semantically related but complementary (e.g., two disjoint sets of meta-data expressed in two different meta-data systems)|
/// | `4` | The two boxes are semantically related but overlap (e.g., two sets of meta-data neither of which is a subset of the other); neither is ‘preferred’ to the other|
/// | `5` | The two boxes are semantically related but the second is a proper subset or weaker version of the first; the first is preferred|
/// | `6` | The two boxes are semantically related and equivalent (e.g., two essentially identical sets of meta-data expressed in two different meta-data systems)|
#[derive(Debug, Clone)]
pub struct Mere {
    /// Header of the `Mere` atom.
    header: Header,

    /// The first meta box to be related.
    first_metabox_handler_type: u32,

    /// The second meta box to be related.
    second_metabox_handler_type: u32,

    /// The relation between the two meta boxes.
    metabox_relation: u8,

    /// The relation between the two meta boxes as an enum. This makes it easier to work with, and less error-prone.
    metabox_relation_enum: MetaboxRelation,
}

impl Mere {
    /// Parses the `Mere` atom, returning `Self`.
    ///
    /// # Arguments
    ///
    /// * `f` - `Mp4File` to read from.
    /// * `header` - `Header` of the `Mere` atom.
    ///
    /// # Returns
    ///
    /// * `Self` - The parsed `Mere` atom.
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let_ok!(
            first_metabox_handler_type,
            f.read_u32(),
            "Unable to determine first metabox handler type."
        );

        let_ok!(
            second_metabox_handler_type,
            f.read_u32(),
            "Unable to determine second metabox handler type."
        );

        let_ok!(
            metabox_relation,
            f.read_u8(),
            "Unable to determine metabox relation."
        );

        let _offset = f.offset_inc(header.data_size);
        let metabox_relation_enum: MetaboxRelation = MetaboxRelation::from(metabox_relation);

        log::trace!("Mere::parse() -- header = {header:?}, first_metabox_handler_type = {first_metabox_handler_type}, second_metabox_handler_type = {second_metabox_handler_type}, metabox_relation = {metabox_relation}, metabox_relation_enum = {}", metabox_relation_enum);

        Ok(Self {
            header,
            first_metabox_handler_type,
            second_metabox_handler_type,
            metabox_relation,
            metabox_relation_enum,
        })
    }

    retref!(header, Header);
    retval!(first_metabox_handler_type, u32);
    retval!(second_metabox_handler_type, u32);
    retval!(metabox_relation, u8);
    retref!(metabox_relation_enum, MetaboxRelation);
}

/// Indicates the relationship between two meta boxes in the `Mere` atom.
///
/// Using this enum makes the `Mere` atom easier to work with, and less error-prone
/// (not to mention more readable, maintainable, and extensible).
/// This is also more idiomatic Rust. The values are taken from the table in the spec.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MetaboxRelation {
    /// The relationship between the boxes is unknown (which is the default when this box is not present).
    Unknown = 1,

    /// The two boxes are semantically un-related (e.g., one is presentation, the other annotation).
    Unrelated = 2,

    /// The two boxes are semantically related but complementary (e.g., two disjoint sets of meta-data expressed in two different meta-data systems).
    Complementary = 3,

    /// The two boxes are semantically related but overlap (e.g., two sets of meta-data neither of which is a subset of the other); neither is ‘preferred’ to the other.
    Overlap = 4,

    /// The two boxes are semantically related but the second is a proper subset or weaker version of the first; the first is preferred.
    Subset = 5,

    /// The two boxes are semantically related and equivalent (e.g., two essentially identical sets of meta-data expressed in two different meta-data systems).
    Equivalent = 6,
}

impl From<u8> for MetaboxRelation {
    /// Converts a `u8` to a `MetaboxRelation`.
    fn from(val: u8) -> Self {
        match val {
            2 => Self::Unrelated,
            3 => Self::Complementary,
            4 => Self::Overlap,
            5 => Self::Subset,
            6 => Self::Equivalent,
            _ => Self::Unknown, // Also covers 1, which is the default when this box is not present.
        }
    }
}

impl std::fmt::Display for MetaboxRelation {
    /// Converts a `MetaboxRelation` to a `String` for printing
    /// (e.g., `MetaboxRelation::Unknown` becomes `"Unknown"`).
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Unknown => "Unknown",
            Self::Unrelated => "Unrelated",
            Self::Complementary => "Complementary",
            Self::Overlap => "Overlap",
            Self::Subset => "Subset",
            Self::Equivalent => "Equivalent",
        };
        write!(f, "{s}")
    }
}
