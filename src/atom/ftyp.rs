
// Complete List of all known MP4 / QuickTime 'ftyp' designations:
//  http://www.ftyps.com

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

use std::str::FromStr;
use std::string::ToString;

pub const FTYP_LENGTH: usize = 4;

#[derive(Debug, Clone)]
pub enum Ftyp{
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

impl FromStr for Ftyp {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err>{
        match s {
            "avc1" => Ok(Ftyp::avc1),
            "iso2" => Ok(Ftyp::iso2),
            "isom" => Ok(Ftyp::isom),
            "mp21" => Ok(Ftyp::mp21),
            "mp41" => Ok(Ftyp::mp41),
            "mp42" => Ok(Ftyp::mp42),
            "qt"  | "qt  " => Ok(Ftyp::qt),
            "M4B" | "M4B " => Ok(Ftyp::M4B),
            "M4P" | "M4P " => Ok(Ftyp::M4P),
            "M4A" | "M4A " => Ok(Ftyp::M4A),
            "M4V" | "M4V " => Ok(Ftyp::M4V),
            "M4VH" => Ok(Ftyp::M4VH),
            "M4VP" => Ok(Ftyp::M4VP),
            "F4V" | "F4V " => Ok(Ftyp::F4V),
            "F4P" | "F4P " => Ok(Ftyp::F4P),
            "F4A" | "F4A " => Ok(Ftyp::F4A),
            "F4B" | "F4B " => Ok(Ftyp::F4B),
            "mmp4" => Ok(Ftyp::mmp4),
            _ => Err("unknow ftyp")
        }
    }
}

impl ToString for Ftyp {
    fn to_string(&self) -> String {
        match *self {
            Ftyp::avc1 => "avc1".to_owned(),
            Ftyp::iso2 => "iso2".to_owned(),
            Ftyp::isom => "isom".to_owned(),
            Ftyp::mp21 => "mp21".to_owned(),
            Ftyp::mp41 => "mp41".to_owned(),
            Ftyp::mp42 => "mp42".to_owned(),
            Ftyp::qt   => "qt".to_owned(),
            Ftyp::M4B  => "M4B".to_owned(),
            Ftyp::M4P  => "M4P".to_owned(),
            Ftyp::M4A  => "M4A".to_owned(),
            Ftyp::M4V  => "M4V".to_owned(),
            Ftyp::M4VH => "M4VH".to_owned(),
            Ftyp::M4VP => "M4VP".to_owned(),
            Ftyp::F4V  => "F4V".to_owned(),
            Ftyp::F4P  => "F4P".to_owned(),
            Ftyp::F4A  => "F4A".to_owned(),
            Ftyp::F4B  => "F4B".to_owned(),
            Ftyp::mmp4 => "mmp4".to_owned()
        }
    }
}

impl Ftyp {
    pub fn from_bytes() -> Result<Self, &'static str> {
        Err(".")
    }
    pub fn into_bytes(&self) -> Vec<u8> {
        let mut bytes = self.to_string().into_bytes();
        for _ in (0 .. (FTYP_LENGTH - bytes.len())) {
            bytes.push(0u8);
        }
        bytes
    }
}
