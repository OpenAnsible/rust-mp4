//! File Partition Atom

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, read_version, retref, retval};

/// The File Partition box identifies the source file and provides a partitioning of that file into source blocks
/// and symbols. Further information about the source file, e.g., filename, content location and group IDs, is
/// contained in the Item Information box ([Iinf](crate::atom::iinf::Iinf)), where the Item Information entry corresponding to
/// the item ID of the source file is of version 1 and includes a File Delivery Item Information Extension
/// ([Fdel](crate::atom::fdel::Fdel)). Version 1 of `FilePartitionBox` should only be used when support for large `item_ID` or
/// `entry_count` values (exceeding 65535) is required or expected to be required.
#[derive(Debug, Clone)]
pub struct Fpar {
    /// The header of the atom.
    pub header: Header,

    /// The item in the item location box ([Iloc](crate::atom::iloc::Iloc)) that the file partitioning applies to.
    pub item_id: u32,

    /// Gives the target ALC/LCT or FLUTE packet payload size of the partitioning algorithm.
    /// Note that UDP packet payloads are larger, as they also contain ALC/LCT or FLUTE headers.
    pub packet_payload_size: u16,

    /// Reserved for future use. Set to 0.
    pub reserved: u8,

    /// Identifies the FEC encoding scheme and is subject to IANA registration (see [RFC 5052](https://www.rfc-editor.org/rfc/rfc5052.html)).
    /// Note that:
    ///
    /// 1. value zero corresponds to the "Compact No‐Code FEC scheme" also known as "Null‐FEC" ([RFC 3695](https://www.rfc-editor.org/rfc/rfc3695.html));
    /// 2. value one corresponds to the “MBMS FEC” ([3GPP TS 26.346](https://portal.3gpp.org/desktopmodules/Specifications/SpecificationDetails.aspx?specificationId=1452));
    /// 3. for values in the range of 0 to 127, inclusive, the FEC scheme is Fully‐Specified,
    /// whereas for values in the range of 128 to 255, inclusive, the FEC scheme is Under‐Specified.
    pub fec_encoding_id: u8,

    /// provides a more specific identification of the FEC encoder being used for an
    /// Under‐Specified FEC scheme. This value should be set to zero for Fully‐Specified FEC schemes
    /// and shall be ignored when parsing a file with FEC_encoding_ID in the range of 0 to 127,
    /// inclusive. FEC_instance_ID is scoped by the FEC_encoding_ID. See [RFC 5052](https://www.rfc-editor.org/rfc/rfc5052.html) for further
    /// details.
    pub fec_instance_id: u16,

    /// The maximum number of source symbols per source block.
    pub max_source_block_length: u16,

    /// The size (in bytes) of one encoding symbol. All encoding symbols of one item have the same length,
    /// except the last symbol which may be shorter.
    pub encoding_symbol_length: u16,

    /// The maximum number of encoding symbols that can be generated for a source block for
    /// those FEC schemes in which the maximum number of encoding symbols is relevant,
    /// such as FEC encoding ID 129 defined in [RFC 5052](https://www.rfc-editor.org/rfc/rfc5052.html).
    /// For those FEC schemes in which the maximum number of encoding symbols is not relevant,
    /// the semantics of this field is unspecified.
    pub max_number_of_encoding_symbols: u16,

    /// A base64‐encoded string of the scheme‐specific object transfer information (FEC‐OTI‐Scheme‐Specific‐Info).
    /// The definition of the information depends on the FEC encoding ID.
    pub scheme_specific_info: String,

    /// The number of entries in the list of (block_count, block_size) pairs that
    /// provides a partitioning of the source file. Starting from the beginning of the file, each entry
    /// indicates how the next segment of the file is divided into source blocks and source symbols.
    pub entry_count: u32,

    /// The list of (block_count, block_size) pairs that provides a partitioning of the source file.
    pub entries: Vec<FparEntry>,
}

impl Fpar {
    /// Parses a `Fpar` atom from the given file. The header is already parsed and passed in.
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        read_version!(item_id, u32, f.read_u16(), f.read_u32(), header);

        let_ok!(
            packet_payload_size,
            f.read_u16(),
            "Unable to read packet payload size."
        );

        let_ok!(reserved, f.read_u8(), "Unable to read reserved.");
        if reserved != 0 {
            log::warn!("Reserved value is not 0. Either the file is corrupt, or there is a bug in the parser.");
            return Err("Reserved value is not 0");
        }

        let_ok!(
            fec_encoding_id,
            f.read_u8(),
            "Unable to read FEC encoding id."
        );
        let_ok!(
            fec_instance_id,
            f.read_u16(),
            "Unable to read FEC instance id."
        );
        let_ok!(
            max_source_block_length,
            f.read_u16(),
            "Unable to read max source block length."
        );
        let_ok!(
            encoding_symbol_length,
            f.read_u16(),
            "Unable to read encoding symbol length."
        );
        let_ok!(
            max_number_of_encoding_symbols,
            f.read_u16(),
            "Unable to read max number of encoding symbols."
        );
        let_ok!(
            scheme_specific_info,
            f.read_null_terminated_string(),
            "Unable to read scheme specific info."
        );

        read_version!(entry_count, u32, f.read_u16(), f.read_u32(), header);

        let mut entries: Vec<FparEntry> = Vec::with_capacity(entry_count as usize);
        for _n in 0..entry_count {
            let entry = FparEntry::parse(f)?;
            entries.push(entry);
        }

        // Advance the file offset by the size of the data.
        let _offset = f.offset_inc(header.data_size);

        Ok(Self {
            header,
            item_id,
            packet_payload_size,
            reserved,
            fec_encoding_id,
            fec_instance_id,
            max_source_block_length,
            encoding_symbol_length,
            max_number_of_encoding_symbols,
            scheme_specific_info,
            entry_count,
            entries,
        })
    }

    retref!(header, Header);
    retval!(item_id, u32);
    retval!(packet_payload_size, u16);
    retval!(reserved, u8);
    retval!(fec_encoding_id, u8);
    retval!(fec_instance_id, u16);
    retval!(max_source_block_length, u16);
    retval!(encoding_symbol_length, u16);
    retval!(max_number_of_encoding_symbols, u16);
    retref!(scheme_specific_info, String);
    retval!(entry_count, u32);
    retref!(entries, Vec<FparEntry>);
}

/// Defines the File Partition Entry
#[derive(Debug, Clone)]
pub struct FparEntry {
    /// The number of consecutive source blocks of size `block_size`.
    block_count: u16,

    /// the size of a block (in bytes). A `block_size` that is not a multiple of the
    /// `encoding_symbol_length` symbol size indicates with Compact No‐Code FEC that the last source
    /// symbols includes padding that is not stored in the item. With MBMS FEC (3GPP TS 26.346) the
    /// padding may extend across multiple symbols but the size of padding should never be more than
    /// `encoding_symbol_length`.
    block_size: u32,
}

impl FparEntry {
    pub fn parse(f: &mut Mp4File) -> Result<Self, &'static str> {
        let_ok!(block_count, f.read_u16(), "Unable to read block count.");
        let_ok!(block_size, f.read_u32(), "Unable to read block size.");

        Ok(Self {
            block_count,
            block_size,
        })
    }

    retval!(block_count, u16);
    retval!(block_size, u32);
}
