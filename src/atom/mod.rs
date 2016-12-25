
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

use std::io::{Write, Read, ErrorKind, SeekFrom, Seek};
use ::byteorder::{BigEndian, ReadBytesExt};

/**
let mut f = try!(File::open("foo.txt"));

// move the cursor 42 bytes from the start of the file
try!(f.seek(SeekFrom::Start(42)));

**/



#[derive(Debug, Clone)]
pub struct Atom {
    size     : u32,
    kind     : String,
    largesize: Option<u64>,
    header_size: u64,
    data_size  : u64,
    data       : Option<Vec<u8>>,
    children   : Option<Vec<Atom>>
}

impl Atom {
    pub fn parse(&mut self, f: &mut File, offset: &mut ::Offset){
        match self.kind.as_ref() {
            "ftyp" => {
                let mut data: Vec<u8> = vec![];
                for _ in 0..self.data_size {
                    data.push(f.read_u8().unwrap());
                }
                self.data = Some(data);
                offset.update(self.data_size);
            },
            "moov" => {
                let mut atoms: Vec<Atom> = vec![];
                let mut idx = 0;
                while idx < self.data_size {
                    let atom = parse(f, offset).unwrap();
                    idx += atom.size as u64;
                    atoms.push(atom);
                }
                self.children = Some(atoms);
            },
            "mdat" => {
                let mut data: Vec<u8> = vec![];
                // for _ in 0..self.data_size {
                //     f.read_u8().unwrap();
                // }
                f.seek(SeekFrom::Start(offset.offset() + self.data_size));

                self.data = Some(data);
                offset.update(self.data_size);
            },
            "free" => {
                let mut data: Vec<u8> = vec![];
                for _ in 0..self.data_size {
                    // data.push();
                    f.read_u8().unwrap();
                }
                self.data = Some(data);
                offset.update(self.data_size);
            },
            "uuid" => {
                let mut data: Vec<u8> = vec![];
                for _ in 0..self.data_size {
                    // data.push(f.read_u8().unwrap());
                    f.read_u8().unwrap();
                }
                self.data = Some(data);
                offset.update(self.data_size);
            },
            _ => {
                let mut data: Vec<u8> = vec![];
                for _ in 0..self.data_size {
                    data.push(f.read_u8().unwrap());
                }
                self.data = Some(data);
                offset.update(self.data_size);
            }
        }
    }
}

pub fn parse(f: &mut File, offset: &mut ::Offset) -> Result<Atom, &'static str>{
    let atom_size: u32 = f.read_u32::<BigEndian>().unwrap();

    let _bytes = [
        f.read_u8().unwrap(), f.read_u8().unwrap(),
        f.read_u8().unwrap(), f.read_u8().unwrap(),
    ];
    let atom_kind = unsafe {str::from_utf8_unchecked(&_bytes[..4])}.to_owned();

    let mut atom_largesize: Option<u64> = None;
    let mut header_size = 8u64;
    let mut data_size   = 0u64;

    if atom_size < 1 {
        return Err("can not parse this mp4 file.");
    } else if atom_size == 1 {
        let atom_largesize = Some(f.read_u64::<BigEndian>().unwrap());
        header_size += 8;
        data_size    = atom_largesize.unwrap() - header_size;
    } else {
        data_size    = atom_size as u64 - header_size;
    }
    offset.update(header_size);

    let mut return_data = Atom {
        size  : atom_size,
        kind  : atom_kind,
        largesize  : atom_largesize,
        header_size: header_size,
        data_size  : data_size,
        data       : None,
        children   : None
    };
    println!("[INFO] parse children . {:?}", return_data.kind);
    return_data.parse(f, offset);
    Ok(return_data)
}

