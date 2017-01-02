
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

Atoms:

ftyp
pdin
moov
    mvhd
    trak
        tkhd
        mdia
            mdhd
            hdlr
            minf
                stbl
                    stsd
                    stts
                    stsc
                    stsz
                    stz2
                    stss
                    stco
                    co64
                    
                    ctts
                    stsh
                    padb
                    stdp
                    sdtp
                    sbgp
                    sgpd
                    subs
                dinf
                    dref
                nmhd
                hmhd
                smhd
                vmhd
        tref
        edts
            elst
    mvex
        mehd
        trex
    ipmc
moof
    mfhd
    traf
        tfhd
        trun
        sdtp
        sbgp
        subs
mfra
    tfra
    mfro
mdat
free
skip
    udta
        cprt
        tsel
        strk
            stri
            strd
meta
    hdlr
    dinf
        dref
    ipmc
    iloc
    ipro
        sinf
            frma
            imif
            schm
            schi
    iinf
    xml
    bxml
    pitm
    fiin
        paen
            fpar
            fecr
        segr
        gitn
        tsel
meco
    mere


Top level Atoms:

ftyp
pdin
moov
mfra
mdat
free
skip
meta
meco

Fragment

Initialization Segments
    ftyp
    moov
    moof
    mdat
    
Media Segments

    moof
        mfhd
        traf
            tfhd
            trun
            sdtp
        traf
            tfhd
            trun
            sdtp
    mdat
**/

use std::str;
use std::string::String;
use std::mem::transmute;
use std::fs::File;

use std::str::FromStr;
use std::string::ToString;
use std::convert::AsRef;

use std::io::{Write, Read, ErrorKind, SeekFrom, Seek};
use ::byteorder::{BigEndian, ReadBytesExt};
pub use super::Mp4File;

/**
let mut f = try!(File::open("foo.txt"));

// move the cursor 42 bytes from the start of the file
try!(f.seek(SeekFrom::Start(42)));

**/
mod kind;

mod ftyp;
mod mdat;
mod meco;
mod meta;
mod mfra;
mod moof;
mod moov;
mod pdin;
mod skip;
mod free;

pub use self::kind::Kind;

use self::ftyp::Ftyp;
use self::free::Free;
use self::skip::Skip;
use self::mdat::Mdat;

#[derive(Debug, Clone)]
pub struct Header {
    size       : u64,  // atom header size, not include data size.
    kind       : Kind, // atom type
    atom_size  : u64,  // atom size , include header and data.
    data_size  : u64,  // atom data size , not include header size.
    offset     : u64   // file offset.
}

#[derive(Debug, Clone)]
pub enum Atom {
    ftyp(Ftyp),
    free(Free),
    skip(Skip),
    mdat(Mdat)
}

impl Atom {
    fn parse_header(f: &mut Mp4File) -> Result<Header, &'static str>{
        let curr_offset = f.offset();
        let atom_size_u32: u32 = f.read_u32().unwrap();

        let kind_bytes: [u8; 4] = [
            f.read_u8().unwrap(), f.read_u8().unwrap(),
            f.read_u8().unwrap(), f.read_u8().unwrap(),
        ];
        let atom_kind = Kind::from_bytes(&kind_bytes).unwrap();

        let mut atom_size   = 0u64;   // atom box Size
        let mut header_size = 8u64;   // atom header size
        let mut data_size   = 0u64;   // atom data size

        if atom_size_u32 < 1 {
            return Err("can not parse this mp4 file.");
        } else if atom_size_u32 == 1 {
            atom_size = f.read_u64().unwrap();
            header_size += 8;
        } else {
            atom_size = atom_size_u32 as u64;
        }

        data_size = atom_size - header_size;

        f.offset_inc(header_size);

        Ok(Header{
            size       : header_size,
            kind       : atom_kind,
            atom_size  : atom_size,
            data_size  : data_size,
            offset     : curr_offset
        })
    }
    pub fn parse(f: &mut Mp4File) -> Result<Self, &'static str> {
        let header = Atom::parse_header(f).unwrap();
        // println!("DO: \n{:?}", header);
        let data = match header.kind {
            // Kind::bxml => ,
            // Kind::co64 => ,
            // Kind::cprt => ,
            // Kind::ctts => ,
            // Kind::dinf => ,
            // Kind::dinf => ,
            // Kind::dref => ,
            // Kind::dref => ,
            // Kind::edts => ,
            // Kind::elst => ,
            // Kind::fecr => ,
            // Kind::fiin => ,
            // Kind::fpar => ,
            Kind::free => {
                Ok(Atom::free(Free::parse(f, header).unwrap()))
            },
            // Kind::frma => ,
            Kind::ftyp => {
                Ok(Atom::ftyp(Ftyp::parse(f, header).unwrap()))
            },
            // Kind::hdlr => ,
            // Kind::hdlr => ,
            // Kind::hmhd => ,
            // Kind::iinf => ,
            // Kind::iloc => ,
            // Kind::imif => ,
            // Kind::ipmc => ,
            // Kind::ipmc => ,
            // Kind::ipro => ,
            // Kind::itn  => ,
            Kind::mdat => {
                Ok(Atom::mdat(Mdat::parse(f, header).unwrap()))
            },
            // Kind::mdhd => ,
            // Kind::mdia => ,
            // Kind::meco => ,
            // Kind::mehd => ,
            // Kind::mere => ,
            // Kind::meta => ,
            // Kind::mfhd => ,
            // Kind::mfra => ,
            // Kind::mfro => ,
            // Kind::minf => ,
            // Kind::moof => ,
            // Kind::moov => ,
            // Kind::mvex => ,
            // Kind::mvhd => ,
            // Kind::nmhd => ,
            // Kind::padb => ,
            // Kind::paen => ,
            // Kind::pdin => ,
            // Kind::pitm => ,
            // Kind::sbgp => ,
            // Kind::sbgp => ,
            // Kind::schi => ,
            // Kind::schm => ,
            // Kind::sdtp => ,
            // Kind::sdtp => ,
            // Kind::sgpd => ,
            // Kind::sinf => ,
            // Kind::skip => ,
            // Kind::smhd => ,
            // Kind::stbl => ,
            // Kind::stco => ,
            // Kind::stdp => ,
            // Kind::stsc => ,
            // Kind::stsd => ,
            // Kind::stsh => ,
            // Kind::stss => ,
            // Kind::stsz => ,
            // Kind::stts => ,
            // Kind::stz2 => ,
            // Kind::subs => ,
            // Kind::subs => ,
            // Kind::tfhd => ,
            // Kind::tfra => ,
            // Kind::tkhd => ,
            // Kind::traf => ,
            // Kind::trak => ,
            // Kind::tref => ,
            // Kind::trex => ,
            // Kind::trun => ,
            // Kind::tsel => ,
            // Kind::udta => ,
            // Kind::vmhd => ,
            // Kind::xml  => ,
            // Kind::strk => ,
            // Kind::stri => ,
            // Kind::strd => 
            _ => {
                println!("[ERROR] UNKNOW ATOM TYPE({}).", header.kind.to_string() );
                println!("[DEBUG] Header: {:?}", header);
                return Err("this atom type not support yet.")   
            }
        };
        data
    }
}

