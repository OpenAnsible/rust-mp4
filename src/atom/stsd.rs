//! Sample Description Box - Gives detailed information about the coding type used
//! and any initialization information needed for that coding.
//!
//! The Sample Description Box shall contain at least one entry.
//!
//! A Sample Description Box is required because it contains the
//! data reference index field which indicates which Data Reference Box
//! to use to retrieve the media samples.
//! Without the Sample Description, it is not possible to determine where the media
//! samples are stored.

use std::mem::size_of;

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref, retval};

/// Gives detailed information about the coding type used and any initialization information needed for that coding.
///
/// The information stored in the sample description box after the entry‐count is both track‐type specific
/// as documented here, and can also have variants within a track type (e.g. different codings may use
/// different specific information after some common fields, even within a video track).
///
/// Which type of sample entry form is used is determined by the media handler, using a suitable form,
/// such as one defined in clause 12, or defined in a derived specification, or registration.
/// Multiple descriptions may be used within a track.
///
/// _Note: Though the count is 32 bits, the number of items is usually much fewer, and is restricted by the fact
/// that the reference index in the sample table is only 16 bits._
///
/// If the ‘format’ field of a SampleEntry is unrecognized, neither the sample description itself, nor the
/// associated media samples, shall be decoded.
///
/// _Note: The definition of sample entries specifies boxes in a particular order, and this is usually also followed in
/// derived specifications. For maximum compatibility, writers should construct files respecting the order both within
/// specifications and as implied by the inheritance, whereas readers should be prepared to accept any box order._
///
/// All string fields shall be null‐terminated, even if unused. “Optional” means there is at least one null byte.
///
/// Entries that identify the format by MIME type, such as a TextSubtitleSampleEntry,
/// TextMetaDataSampleEntry, or SimpleTextSampleEntry, all of which contain a MIME type, may be used
/// to identify the format of streams for which a MIME type applies. A MIME type applies if the contents of
/// the string in the optional configuration box (without its null termination), followed by the contents of a
/// set of samples, starting with a sync sample and ending at the sample immediately preceding a sync
/// sample, are concatenated in their entirety, and the result meets the decoding requirements for
/// documents of that MIME type. Non‐sync samples should be used only if that format specifies the
/// behaviour of ‘progressive decoding’, and then the sample times indicate when the results of such
/// progressive decoding should be presented (according to the media type).
///
/// _Note: The samples in a track that is all sync samples are therefore each a valid document for that MIME
/// type._
///
/// In some classes derived from SampleEntry, namespace and schema_location are used both to identify
/// the XML document content and to declare “brand” or profile compatibility. Multiple namespace
/// identifiers indicate that the track conforms to the specification represented by each of the identifiers,
/// some of which may identify supersets of the features present. A decoder should be able to decode all the
/// namespaces in order to be able to decode and present correctly the media associated with this sample
/// entry.
///
/// _Note: Additionally, namespace identifiers may represent performance constraints, such as limits on
/// document size, font size, drawing rate, etc., as well as syntax constraints such as features that are not
/// permitted or ignored._
#[derive(Debug, Clone)]
pub struct Stsd {
    pub header: Header,

    /// The number of entries in the following table.
    pub entry_count: u32,

    /// A list of Sample Entries
    pub entries: Vec<SampleEntry>,

    /// Additional data we don't know what to do with at this point
    pub extra_data: String,
}

impl Stsd {
    /// parses the Sample Description Box from the given `Mp4File`.
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);
        let curr_offset = f.offset();

        log::trace!("Stsd::parse() -- header = {header:?}");

        let_ok!(entry_count, f.read_u32(), "Unable to read entry count.");
        log::trace!("Stsd::parse() -- entry_count = {entry_count}");

        let mut entries = Vec::with_capacity(entry_count as usize);

        for _ in 0..entry_count {
            let_ok!(entry, SampleEntry::parse(f), "Unable to read sample entry.");
            entries.push(entry);
        }

        let num_entries = entries.len();
        log::trace!("Stsd::parse() -- num_entries = {num_entries}, entries = {entries:?}");

        // If for some reason we haven't read all the data, skip the rest.
        let bytes_read = (size_of::<SampleEntry>() * entry_count as usize) + size_of::<u32>();
        log::trace!(
            "Stsd::parse() -- bytes_read = {bytes_read}, header.data_size = {}",
            header.data_size()
        );

        // FIXME: This is a hack to get around the fact that we don't know how to parse the extra data.
        // let mut extra_data = Vec::new();
        let mut extra_data = String::new();
        if bytes_read < header.data_size() as usize {
            log::trace!(
                "Stsd::parse() -- reading {} bytes",
                header.data_size() - bytes_read as u64
            );
            for _ in bytes_read..header.data_size() as usize {
                let_ok!(byte, f.read_u8(), "Unable to read extra data byte.");
                extra_data.push(byte as char);
            }

            log::trace!("Stsd::parse() -- extra_data = {extra_data:?}");
        }

        let_ok!(
            _offset,
            f.seek(curr_offset + header.data_size()),
            "Unable to seek file."
        );

        // Advance the file offset by the size of the data.
        let _offset = f.offset_inc(header.data_size);

        Ok(Self {
            header,
            entry_count,
            entries,
            extra_data,
        })
    }

    retref!(header, Header);
    retval!(entry_count, u32);
    retref!(entries, Vec<SampleEntry>);
    retref!(extra_data, String);
}

/// A Sample Entry is a table that describes the format of one or more samples in a track.
#[derive(Debug, Clone)]
pub struct SampleEntry {
    /// The format. If the ‘format’ field of a SampleEntry is unrecognized, neither the sample description itself,
    /// nor the associated media samples, shall be decoded.
    pub format: String,

    /// The brand. Definitely unsure about this.
    pub brand: String,

    /// Reserved. Should be set to 0.
    pub reserved: [u8; 6],

    /// A table that describes the format of the sample data.
    pub data_reference_index: u16,
}

impl SampleEntry {
    /// Parses a Sample Entry from the given `Mp4File` and returns it for use in a `Stsd` atom.
    pub fn parse(f: &mut Mp4File) -> Result<Self, &'static str> {
        let_ok!(format, f.read_4_char_string(), "Unable to read format.");
        let_ok!(unknown, f.read_4_char_string(), "Unable to read unknown.");

        let mut reserved = [0u8; 6];
        for n in 0..6 {
            let_ok!(byte, f.read_u8(), "Unable to read reserved byte.");
            reserved[n] = byte;
        }

        let_ok!(
            data_reference_index,
            f.read_u16(),
            "Unable to read data reference index."
        );

        Ok(Self {
            format,
            brand: unknown,
            reserved,
            data_reference_index,
        })
    }

    retref!(format, String);
    retref!(brand, String);
    retref!(reserved, [u8; 6]);
    retval!(data_reference_index, u16);
}
