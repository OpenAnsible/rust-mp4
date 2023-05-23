//! The `ftyp` atom is the first atom in the file. It contains the file type and version information.
//!
//! This atom is mandatory, and there must be exactly one of them in the file. It must be placed as early as possible in the file.
//! It identifies which specification is the ‘best use’ of the file, and a minor version of that specification;
//! and also a set of other specifications to which the file complies.
//!
//! The Atom is defined in ISO/IEC 14496-12 § 4.3.2.

#![allow(clippy::doc_markdown)]

use std::str;
use std::str::FromStr;
use std::string::ToString;

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref, retval};

/// The FileType enum represents the file type of the MP4 file. This is the
/// first 4 bytes of the file.
///
/// The following table shows the file types and the corresponding file extensions, mime types, and codecs:
///
/// | File Type | Category | File Extension | MIME Type      | Codec  |
/// | --------- | -------- |--------------- |  ------------- | -------|
/// | `AVC1`      | ISO      | `.mp4`           | video/mp4      | H.264  |
/// | `ISO2`      | ISO      | `.mp4`           | video/mp4      | H.264  |
/// | `ISOM`      | ISO      | `.mp4`           | video/mp4      | H.264  |
/// | `MP21`      | ISO      | `.mp4`           | video/mp4      | H.264  |
/// | `MP41`      | ISO      | `.mp4`           | video/mp4      | H.264  |
/// | `MP42`      | ISO      | `.mp4`           | video/mp4      | H.264  |
/// | `QT`        | Apple    | `.mov`           | video/quicktime| H.264  |
/// | `M4B`       | Apple    | `.m4b`           | audio/mp4      | AAC    |
/// | `M4P`       | Apple    | `.m4p`           | audio/mp4      | AAC    |
/// | `M4A`       | Apple    | `.m4a`           | audio/mp4      | AAC    |
/// | `M4V`       | Apple    | `.m4v`           | video/mp4      | H.264  |
/// | `M4VH`      | Apple    | `.m4v`           | video/mp4      | H.264  |
/// | `M4VP`      | Apple    | `.m4v`           | video/mp4      | H.264  |
/// | `F4V`       | Adobe    | `.f4v`           | video/mp4      | H.264  |
/// | `F4P`       | Adobe    | `.f4p`           | video/mp4      | H.264  |
/// | `F4A`       | Adobe    | `.f4a`           | video/mp4      | AAC    |
/// | `F4B`       | Adobe    | `.f4b`           | video/mp4      | AAC    |
/// | `MMP4`      | 3GPP/GSM | `.3gp`           | video/3gpp     | H.264  |
///
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    // ISO
    AVC1,
    ISO2,
    ISOM,
    MP21,
    MP41,
    MP42,

    // Apple
    QT,
    M4B,
    M4P,
    M4A,
    M4V,
    M4VH,
    M4VP,

    // Adobe
    F4V,
    F4P,
    F4A,
    F4B,

    // 3GPP/GSM
    MMP4,
}

impl FromStr for FileType {
    type Err = &'static str;

    /// Returns the FileType enum from a string.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "avc1" => Ok(Self::AVC1),
            "iso2" => Ok(Self::ISO2),
            "isom" => Ok(Self::ISOM),
            "mp21" => Ok(Self::MP21),
            "mp41" => Ok(Self::MP41),
            "mp42" => Ok(Self::MP42),
            "qt" | "qt\u{0}\u{0}" => Ok(Self::QT),
            "M4B" | "M4B\u{0}" => Ok(Self::M4B),
            "M4P" | "M4P\u{0}" => Ok(Self::M4P),
            "M4A" | "M4A\u{0}" => Ok(Self::M4A),
            "M4V" | "M4V\u{0}" => Ok(Self::M4V),
            "M4VH" => Ok(Self::M4VH),
            "M4VP" => Ok(Self::M4VP),
            "F4V" | "F4V\u{0}" => Ok(Self::F4V),
            "F4P" | "F4P\u{0}" => Ok(Self::F4P),
            "F4A" | "F4A\u{0}" => Ok(Self::F4A),
            "F4B" | "F4B\u{0}" => Ok(Self::F4B),
            "mmp4" => Ok(Self::MMP4),
            _ => Err("Unknown FileType"),
        }
    }
}

impl std::fmt::Display for FileType {
    /// Returns the string representation of the FileType enum for display.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::AVC1 => "avc1",
            Self::ISO2 => "iso2",
            Self::ISOM => "isom",
            Self::MP21 => "mp21",
            Self::MP41 => "mp41",
            Self::MP42 => "mp42",
            Self::QT => "qt",
            Self::M4B => "M4B",
            Self::M4P => "M4P",
            Self::M4A => "M4A",
            Self::M4V => "M4V",
            Self::M4VH => "M4VH",
            Self::M4VP => "M4VP",
            Self::F4V => "F4V",
            Self::F4P => "F4P",
            Self::F4A => "F4A",
            Self::F4B => "F4B",
            Self::MMP4 => "mmp4",
        };
        write!(f, "{s}")
    }
}

impl FileType {
    /// Returns the FileType enum from a byte array.
    ///
    /// # Arguments
    ///
    /// * `bytes` - The byte array to convert to a FileType enum.
    ///
    /// # Returns
    ///
    /// * `Result<FileType, &'static str>` - The FileType enum.
    ///
    /// # Errors
    ///
    /// * `ftyp parse error.` - If the byte array cannot be converted to a string.
    pub fn from_bytes(bytes: [u8; 4]) -> Result<Self, &'static str> {
        let_ok!(s, str::from_utf8(&bytes), "ftyp parse error.");
        Self::from_str(s)
    }

    /// Returns the byte array representation of the FileType enum.
    ///
    /// # Returns
    ///
    /// * `Vec<u8>` - The byte array representation of the FileType enum.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}

/// Describes the file type and version information. This is the first atom in the file.
///
/// Files written to this version of this specification must contain a file-type box.
/// For compatibility with an earlier version of this specification,
/// files may be conformant to this specification and not contain a file-type box.
/// Files with no file-type box should be read as if they contained an FTYP box
/// with `major_brand=MP41`, `minor_version=0`, and the single compatible brand `MP41`.
///
/// A media-file structured to this part of this specification may be compatible with
/// more than one detailed specification, and it is therefore not always possible to
/// speak of a single ‘type’ or ‘brand’ for the file. This means that the utility of
/// the file name extension and Multipurpose Internet Mail Extension (MIME) type are somewhat reduced.
///
/// This box must be placed as early as possible in the file (e.g. after any obligatory signature,
/// but before any significant variable-size boxes such as a Movie Box, Media Data Box, or Free Space).
/// It identifies which specification is the ‘best use’ of the file, and a minor version of that specification;
/// and also a set of other specifications to which the file complies.
/// Readers implementing this format should attempt to read files that are marked as compatible with
/// any of the specifications that the reader implements.
/// Any incompatible change in a specification should therefore register a new ‘brand’
/// identifier to identify files conformant to the new specification.
///
/// The minor version is informative only. It does not appear for compatible-brands,
/// and must not be used to determine the conformance of a file to a standard.
/// It may allow more precise identification of the major specification,
/// for inspection, debugging, or improved decoding.
///
/// Files would normally be externally identified (e.g. with a file extension or mime type)
/// that identifies the ‘best use’ (major brand),
/// or the brand that the author believes will provide the greatest compatibility.
///
/// This section of this specification does not define any brands.
/// However, see subclause 6.3 below for brands for files conformant to the whole specification and not just this section.
/// All file format brands defined in this specification are included in Annex E with a summary of which features they require.
///
/// - Box Type: `Ftyp`
/// - Container: File
/// - Mandatory: Yes
/// - Quantity: Exactly one (but see above)
#[derive(Debug, Clone)]
pub struct Ftyp {
    /// Header of the `Ftyp` atom.
    pub header: Header,

    /// The file type. This is the first 4 bytes of the file.
    /// See the `FileType` enum for the possible values.
    pub major_brand: FileType,

    /// The minor version of the file. This is the next 4 bytes of the file.
    /// This is usually 0. If it is not 0, then the file is not compatible with this spec.
    pub minor_version: u32,

    /// The compatible brands. This is the rest of the bytes in the file. Each compatible brand
    /// is 4 bytes long. This is a list of all the compatible brands.
    pub compatible_brands: Vec<FileType>,
}

impl Ftyp {
    /// Determine the file type from the first 4 bytes of the file.
    ///
    /// # Arguments
    ///
    /// * `f` - `Mp4File` to read from.
    ///
    /// # Returns
    ///
    /// * `Result<FileType, &'static str>` - The file type.
    ///
    /// # Errors
    ///
    /// * `Ftyp: Unable to read filetype byte 1` - If the first byte cannot be read.
    /// * `Ftyp: Unable to read filetype byte 2` - If the second byte cannot be read.
    /// * `Ftyp: Unable to read filetype byte 3` - If the third byte cannot be read.
    /// * `Ftyp: Unable to read filetype byte 4` - If the fourth byte cannot be read.
    fn parse_filetype(f: &mut Mp4File) -> Result<FileType, &'static str> {
        let_ok!(b1, f.read_u8(), "Ftyp: Unable to read filetype byte 1");
        let_ok!(b2, f.read_u8(), "Ftyp: Unable to read filetype byte 2");
        let_ok!(b3, f.read_u8(), "Ftyp: Unable to read filetype byte 3");
        let_ok!(b4, f.read_u8(), "Ftyp: Unable to read filetype byte 4");

        let ft_bytes: [u8; 4] = [b1, b2, b3, b4];
        FileType::from_bytes(ft_bytes)
    }

    /// Parse the ftyp box. The ftyp box is the first box in the file.
    /// It contains the file type and version information, as well as the compatible brands.
    ///
    /// # Arguments
    ///
    /// * `f` - `Mp4File` to read from.
    /// * `header` - `Header` of the `Ftyp` atom.
    ///
    /// # Returns
    ///
    /// * `Self` - The parsed `Ftyp` atom.
    ///
    /// # Errors
    ///
    /// * `Ftyp: Unable to parse filetype.` - If the file type cannot be parsed.
    /// * `Ftyp: Unable to read the minor version.` - If the minor version cannot be read.
    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str> {
        let_ok!(
            major_brand,
            Self::parse_filetype(f),
            "Ftyp: Unable to parse filetype."
        );
        let_ok!(
            minor_version,
            f.read_u32(),
            "Ftyp: Unable read the minor version."
        );

        let mut compatible_brands: Vec<FileType> = Vec::new();

        let mut idx = (header.data_size - 8) / 4;
        while idx > 0 {
            let_ok!(
                ft,
                Self::parse_filetype(f),
                "Ftyp: Unable to parse filetype."
            );
            compatible_brands.push(ft);
            idx -= 1;
        }

        f.offset_inc(header.data_size);

        log::trace!("Ftyp::parse -- header = {header:?}, major_brand = {major_brand:?}, minor_version = {minor_version}, compatible_brands = {compatible_brands:?}");

        Ok(Self {
            header,
            major_brand,
            minor_version,
            compatible_brands,
        })
    }

    retref!(header, Header);
    retval!(major_brand, FileType);
    retval!(minor_version, u32);
    retref!(compatible_brands, Vec<FileType>);
}
