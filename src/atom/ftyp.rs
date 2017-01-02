
// Complete List of all known MP4 / QuickTime 'ftyp' designations:
//  http://www.fileType.com

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

All ftyp's must contain 4 characters. 
If three characters are shown in the table, 
a trailing blank (i.e. a space character; ASCII 0x20) is implied. 
Similarly, if only two characters are shown, 
two trailing blanks are implied. 
For example, "qt" is really "qt  " - note the two trailing spaces
**/

use std::str;
use std::str::FromStr;
use std::string::ToString;
use std::fs::File;

use super::{Mp4File, Kind, Header, Atom};

#[derive(Debug, Clone)]
pub enum FileType{
    // ISO
    avc1,
    iso2,
    isom,
    mp21,
    mp41,
    mp42,
    // Apple
    qt,
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
    mmp4
}

impl FromStr for FileType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err>{
        match s {
            "avc1" => Ok(FileType::avc1),
            "iso2" => Ok(FileType::iso2),
            "isom" => Ok(FileType::isom),
            "mp21" => Ok(FileType::mp21),
            "mp41" => Ok(FileType::mp41),
            "mp42" => Ok(FileType::mp42),
            "qt"  | "qt\u{0}\u{0}" => Ok(FileType::qt),
            "M4B" | "M4B\u{0}" => Ok(FileType::M4B),
            "M4P" | "M4P\u{0}" => Ok(FileType::M4P),
            "M4A" | "M4A\u{0}" => Ok(FileType::M4A),
            "M4V" | "M4V\u{0}" => Ok(FileType::M4V),
            "M4VH" => Ok(FileType::M4VH),
            "M4VP" => Ok(FileType::M4VP),
            "F4V" | "F4V\u{0}" => Ok(FileType::F4V),
            "F4P" | "F4P\u{0}" => Ok(FileType::F4P),
            "F4A" | "F4A\u{0}" => Ok(FileType::F4A),
            "F4B" | "F4B\u{0}" => Ok(FileType::F4B),
            "mmp4" => Ok(FileType::mmp4),
            _ => Err("unknow fileType")
        }
    }
}

impl ToString for FileType {
    fn to_string(&self) -> String {
        match *self {
            FileType::avc1 => "avc1".to_owned(),
            FileType::iso2 => "iso2".to_owned(),
            FileType::isom => "isom".to_owned(),
            FileType::mp21 => "mp21".to_owned(),
            FileType::mp41 => "mp41".to_owned(),
            FileType::mp42 => "mp42".to_owned(),
            FileType::qt   => "qt\u{0}\u{0}".to_owned(),
            FileType::M4B  => "M4B\u{0}".to_owned(),
            FileType::M4P  => "M4P\u{0}".to_owned(),
            FileType::M4A  => "M4A\u{0}".to_owned(),
            FileType::M4V  => "M4V\u{0}".to_owned(),
            FileType::M4VH => "M4VH".to_owned(),
            FileType::M4VP => "M4VP".to_owned(),
            FileType::F4V  => "F4V\u{0}".to_owned(),
            FileType::F4P  => "F4P\u{0}".to_owned(),
            FileType::F4A  => "F4A\u{0}".to_owned(),
            FileType::F4B  => "F4B\u{0}".to_owned(),
            FileType::mmp4 => "mmp4".to_owned()
        }
    }
}

impl FileType {
    pub fn from_bytes(bytes: &[u8; 4]) -> Result<Self, &'static str> {
        let kind_str = match str::from_utf8(bytes) {
            Ok(s)  => s,
            Err(_) => {
                println!("ftyp ({:?}) parse error.", bytes);
                return Err("ftyp parse error.");
            }
        };
        FileType::from_str(kind_str)
    }
    pub fn into_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}

/**
    Ftyp Box

    4.3.2 Syntax

    aligned(8) class FileTypeBox
    extends Box(‘ftyp’) {
        unsigned int(32) major_brand;
        unsigned int(32) minor_version;
        unsigned int(32) compatible_brands[]; // to end of the box
    }

    4.3.3 Semantics

    This box identifies the specifications to which this file complies.
    Each brand is a printable four-character code, registered with ISO, that identifies a precise specification.
    major_brand – is a brand identifier
    minor_version – is an informative integer for the minor version of the major brand compatible_brands – is a list, to the end of the box, of brands
**/

#[derive(Debug, Clone)]
pub struct Ftyp {
    header: Header,
    major_brand  : FileType,
    minor_version: u32,
    compatible_brands: Vec<FileType>
}

impl Ftyp {
    fn parse_filetype(f: &mut Mp4File) -> Result<FileType, &'static str>{
        let ft_bytes: [u8; 4] = [
            f.read_u8().unwrap(), f.read_u8().unwrap(),
            f.read_u8().unwrap(), f.read_u8().unwrap()
        ];
        FileType::from_bytes(&ft_bytes)
    }
    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str>{
        let major_brand = Ftyp::parse_filetype(f).unwrap(); // NOTE: 文档上描述是 i32 类型
        let minor_version = f.read_u32().unwrap();          // NOTE: 文档上描述是 i32 类型
        let mut compatible_brands: Vec<FileType> = Vec::new();
        let mut idx = (header.data_size - 8) / 4;
        while idx > 0 {
            // // NOTE: 文档上描述是 i32 类型
            compatible_brands.push(Ftyp::parse_filetype(f).unwrap());
            idx -= 1;
        }
        f.offset_inc(header.data_size);
        Ok(Ftyp{
            header: header,
            major_brand: major_brand,
            minor_version: minor_version,
            compatible_brands: compatible_brands
        })
    }
}
