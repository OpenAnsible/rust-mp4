
// https://wiki.multimedia.cx/index.php/QuickTime_container
// http://developer.apple.com/documentation/QuickTime/QTFF/index.html
// http://www.adobe.com/devnet/video/articles/mp4_movie_atom.html
// http://mpeg.chiariglione.org/standards/mpeg-4/mpeg-4.htm

// http://www.adobe.com/devnet/f4v.html

// box types: http://mp4ra.org/atoms.html

/**
    Box Struct:
    
        size(u32), type(u32), largesize(u64),
        data

其中, `size` 指明了整个 `box` 的大小, 包括 `header` 部分.
如果 `box` 大小超过了 `u32` 的最大数值, `size` 就被设置为 `1` ,
并用接下来的 `8位` u64 来存放大小。


Top level Box:

    ftyp
    moov
    mdat
**/

use std::str;
use std::string::String;
use std::mem::transmute;
use std::fs::File;
use std::ops::Add;

use std::str::FromStr;
use std::string::ToString;
use std::convert::AsRef;

use std::io::{Write, Read, ErrorKind};
use ::byteorder::{BigEndian, ReadBytesExt};

pub mod ftyp;
pub mod moov;
pub mod mdat;


/**
    
    length: 4 char
**/
#[derive(Debug, Clone)]
pub enum AtomType {
    ftyp, // 
    moov,
    mdat,
    unknow(String)
}

impl FromStr for AtomType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err>{
        match s {
            "ftyp" => Ok(AtomType::ftyp),
            "moov" => Ok(AtomType::moov),
            "mdat" => Ok(AtomType::mdat),
            _      => Ok(AtomType::unknow(s.to_owned()))
        }
    }
}


impl AtomType {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str>{
        assert_eq!(bytes.len(), 4);
        let s = match str::from_utf8(&bytes[..4]){
            Ok(s)  => s,
            Err(_) => {
                println!("UTF8 Error: {:?}", bytes);
                " ... "
            }
        };
        AtomType::from_str(s)
    }
}


#[derive(Debug, Clone)]
pub struct AtomBox {
    size     : u32,
    type_    : AtomType,
    largesize: Option<u64>,
    data     : Vec<u8>,
    // children : Vec<AtomBox>
}

impl AtomBox {

}



pub fn parse(f: &mut File, offset: &mut ::Offset) -> Result<AtomBox, &'static str>{
    let atom_size: u32 = f.read_u32::<BigEndian>().unwrap();
    let atom_type = AtomType::from_bytes(&[
            f.read_u8().unwrap(), f.read_u8().unwrap(),
            f.read_u8().unwrap(), f.read_u8().unwrap(),
    ]).unwrap();

    let mut atom_largesize: Option<u64> = None;
    let mut header_size = 8u64;
    let mut data_size   = 0u64;

    if atom_size < 1 {
        return Err("can not parse this mp4 file.");
    } else if atom_size == 1 {
        let atom_largesize = Some(f.read_u64::<BigEndian>().unwrap());
        header_size += 8;
        data_size    = atom_largesize.unwrap() - header_size;
        offset.update(atom_largesize.unwrap());
    } else {
        data_size    = atom_size as u64 - header_size;
        offset.update(atom_size as u64);
    }

    let mut data: Vec<u8> = vec![];
    for _ in (0..data_size) {
        f.read_u8().unwrap();
        // data.push(f.read_u8().unwrap());
    }

    let return_data = AtomBox {
        size  : atom_size,
        type_ : atom_type,
        largesize: atom_largesize,
        data  : vec![]
    };
    Ok(return_data)
}

