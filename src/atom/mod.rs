
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

mod kind;

mod ftyp;
mod mdat;
mod pdin;
mod freespace;
mod uuid;
mod meco;
mod meta;
mod mfra;
mod moof;
mod moov;
mod ignore;
mod unrecognized;

pub use self::kind::Kind;

use self::ftyp::Ftyp;
use self::freespace::{Free, Skip};
use self::mdat::Mdat;
use self::pdin::Pdin;
use self::uuid::Uuid;

use self::moov::{
    Moov, Mvhd, Trak, Tkhd, Tref, Mdia, Mdhd, Hdlr,
    Minf, Vmhd, Smhd, Hmhd, Nmhd, Stbl, Stsd, Stdp,
    Stts, Ctts, Cslg, Stss, Stsh, Sdtp, Stsc, Stsz,
    Stz2, Stco, Co64, Padb, 
    Mvex, Mehd, Trex
};
use self::moof::{
    Moof, Mfhd, Traf, Tfhd, Trun, 
};
use self::mfra::{
    Mfra, Tfra, Mfro
};
use self::ignore::Ignore;
use self::unrecognized::Unrecognized;

#[derive(Debug, Clone)]
pub struct Entry {
    first_chunk             : u32,
    samples_per_chunk       : u32,
    sample_description_index: u32
}

#[derive(Debug, Clone)]
pub struct Sample {
    duration: Option<u32>,
    size    : Option<u32>,
    flags   : Option<u32>,
    composition_time_offset: Option<i32>,
    description_index      : Option<u32>
}

/**
    aligned(8) class Box (unsigned int(32) boxtype,
                          optional unsigned int(8)[16] extended_type) {
        unsigned int(32) size;
        unsigned int(32) type = boxtype;
        if (size==1) {
            unsigned int(64) largesize;
        } else if (size==0) {
            // box extends to end of file
        }
        if (boxtype==‘uuid’) {
            unsigned int(8)[16] usertype = extended_type;
        }
    }

    The semantics of these two fields are:

        `size` is an integer that specifies the number of bytes in this box, 
            including all its fields and contained boxes; if size is 1 then the actual size is 
            in the field largesize; if size is 0, then this box is the last one in the file, 
            and its contents extend to the end of the file (normally only used for a Media Data Box)
        `type` identifies the box type; standard boxes use a compact type, 
            which is normally four printable characters, to permit ease of identification, 
            and is shown so in the boxes below. User extensions use an extended type; in this case, 
            the type field is set to ‘uuid’.

    Boxes with an unrecognized type shall be ignored and skipped. 

    Many objects also contain a version number and flags field:
    
    aligned(8) class FullBox(unsigned int(32) boxtype,
                             unsigned int(8) v,
                             bit(24) f) extends Box(boxtype) {
        unsigned int(8) version = v;
        bit(24) flags = f;
    }
    
    The semantics of these two fields are:
        `version` is an integer that specifies the version of this format of the box.
        `flags` is a map of flags
    
    Boxes with an unrecognized version shall be ignored and skipped.

    简单来说，Box Header 有两个版本的格式:
    
    pub struct Box {
        type: BoxType,
        size: u32,
        // if size == 0 {
        //   box extends to end of file
        // } else if size == 1 {
        //   largesize = u64
        // }
        largesize: Option<u64>,
        usertype: Option<Vec<u8>> // length 16. if type === 'uuid', usertype is active.
    }

    pub struct FullBox {
        type: BoxType,
        size: u32,
        // if size == 0 {
        //   box extends to end of file
        // } else if size == 1 {
        //   largesize = u64
        // }
        largesize: Option<u64>,
        // if type === 'uuid', then usertype is active
        usertype: Option<[u8; 16]>, // length 16.
        
        version: u8,
        flags  : [u8; 3],   // 24 Bits
    }
**/

#[derive(Debug, Clone)]
pub struct Header {
    size       : u32,
    kind       : Kind, // atom type
    
    // Optional
    largesize  : Option<u64>,
    usertype   : Option<[u8; 16]>,
    version    : Option<u8>,
    flags      : Option<[u8; 3]>, // 24 Bits
    // 自定义抽象
    atom_size  : u64,  // atom size , include header and data.
    header_size: u64,  // atom header size, not include data size.
    data_size  : u64,  // atom data size , not include header size.
    offset     : u64,  // file offset.
}

impl Header {
    pub fn parse(f: &mut Mp4File) -> Result<Header, &'static str>{
        let curr_offset = f.offset();
        let size: u32 = f.read_u32().unwrap();

        let kind_bytes: [u8; 4] = [
            f.read_u8().unwrap(), f.read_u8().unwrap(),
            f.read_u8().unwrap(), f.read_u8().unwrap(),
        ];
        let kind = Kind::from_bytes(&kind_bytes).unwrap();

        let header_size = 8u64;
        let atom_size = size as u64;
        let data_size = atom_size - header_size;

        f.offset_inc(header_size);

        let mut header = Header{
            size: size,
            kind: kind,

            largesize  : None,
            usertype   : None,
            version    : None,
            flags      : None,

            atom_size  : atom_size,    // atom size , include header and data.
            header_size: header_size,  // atom header size, not include data size.
            data_size  : data_size,    // atom data size , not include header size.
            offset     : curr_offset,  // file offset.
        };
        if size == 1u32 {
            header.parse_largesize(f);
        } else if size < 1u32 {
            return Err("can not parse this mp4 file.");
        }
        Ok(header)
    }
    pub fn parse_largesize(&mut self, f: &mut Mp4File){
        assert_eq!(self.size, 1u32);

        let largesize = f.read_u64().unwrap();
        self.atom_size = largesize;
        self.header_size = self.header_size + 8;
        self.data_size = largesize - self.header_size;

        self.largesize = Some(largesize);
        f.offset_inc(8);
    }
    pub fn parse_usertype(&mut self, f: &mut Mp4File){
        let usertype: [u8; 16] = [
            f.read_u8().unwrap(), f.read_u8().unwrap(),
            f.read_u8().unwrap(), f.read_u8().unwrap(),
            f.read_u8().unwrap(), f.read_u8().unwrap(),
            f.read_u8().unwrap(), f.read_u8().unwrap(),
            f.read_u8().unwrap(), f.read_u8().unwrap(),
            f.read_u8().unwrap(), f.read_u8().unwrap(),
            f.read_u8().unwrap(), f.read_u8().unwrap(),
            f.read_u8().unwrap(), f.read_u8().unwrap()
        ];
        self.usertype = Some(usertype);

        self.header_size = self.header_size + 16;
        assert!((self.atom_size - self.header_size) >= 0);
        self.data_size = self.atom_size - self.header_size;
        f.offset_inc(16);
    }
    pub fn parse_version(&mut self, f: &mut Mp4File){
        let version = f.read_u8().unwrap();
        self.version = Some(version);

        self.header_size = self.header_size + 1;
        assert!((self.atom_size - self.header_size) >= 0);
        self.data_size = self.atom_size - self.header_size;
        f.offset_inc(1);
    }
    pub fn parse_flags(&mut self, f: &mut Mp4File){
        let flags: [u8; 3] = [
            f.read_u8().unwrap(), f.read_u8().unwrap(), 
            f.read_u8().unwrap()
        ];
        self.flags = Some(flags);

        self.header_size = self.header_size + 3;
        assert!((self.atom_size - self.header_size) >= 0);
        self.data_size = self.atom_size - self.header_size;
        f.offset_inc(3);
    }
}

#[derive(Debug, Clone)]
pub enum Atom {
    ftyp(Ftyp),
    free(Free),
    skip(Skip),
    mdat(Mdat),
    pdin(Pdin),
    uuid(Uuid),
    // MOOV
    moov(Moov),
    mvhd(Mvhd),
    trak(Trak),
    tkhd(Tkhd),
    tref(Tref),
    mdia(Mdia),
    mdhd(Mdhd),
    hdlr(Hdlr),
    minf(Minf),
    vmhd(Vmhd),
    smhd(Smhd),
    hmhd(Hmhd),
    nmhd(Nmhd),

    mvex(Mvex),
    mehd(Mehd),
    trex(Trex),
    
    // STBL
    stbl(Stbl),
    stsc(Stsc),
    stsz(Stsz),
    stz2(Stz2),
    stco(Stco),
    co64(Co64),
    stsd(Stsd),
    stdp(Stdp),
    stts(Stts),
    ctts(Ctts),
    cslg(Cslg),
    stss(Stss),
    stsh(Stsh),
    sdtp(Sdtp),
    padb(Padb),

    // MOOF
    moof(Moof),
    mfhd(Mfhd),
    traf(Traf),
    tfhd(Tfhd),
    trun(Trun),
    // MFRA
    mfra(Mfra),
    tfra(Tfra),
    mfro(Mfro),
    
    ignore(Ignore),
    unrecognized(Unrecognized)
}

impl Atom {
    fn parse_kind(f: &mut Mp4File) -> Result<Kind, &'static str> {
        let kind_bytes: [u8; 4] = [
            f.read_u8().unwrap(), f.read_u8().unwrap(),
            f.read_u8().unwrap(), f.read_u8().unwrap(),
        ];
        Kind::from_bytes(&kind_bytes)
    }
    
    pub fn parse(f: &mut Mp4File) -> Result<Self, &'static str> {
        let mut header = Header::parse(f).unwrap();
        // println!("DO: \n{:?}", header);
        let data = match header.kind {
            // Kind::bxml => ,
            Kind::co64 => Ok(Atom::co64(Co64::parse(f, header).unwrap())),
            Kind::cslg => Ok(Atom::cslg(Cslg::parse(f, header).unwrap())),
            // Kind::cprt => ,
            Kind::ctts => Ok(Atom::ctts(Ctts::parse(f, header).unwrap())),
            // Kind::dinf => ,
            // Kind::dref => ,
            // Kind::edts => ,
            // Kind::elst => ,
            // Kind::fecr => ,
            // Kind::fiin => ,
            // Kind::fpar => ,
            Kind::free => Ok(Atom::free(Free::parse(f, header).unwrap())),
            // Kind::frma => ,
            Kind::ftyp => Ok(Atom::ftyp(Ftyp::parse(f, header).unwrap())),
            Kind::hdlr => Ok(Atom::hdlr(Hdlr::parse(f, header).unwrap())),
            Kind::hmhd => Ok(Atom::hmhd(Hmhd::parse(f, header).unwrap())),
            // Kind::iinf => ,
            // Kind::iloc => ,
            // Kind::imif => ,
            // Kind::ipmc => ,
            // Kind::ipro => ,
            // Kind::itn  => ,
            Kind::mdat => Ok(Atom::mdat(Mdat::parse(f, header).unwrap())),
            Kind::mdhd => Ok(Atom::mdhd(Mdhd::parse(f, header).unwrap())),
            Kind::mdia => Ok(Atom::mdia(Mdia::parse(f, header).unwrap())),
            // Kind::meco => ,
            Kind::mehd => Ok(Atom::mehd(Mehd::parse(f, header).unwrap())),
            // Kind::mere => ,
            // Kind::meta => ,
            Kind::mfhd => Ok(Atom::mfhd(Mfhd::parse(f, header).unwrap())),
            Kind::mfra => Ok(Atom::mfra(Mfra::parse(f, header).unwrap())),
            Kind::mfro => Ok(Atom::mfro(Mfro::parse(f, header).unwrap())),
            Kind::minf => Ok(Atom::minf(Minf::parse(f, header).unwrap())),
            Kind::moof => Ok(Atom::moof(Moof::parse(f, header).unwrap())),
            Kind::moov => Ok(Atom::moov(Moov::parse(f, header).unwrap())),
            Kind::mvex => Ok(Atom::mvex(Mvex::parse(f, header).unwrap())),
            Kind::mvhd => Ok(Atom::mvhd(Mvhd::parse(f, header).unwrap())),
            Kind::nmhd => Ok(Atom::nmhd(Nmhd::parse(f, header).unwrap())),
            Kind::padb => Ok(Atom::padb(Padb::parse(f, header).unwrap())),
            // Kind::paen => ,
            Kind::pdin => Ok(Atom::pdin(Pdin::parse(f, header).unwrap())),
            // Kind::pitm => ,
            // Kind::sbgp => ,
            // Kind::schi => ,
            // Kind::schm => ,
            Kind::sdtp => Ok(Atom::sdtp(Sdtp::parse(f, header).unwrap())),
            // Kind::sgpd => ,
            // Kind::sinf => ,
            Kind::skip => Ok(Atom::skip(Skip::parse(f, header).unwrap())),
            Kind::smhd => Ok(Atom::smhd(Smhd::parse(f, header).unwrap())),
            Kind::stbl => Ok(Atom::stbl(Stbl::parse(f, header).unwrap())),
            Kind::stco => Ok(Atom::stco(Stco::parse(f, header).unwrap())),
            Kind::stdp => Ok(Atom::stdp(Stdp::parse(f, header).unwrap())),
            Kind::stsc => Ok(Atom::stsc(Stsc::parse(f, header).unwrap())),
            Kind::stsd => Ok(Atom::stsd(Stsd::parse(f, header).unwrap())),
            Kind::stsh => Ok(Atom::stsh(Stsh::parse(f, header).unwrap())),
            Kind::stss => Ok(Atom::stss(Stss::parse(f, header).unwrap())),
            Kind::stsz => Ok(Atom::stsz(Stsz::parse(f, header).unwrap())),
            Kind::stts => Ok(Atom::stts(Stts::parse(f, header).unwrap())),
            Kind::stz2 => Ok(Atom::stz2(Stz2::parse(f, header).unwrap())),
            // Kind::subs => ,
            Kind::tfhd => Ok(Atom::tfhd(Tfhd::parse(f, header).unwrap())),
            Kind::tfra => Ok(Atom::tfra(Tfra::parse(f, header).unwrap())),
            Kind::tkhd => Ok(Atom::tkhd(Tkhd::parse(f, header).unwrap())),
            Kind::traf => Ok(Atom::traf(Traf::parse(f, header).unwrap())),
            Kind::trak => Ok(Atom::trak(Trak::parse(f, header).unwrap())),
            Kind::tref => Ok(Atom::tref(Tref::parse(f, header).unwrap())),
            Kind::trex => Ok(Atom::trex(Trex::parse(f, header).unwrap())),
            Kind::trun => Ok(Atom::trun(Trun::parse(f, header).unwrap())),
            // Kind::tsel => ,
            // Kind::udta => ,
            Kind::uuid => Ok(Atom::uuid(Uuid::parse(f, header).unwrap())),
            Kind::vmhd => Ok(Atom::vmhd(Vmhd::parse(f, header).unwrap())),
            // Kind::xml  => ,
            // Kind::strk => ,
            // Kind::stri => ,
            // Kind::strd => 

            Kind::Unrecognized(_) => Ok(Atom::unrecognized(Unrecognized::parse(f, header).unwrap())),
            _ => Ok(Atom::ignore(Ignore::parse(f, header).unwrap()))
        };
        data
    }
    pub fn parse_children(f: &mut Mp4File) -> Vec<Atom> {
        let mut atoms: Vec<Atom> = Vec::new();
        loop {
            if f.offset() == f.file_size() {
                break;
            }
            match Atom::parse(f) {
                Ok(atom) => {
                    atoms.push(atom);
                },
                Err(e) => {
                    println!("[ERROR] ATOM parse error ({:?})", e);
                    break;
                }
            }
        }
        atoms
    }
}

