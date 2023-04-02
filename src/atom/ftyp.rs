// Complete List of all known MP4 / QuickTime 'ftyp' designations:
//  http://www.fileType.com

#![allow(clippy::doc_markdown)]
/**

**/
use std::str;
use std::str::FromStr;
use std::string::ToString;

use crate::{let_ok, retref, retval};

use super::{Header, Mp4File};

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone)]
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
            _ => Err("unknow fileType"),
        }
    }
}

impl ToString for FileType {
    fn to_string(&self) -> String {
        match *self {
            Self::AVC1 => "avc1".to_owned(),
            Self::ISO2 => "iso2".to_owned(),
            Self::ISOM => "isom".to_owned(),
            Self::MP21 => "mp21".to_owned(),
            Self::MP41 => "mp41".to_owned(),
            Self::MP42 => "mp42".to_owned(),
            Self::QT => "qt\u{0}\u{0}".to_owned(),
            Self::M4B => "M4B\u{0}".to_owned(),
            Self::M4P => "M4P\u{0}".to_owned(),
            Self::M4A => "M4A\u{0}".to_owned(),
            Self::M4V => "M4V\u{0}".to_owned(),
            Self::M4VH => "M4VH".to_owned(),
            Self::M4VP => "M4VP".to_owned(),
            Self::F4V => "F4V\u{0}".to_owned(),
            Self::F4P => "F4P\u{0}".to_owned(),
            Self::F4A => "F4A\u{0}".to_owned(),
            Self::F4B => "F4B\u{0}".to_owned(),
            Self::MMP4 => "mmp4".to_owned(),
        }
    }
}

impl FileType {
    pub fn from_bytes(bytes: [u8; 4]) -> Result<Self, &'static str> {
        let_ok!(s, str::from_utf8(&bytes), "ftyp parse error.");
        Self::from_str(s)
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}

// Ref: ftyp.md
#[derive(Debug, Clone)]
pub struct Ftyp {
    header: Header,
    major_brand: FileType,
    minor_version: u32,
    compatible_brands: Vec<FileType>,
}

impl Ftyp {
    fn parse_filetype(f: &mut Mp4File) -> Result<FileType, &'static str> {
        let_ok!(b1, f.read_u8(), "Ftyp: Unable to read filetype byte 1");
        let_ok!(b2, f.read_u8(), "Ftyp: Unable to read filetype byte 2");
        let_ok!(b3, f.read_u8(), "Ftyp: Unable to read filetype byte 3");
        let_ok!(b4, f.read_u8(), "Ftyp: Unable to read filetype byte 4");

        let ft_bytes: [u8; 4] = [b1, b2, b3, b4];
        FileType::from_bytes(ft_bytes)
    }

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

        Ok(Self {
            header,
            major_brand,
            minor_version,
            compatible_brands,
        })
    }

    retref!(header, Header);
    retref!(major_brand, FileType);
    retval!(minor_version, u32);
    retref!(compatible_brands, Vec<FileType>);
}
