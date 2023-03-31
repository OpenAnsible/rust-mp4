// Complete List of all known MP4 / QuickTime 'ftyp' designations:
//  http://www.fileType.com

#![allow(clippy::doc_markdown)]
/**
avc1    MP4 Base w/ AVC ext [ISO 14496-12:2005]         ISO     YES video/mp4   [11]
iso2    MP4 Base Media v2 [ISO 14496-12:2005]           ISO     YES video/mp4   [6]
isom    MP4  Base Media v1 [IS0 14496-12:2003]          ISO     YES video/mp4   [5]
mp21    MPEG-21 [ISO/IEC 21000-9]                       ISO     YES various
mp41    MP4 v1 [ISO 14496-1:ch13]                       ISO     YES video/mp4
mp42    MP4 v2 [ISO 14496-14]                           ISO     YES video/mp4

qt      Apple QuickTime (.MOV/QT)                       Apple   YES video/quicktime
M4B     Apple iTunes AAC-LC (.M4B) Audio Book           Apple   YES audio/mp4   [9]
M4P     Apple iTunes AAC-LC (.M4P) AES Protected Audio  Apple   YES audio/mp4   [9]
M4A     Apple iTunes AAC-LC (.M4A) Audio                Apple   YES audio/x-m4a [9]
M4V     Apple iTunes Video (.M4V) Video                 Apple   YES video/x-m4v [9]
M4VH    Apple TV (.M4V)                                 Apple   NO  video/x-m4v
M4VP    Apple iPhone (.M4V)                             Apple   NO  video/x-m4v

F4V     Video for Adobe Flash Player 9+ (.F4V)              Adobe    NO video/mp4
F4P     Protected Video for Adobe Flash Player 9+ (.F4P)    Adobe    NO video/mp4
F4A     Audio for Adobe Flash Player 9+ (.F4A)              Adobe    NO audio/mp4
F4B     Audio Book for Adobe Flash Player 9+ (.F4B)         Adobe    NO audio/mp4

mmp4    MPEG-4/3GPP Mobile Profile(.MP4/.3GP)(for NTT)      3GPP/GSM NO video/mp4

All `Ftyp`-es must contain 4 characters.
If three characters are shown in the table, a trailing blank (i.e. a space character; ASCII 0x20) is implied.
Similarly, if only two characters are shown, two trailing blanks are implied.
For example, "qt" is really "qt  " - note the two trailing spaces
**/
use std::str;
use std::str::FromStr;
use std::string::ToString;

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
    pub fn from_bytes(bytes: &[u8; 4]) -> Result<Self, &'static str> {
        let Ok(s) = str::from_utf8(bytes) else {
                println!("ftyp ({bytes:?}) parse error.");
                return Err("ftyp parse error.");
        };
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
        let ft_bytes: [u8; 4] = [
            f.read_u8().unwrap(),
            f.read_u8().unwrap(),
            f.read_u8().unwrap(),
            f.read_u8().unwrap(),
        ];
        FileType::from_bytes(&ft_bytes)
    }

    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str> {
        let major_brand = Self::parse_filetype(f).unwrap();
        let minor_version = f.read_u32().unwrap();
        let mut compatible_brands: Vec<FileType> = Vec::new();

        let mut idx = (header.data_size - 8) / 4;
        while idx > 0 {
            compatible_brands.push(Self::parse_filetype(f).unwrap());
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

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn major_brand(&self) -> &FileType {
        &self.major_brand
    }

    pub fn minor_version(&self) -> u32 {
        self.minor_version
    }

    pub fn compatible_brands(&self) -> &Vec<FileType> {
        &self.compatible_brands
    }
}
