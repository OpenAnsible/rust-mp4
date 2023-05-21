//! Item Location

use std::fmt::Formatter;

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::read_extent;
use crate::{let_ok, retref, retval};

/// provides a directory of resources in this or other files, by locating their container,
/// their offset within that container, and their length.
///
/// Placing this in binary format enables common handling of this data,
/// even by systems which do not understand the particular metadata system (handler) used.
/// For example, a system might integrate all the externally referenced metadata resources
/// into one place, re‐adjusting offsets and references accordingly.
///
/// The box starts with three or four values, specifying the size in bytes of the offset field, length
/// field, base_offset field, and, in versions 1 and 2 of this box, the extent_index fields, respectively.
/// These values must be from the set `{0, 4, 8}`.
///
/// The construction_method field indicates the ‘construction method’ for the item:
///
/// 1. `file_offset`: by the usual absolute file offsets into the file at `data_reference_index`;
/// (`construction_method == 0` (`File`))
/// 2. `idat_offset`: by box offsets into the idat box in the same meta box; neither the
/// `data_reference_index` nor extent_index fields are used; (`construction_method == 1` (`Idat`))
/// 3. `item_offset`: by item offset into the items indicated by the `extent_index` field, which is only
/// used (currently) by this construction method. (`construction_method == 2` (`Item`)).
///
/// The `extent_index` is only used for the method `item_offset`; it indicates the 1‐based index of the item
/// reference with referenceType ‘iloc’ linked from this item. If `index_size` is 0, then the value 1 is implied;
/// the value 0 is reserved.
///
/// Items may be stored fragmented into extents, e.g. to enable interleaving. An extent is a contiguous
/// subset of the bytes of the resource; the resource is formed by concatenating the extents. If only one
/// extent is used (extent_count = 1) then either or both of the offset and length may be implied:
///
/// - If the offset is not identified (the field has a length of zero), then the beginning of the source
/// (offset 0) is implied.
/// - If the length is not specified, or specified as zero, then the entire length of the source is implied.
/// References into the same file as this metadata, or items divided into more than one extent,
/// should have an explicit offset and length, or use a MIME type requiring a different interpretation
/// of the file, to avoid infinite recursion.
///
/// The size of the item is the sum of the extent lengths.
///
/// _NOTE: Extents may be interleaved with the chunks defined by the sample tables of tracks._
///
/// The offsets are relative to a data origin. That origin is determined as follows:
///
/// 1. When the [Meta](crate::atom::meta::Meta) box is in a Movie Fragment, and the `construction_method` specifies a file offset,
/// and the data reference indicates ‘same file’, the data origin is the first byte of the enclosing
/// Movie Fragment Box (as for the `default‐base‐is‐moof` flag in the Track Fragment Header);
/// 2. In all other cases when the `construction_method` specifies a file offset, the data origin is the
/// beginning of the file identified by the data reference;
/// 3. When the `construction_method` specifies offsets into the Item Data box, the data origin is the
/// beginning of `data[]` in the Item Data box;
/// 4. When the data reference specifies another item, the data origin is the first byte of the
/// concatenated data (of all the extents) of that item;
///
/// _NOTE: There are offset calculations in other parts of this file format based on the beginning of a box header; in
/// contrast, item data offsets are calculated relative to the box contents._
///
/// The data‐reference index may take the value 0, indicating a reference into the same file as this
/// metadata, or an index into the data‐reference table.
///
/// Some referenced data may itself use offset/length techniques to address resources within it (e.g. an
/// MP4 file might be ‘included’ in this way). Normally such offsets in the item itself are relative to the
/// beginning  of  the  containing  file.  The  field  ‘base  offset’  provides  an  additional  offset  for  offset
/// calculations within that contained data. For example, if an MP4 file is included within a file formatted to
/// this specification, then normally data‐offsets within that MP4 section are relative to the beginning of
/// file; the base offset adds to those offsets.
///
/// If an item is constructed from other items, and those source items are protected, the offset and length
/// information apply to the source items after they have been de‐protected. That is, the target item data is
/// formed from unprotected source data.
///
/// For maximum compatibility, version 0 of this box should be used in preference to version 1 with
/// `construction_method==0`, or version 2 when possible. Similarly, version 2 of this box should only
/// be used when support for large item_ID values (exceeding 65535) is required or expected to be
/// required.
#[derive(Debug, Clone)]
pub struct Iloc {
    /// The header of the atom.
    pub header: Header,

    /// Indicates the length in bytes of the offset field. Valid values are 0, 4, and 8.
    pub offset_size: u8,

    /// Indicates the length in bytes of the length field. Valid values are 0, 4, and 8.
    pub length_size: u8,

    /// Indicates the length in bytes of the base offset field. Valid values are 0, 4, and 8.
    pub base_offset_size: u8,

    /// Indicates the length in bytes of the index field. Valid values are 0, 4, and 8.
    pub index_size: u8,

    /// Reserved. Should be 0.
    pub reserved: u8,

    /// Indicates the number of items in the following table.
    pub item_count: u32,

    /// The table of items.
    pub items: Vec<IlocItem>,
}

impl Iloc {
    /// Parses the `Iloc` atom, returning `Self`.
    ///
    /// # Arguments
    ///
    /// * `f` - `Mp4File` to read from.
    /// * `header` - `Header` of the `Iloc` atom.
    ///
    /// # Returns
    ///
    /// * `Result<Self, &'static str>` - The parsed `Iloc` atom or an error message.
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        let offset_size;
        let length_size;
        let base_offset_size;
        let mut index_size = 0;
        let reserved = 0;
        let item_count;

        header.parse_version(f);
        header.parse_flags(f);

        let version = header.version().unwrap_or(0);

        let_ok!(
            offset_length,
            f.read_u8(),
            "Unable to read offset_size and length_size."
        );
        offset_size = offset_length >> 4;
        length_size = offset_length & 0x0F;

        let_ok!(
            bosisr,
            f.read_u8(),
            "Unable to read base_offset_size, index_size, and reserved."
        );
        base_offset_size = bosisr >> 4;

        if version == 1 || version == 2 {
            index_size = bosisr & 0x0F;
        }

        if version < 2 {
            let_ok!(ic, f.read_u16(), "Unable to read item count.");
            item_count = ic as u32;
        } else if version == 2 {
            let_ok!(ic, f.read_u32(), "Unable to read item count.");
            item_count = ic;
        } else {
            return Err("Invalid version.");
        }

        let mut items = Vec::with_capacity(item_count as usize);
        for _ in 0..item_count {
            let item = IlocItem::parse(
                f,
                version,
                base_offset_size,
                index_size,
                offset_size,
                length_size,
            )?;
            items.push(item);
        }

        // Advance the file offset by the size of the data.
        f.offset_inc(header.data_size);

        Ok(Self {
            header,
            offset_size,
            length_size,
            base_offset_size,
            index_size,
            reserved,
            item_count,
            items,
        })
    }

    retref!(header, Header);
    retval!(offset_size, u8);
    retval!(length_size, u8);
    retval!(base_offset_size, u8);
    retval!(index_size, u8);
    retval!(reserved, u8);
    retval!(item_count, u32);
    retref!(items, Vec<IlocItem>);
}

#[derive(Debug, Clone)]
pub struct IlocItem {
    /// An arbitrary integer ‘name’ for this resource which can be used to refer to it (e.g. in a URL).
    pub item_id: u32,

    /// Reserved. Should be 0.
    pub reserved: u16,

    /// Indicates the method by which the data referenced by this item is stored.
    /// Valid values are 0 (`file`), 1 (`idat`), and 2 (`item`).
    pub construction_method: u8,

    /// Indicates the method by which the data referenced by this item is stored. This is just the previous field turned into an enum for convenience.
    pub construction_method_enum: ConstructionMethod,

    /// Either zero (‘this file’) or a 1‐based index into the data references in the data information box.
    pub data_reference_index: u16,

    /// provides a base value for offset calculations within the referenced data. If
    /// `base_offset_size` is 0, `base_offset` takes the value 0, i.e. it is unused.
    pub base_offset: u64,

    /// Provides the count of the number of extents into which the resource is fragmented;
    /// it must have the value 1 or greater
    pub extent_count: u16,

    /// The table of extents.
    pub extents: Vec<IlocExtent>,
}

impl IlocItem {
    pub fn parse(
        f: &mut Mp4File,
        version: u8,
        base_offset_size: u8,
        index_size: u8,
        offset_size: u8,
        length_size: u8,
    ) -> Result<Self, &'static str> {
        let item_id;
        let mut reserved = 0;
        let mut construction_method = 0;
        let base_offset;

        if version < 2 {
            let_ok!(id, f.read_u16(), "Unable to read item id.");
            item_id = id as u32;
        } else if version == 2 {
            let_ok!(id, f.read_u32(), "Unable to read item id.");
            item_id = id;
        } else {
            return Err("Invalid version.");
        }

        if version == 1 || version == 2 {
            let_ok!(r, f.read_u16(), "Unable to read reserved.");
            reserved = r >> 4;
            construction_method = (r & 0x000F) as u8;
        }

        let construction_method_enum = ConstructionMethod::from_u8(construction_method);

        if base_offset_size == 4 {
            let_ok!(bo, f.read_u32(), "Unable to read base offset.");
            base_offset = bo as u64;
        } else if base_offset_size == 8 {
            let_ok!(bo, f.read_u64(), "Unable to read base offset.");
            base_offset = bo;
        } else {
            base_offset = 0;
        }

        let_ok!(
            data_reference_index,
            f.read_u16(),
            "Unable to read data reference index."
        );
        let_ok!(extent_count, f.read_u16(), "Unable to read extent count.");

        let mut extents = Vec::with_capacity(extent_count as usize);
        for _ in 0..extent_count {
            let extent = IlocExtent::parse(f, version, index_size, offset_size, length_size)?;
            extents.push(extent);
        }

        Ok(Self {
            item_id,
            reserved,
            construction_method,
            construction_method_enum,
            data_reference_index,
            base_offset,
            extent_count,
            extents,
        })
    }

    retval!(item_id, u32);
    retval!(reserved, u16);
    retval!(construction_method, u8);
    retval!(data_reference_index, u16);
    retval!(base_offset, u64);
    retval!(extent_count, u16);
    retref!(extents, Vec<IlocExtent>);
}

/// The individual extents of an `IlocItem`.
#[derive(Debug, Clone)]
pub struct IlocExtent {
    /// Provides an index as defined for the construction method
    extent_index: u64,

    /// Provides the absolute offset, in bytes from the data origin of the container, of this
    /// extent data. If `offset_size` is 0, `extent_offset` takes the value 0
    extent_offset: u64,

    /// Provides the absolute length in bytes of this metadata item extent.
    /// If `length_size` is 0, `extent_length` takes the value 0.
    /// If the value is 0, then length of the extent is the length of the entire referenced container.
    extent_length: u64,
}

impl IlocExtent {
    /// Parses an `IlocExtent` from the given file.
    ///
    /// # Arguments
    ///
    /// * `f` - The file to read from.
    /// * `version` - The version of the `Iloc` atom.
    /// * `is` - The size of the extent index. Should be 0, 4, or 8.
    /// * `os` - The size of the extent offset. Should be 0, 4, or 8.
    /// * `ls` - The size of the extent length. Should be 0, 4, or 8.
    ///
    /// # Returns
    ///
    /// * `Result<Self, &'static str>` - The parsed `IlocExtent`.
    ///
    /// # Errors
    ///
    /// * `Unable to read extent index.` - If the extent index cannot be read from the file.
    /// * `Invalid index size.` - If the extent index size is not 0, 4, or 8.
    ///
    /// * `Unable to read extent offset.` - If the extent offset cannot be read from the file.
    /// * `Invalid offset size.` - If the extent offset size is not 0, 4, or 8.
    ///
    /// * `Unable to read extent length.` - If the extent length cannot be read from the file.
    /// * `Invalid length size.` - If the extent length size is not 0, 4, or 8.
    fn parse(
        f: &mut Mp4File,
        version: u8,
        index_size: u8,
        offset_size: u8,
        length_size: u8,
    ) -> Result<Self, &'static str> {
        let extent_index;
        let extent_offset;
        let extent_length;

        // Read the extent index if it exists.
        if version == 1 || version == 2 && index_size > 0 {
            read_extent!(
                index_size,
                extent_index,
                f.read_u32(),
                f.read_u64(),
                "Unable to read extent index.",
                "Invalid index size."
            )
        } else {
            extent_index = 0;
        }

        // Read the extent offset if it exists.
        read_extent!(
            offset_size,
            extent_offset,
            f.read_u32(),
            f.read_u64(),
            "Unable to read extent offset.",
            "Invalid offset size."
        );

        // Read the extent length if it exists.
        read_extent!(
            length_size,
            extent_length,
            f.read_u32(),
            f.read_u64(),
            "Unable to read extent length.",
            "Invalid length size."
        );

        Ok(Self {
            extent_index,
            extent_offset,
            extent_length,
        })
    }

    retval!(extent_index, u64);
    retval!(extent_offset, u64);
    retval!(extent_length, u64);
}

#[macro_export]
macro_rules! read_extent {
    ($sz:ident, $var:ident, $s:expr, $l:expr, $msg:literal, $err:literal) => {
        if $sz == 4 {
            let_ok!(v, $s, $msg);
            $var = v as u64;
        } else if $sz == 8 {
            let_ok!(v, $l, $msg);
            $var = v;
        } else {
            $var = 0;
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConstructionMethod {
    File = 0,
    Idat = 1,
    Item = 2,
}

impl ConstructionMethod {
    pub fn from_u8(val: u8) -> Self {
        match val {
            0 => ConstructionMethod::File,
            1 => ConstructionMethod::Idat,
            2 => ConstructionMethod::Item,
            _ => panic!("Invalid construction method."),
        }
    }
}

impl std::fmt::Display for ConstructionMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ConstructionMethod::File => write!(f, "file"),
            ConstructionMethod::Idat => write!(f, "idat"),
            ConstructionMethod::Item => write!(f, "item"),
        }
    }
}
