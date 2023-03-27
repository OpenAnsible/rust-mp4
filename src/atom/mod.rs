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

pub use super::Mp4File;
use byteorder::ReadBytesExt;

mod kind;

mod freespace;
mod ftyp;
mod ignore;
mod mdat;
mod meco;
mod meta;
mod mfra;
mod moof;
mod moov;
mod pdin;
mod unrecognized;
mod uuid;

pub use self::kind::Kind;

use self::freespace::{Free, Skip};
use self::ftyp::Ftyp;
use self::mdat::Mdat;
use self::pdin::Pdin;
use self::uuid::Uuid;

use self::ignore::Ignore;
use self::meco::{Meco, Mere};
use self::meta::{Bxml, Meta, Xml};
use self::mfra::{Mfra, Mfro, Tfra};
use self::moof::{Mfhd, Moof, Tfhd, Traf, Trun};
use self::moov::{
    Co64, Cslg, Ctts, Hdlr, Hmhd, Mdhd, Mdia, Mehd, Minf, Moov, Mvex, Mvhd, Nmhd, Padb, Sdtp, Smhd,
    Stbl, Stco, Stdp, Stsc, Stsd, Stsh, Stss, Stsz, Stts, Stz2, Tkhd, Trak, Tref, Trex, Vmhd,
};
use self::unrecognized::Unrecognized;

#[derive(Debug, Clone)]
pub struct Entry {
    pub first_chunk: u32,
    pub samples_per_chunk: u32,
    pub sample_description_index: u32,
}

#[derive(Debug, Clone)]
pub struct Sample {
    pub duration: Option<u32>,
    pub size: Option<u32>,
    pub flags: Option<u32>,
    pub composition_time_offset: Option<i32>,
    pub description_index: Option<u32>,
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
    pub size: u32,
    pub kind: Kind, // atom type

    // Optional
    pub largesize: Option<u64>,
    pub usertype: Option<[u8; 16]>,
    pub version: Option<u8>,
    pub flags: Option<[u8; 3]>, // 24 Bits
    // custom abstraction
    pub atom_size: u64,   // atom size , include header and data.
    pub header_size: u64, // atom header size, not include data size.
    pub data_size: u64,   // atom data size , not include header size.
    pub offset: u64,      // file offset.
}

impl Header {
    pub fn parse(f: &mut Mp4File) -> Result<Self, &'static str> {
        let curr_offset = f.offset();
        let size: u32 = f.read_u32().unwrap();

        let kind_bytes: [u8; 4] = [
            f.read_u8().unwrap(),
            f.read_u8().unwrap(),
            f.read_u8().unwrap(),
            f.read_u8().unwrap(),
        ];
        let kind = Kind::from_bytes(&kind_bytes).unwrap();

        let header_size = 8u64;
        let atom_size = u64::from(size);
        // let data_size = atom_size - header_size;
        let data_size = 0u64;

        f.offset_inc(header_size);

        let mut header = Self {
            size,
            kind,

            largesize: None,
            usertype: None,
            version: None,
            flags: None,

            atom_size,           // atom size , include header and data.
            header_size,         // atom header size, not include data size.
            data_size,           // atom data size , not include header size.
            offset: curr_offset, // file offset.
        };
        if size == 1u32 {
            header.parse_largesize(f);
        } else if size < 1u32 {
            return Err("can not parse this mp4 file.");
        } else {
            header.data_size = atom_size - header_size;
        }
        Ok(header)
    }

    pub fn parse_largesize(&mut self, f: &mut Mp4File) {
        assert_eq!(self.size, 1u32);

        let largesize = f.read_u64().unwrap();
        self.atom_size = largesize;
        self.header_size += 8;
        self.data_size = largesize - self.header_size;

        self.largesize = Some(largesize);
        f.offset_inc(8);
    }

    pub fn parse_usertype(&mut self, f: &mut Mp4File) {
        let usertype: [u8; 16] = [
            f.read_u8().unwrap(),
            f.read_u8().unwrap(),
            f.read_u8().unwrap(),
            f.read_u8().unwrap(),
            f.read_u8().unwrap(),
            f.read_u8().unwrap(),
            f.read_u8().unwrap(),
            f.read_u8().unwrap(),
            f.read_u8().unwrap(),
            f.read_u8().unwrap(),
            f.read_u8().unwrap(),
            f.read_u8().unwrap(),
            f.read_u8().unwrap(),
            f.read_u8().unwrap(),
            f.read_u8().unwrap(),
            f.read_u8().unwrap(),
        ];
        self.usertype = Some(usertype);

        self.header_size += 16;
        self.data_size = self.atom_size - self.header_size;
        f.offset_inc(16);
    }

    pub fn parse_version(&mut self, f: &mut Mp4File) {
        let version = f.read_u8().unwrap();
        self.version = Some(version);

        self.header_size += 1;
        self.data_size = self.atom_size - self.header_size;
        f.offset_inc(1);
    }

    pub fn parse_flags(&mut self, f: &mut Mp4File) {
        let flags: [u8; 3] = [
            f.read_u8().unwrap(),
            f.read_u8().unwrap(),
            f.read_u8().unwrap(),
        ];
        self.flags = Some(flags);

        self.header_size += 3;
        self.data_size = self.atom_size - self.header_size;
        f.offset_inc(3);
    }
}

#[derive(Debug, Clone)]
pub enum Atom {
    Ftyp(Ftyp),
    Free(Free),
    Skip(Skip),
    Mdat(Mdat),
    Pdin(Pdin),
    Uuid(Uuid),
    // MOOV
    Moov(Moov),
    Mvhd(Mvhd),
    Trak(Trak),
    Tkhd(Tkhd),
    Tref(Tref),
    Mdia(Mdia),
    Mdhd(Mdhd),
    Hdlr(Hdlr),
    Minf(Minf),
    Vmhd(Vmhd),
    Smhd(Smhd),
    Hmhd(Hmhd),
    Mmhd(Nmhd),

    Mvex(Mvex),
    Mehd(Mehd),
    Trex(Trex),

    // STBL
    Stbl(Stbl),
    Stsc(Stsc),
    Stsz(Stsz),
    Stz2(Stz2),
    Stco(Stco),
    Co64(Co64),
    Stsd(Stsd),
    Stdp(Stdp),
    Stts(Stts),
    Ctts(Ctts),
    Cslg(Cslg),
    Stss(Stss),
    Stsh(Stsh),
    Sdtp(Sdtp),
    Padb(Padb),

    // MOOF
    Moof(Moof),
    Mfhd(Mfhd),
    Traf(Traf),
    Tfhd(Tfhd),
    Trun(Trun),
    // MFRA
    Mfra(Mfra),
    Tfra(Tfra),
    Mfro(Mfro),
    // Meta
    Meta(Meta),
    Xml(Xml),
    Bxml(Bxml),
    // Meco
    Meco(Meco),
    Mere(Mere),

    Ignore(Ignore),
    Unrecognized(Unrecognized),
}

impl Atom {
    fn parse_kind(f: &mut Mp4File) -> Result<Kind, &'static str> {
        let kind_bytes: [u8; 4] = [
            f.read_u8().unwrap(),
            f.read_u8().unwrap(),
            f.read_u8().unwrap(),
            f.read_u8().unwrap(),
        ];
        Kind::from_bytes(&kind_bytes)
    }

    pub fn parse(f: &mut Mp4File) -> Result<Self, &'static str> {
        let header = Header::parse(f).unwrap();

        let data = match header.kind {
            Kind::Bxml => Ok(Self::Bxml(Bxml::parse(f, header).unwrap())),
            Kind::Co64 => Ok(Self::Co64(Co64::parse(f, header).unwrap())),
            Kind::Cslg => Ok(Self::Cslg(Cslg::parse(f, header).unwrap())),
            // Kind::cprt => ,
            Kind::Ctts => Ok(Self::Ctts(Ctts::parse(f, header).unwrap())),
            // Kind::dinf => ,
            // Kind::dref => ,
            // Kind::edts => ,
            // Kind::elst => ,
            // Kind::fecr => ,
            // Kind::fiin => ,
            // Kind::fpar => ,
            Kind::Free => Ok(Self::Free(Free::parse(f, header))),
            // Kind::frma => ,
            Kind::Ftyp => Ok(Self::Ftyp(Ftyp::parse(f, header).unwrap())),
            Kind::Hdlr => Ok(Self::Hdlr(Hdlr::parse(f, header).unwrap())),
            Kind::Hmhd => Ok(Self::Hmhd(Hmhd::parse(f, header).unwrap())),
            // Kind::iinf => ,
            // Kind::iloc => ,
            // Kind::imif => ,
            // Kind::ipmc => ,
            // Kind::ipro => ,
            // Kind::itn  => ,
            Kind::Mdat => Ok(Self::Mdat(Mdat::parse(f, header).unwrap())),
            Kind::Mdhd => Ok(Self::Mdhd(Mdhd::parse(f, header).unwrap())),
            Kind::Mdia => Ok(Self::Mdia(Mdia::parse(f, header).unwrap())),
            Kind::Meco => Ok(Self::Meco(Meco::parse(f, header))),
            Kind::Mehd => Ok(Self::Mehd(Mehd::parse(f, header).unwrap())),
            Kind::Mere => Ok(Self::Mere(Mere::parse(f, header).unwrap())),
            Kind::Meta => Ok(Self::Meta(Meta::parse(f, header).unwrap())),
            Kind::Mfhd => Ok(Self::Mfhd(Mfhd::parse(f, header).unwrap())),
            Kind::Mfra => Ok(Self::Mfra(Mfra::parse(f, header).unwrap())),
            Kind::Mfro => Ok(Self::Mfro(Mfro::parse(f, header).unwrap())),
            Kind::Minf => Ok(Self::Minf(Minf::parse(f, header).unwrap())),
            Kind::Moof => Ok(Self::Moof(Moof::parse(f, header).unwrap())),
            Kind::Moov => Ok(Self::Moov(Moov::parse(f, header).unwrap())),
            Kind::Mvex => Ok(Self::Mvex(Mvex::parse(f, header).unwrap())),
            Kind::Mvhd => Ok(Self::Mvhd(Mvhd::parse(f, header).unwrap())),
            Kind::Mmhd => Ok(Self::Mmhd(Nmhd::parse(f, header).unwrap())),
            Kind::Padb => Ok(Self::Padb(Padb::parse(f, header).unwrap())),
            // Kind::paen => ,
            Kind::Pdin => Ok(Self::Pdin(Pdin::parse(f, header).unwrap())),
            // Kind::pitm => ,
            // Kind::sbgp => ,
            // Kind::schi => ,
            // Kind::schm => ,
            Kind::Sdtp => Ok(Self::Sdtp(Sdtp::parse(f, header).unwrap())),
            // Kind::sgpd => ,
            // Kind::sinf => ,
            Kind::Skip => Ok(Self::Skip(Skip::parse(f, header))),
            Kind::Smhd => Ok(Self::Smhd(Smhd::parse(f, header).unwrap())),
            Kind::Stbl => Ok(Self::Stbl(Stbl::parse(f, header).unwrap())),
            Kind::Stco => Ok(Self::Stco(Stco::parse(f, header).unwrap())),
            Kind::Stdp => Ok(Self::Stdp(Stdp::parse(f, header).unwrap())),
            Kind::Stsc => Ok(Self::Stsc(Stsc::parse(f, header).unwrap())),
            Kind::Stsd => Ok(Self::Stsd(Stsd::parse(f, header).unwrap())),
            Kind::Stsh => Ok(Self::Stsh(Stsh::parse(f, header).unwrap())),
            Kind::Stss => Ok(Self::Stss(Stss::parse(f, header).unwrap())),
            Kind::Stsz => Ok(Self::Stsz(Stsz::parse(f, header).unwrap())),
            Kind::Stts => Ok(Self::Stts(Stts::parse(f, header).unwrap())),
            Kind::Stz2 => Ok(Self::Stz2(Stz2::parse(f, header).unwrap())),
            // Kind::subs => ,
            Kind::Tfhd => Ok(Self::Tfhd(Tfhd::parse(f, header).unwrap())),
            Kind::Tfra => Ok(Self::Tfra(Tfra::parse(f, header).unwrap())),
            Kind::Tkhd => Ok(Self::Tkhd(Tkhd::parse(f, header).unwrap())),
            Kind::Traf => Ok(Self::Traf(Traf::parse(f, header).unwrap())),
            Kind::Trak => Ok(Self::Trak(Trak::parse(f, header).unwrap())),
            Kind::Tref => Ok(Self::Tref(Tref::parse(f, header).unwrap())),
            Kind::Trex => Ok(Self::Trex(Trex::parse(f, header).unwrap())),
            Kind::Trun => Ok(Self::Trun(Trun::parse(f, header).unwrap())),
            // Kind::tsel => ,
            // Kind::udta => ,
            Kind::Uuid => Ok(Self::Uuid(Uuid::parse(f, header).unwrap())),
            Kind::Vmhd => Ok(Self::Vmhd(Vmhd::parse(f, header).unwrap())),
            Kind::Xml => Ok(Self::Xml(Xml::parse(f, header).unwrap())),
            // Kind::strk => ,
            // Kind::stri => ,
            // Kind::strd =>
            Kind::Unrecognized(_) => {
                Ok(Self::Unrecognized(Unrecognized::parse(f, header).unwrap()))
            }
            _ => Ok(Self::Ignore(Ignore::parse(f, header).unwrap())),
        };
        data
    }
    pub fn parse_children(f: &mut Mp4File) -> Vec<Self> {
        let mut atoms: Vec<Self> = Vec::new();
        loop {
            if f.offset() == f.file_size() {
                break;
            }
            match Self::parse(f) {
                Ok(atom) => {
                    atoms.push(atom);
                }
                Err(e) => {
                    println!("[ERROR] ATOM parse error ({e:?})");
                    break;
                }
            }
        }
        atoms
    }
}
