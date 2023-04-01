// https://wiki.multimedia.cx/index.php/QuickTime_container
// http://developer.apple.com/documentation/QuickTime/QTFF/index.html
// http://www.adobe.com/devnet/video/articles/mp4_movie_atom.html
// http://mpeg.chiariglione.org/standards/mpeg-4/mpeg-4.htm

// http://www.adobe.com/devnet/f4v.html

// box types: http://mp4ra.org/atoms.html

use std::cmp::Ordering;
/**
    Box Struct:

        size(u32), type(u32), largesize(u64),
        data

Among them, `size` specifies the size of the entire `box`, including the `header` part.
If the `box` size exceeds the maximum value of `u32`, `size` is set to `1`,
And use the next `8-bit` u64 to store the size.

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

use crate::let_ok;

pub use super::Mp4File;
// use byteorder::ReadBytesExt;

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
    /// Parses a file and reads the header information
    ///
    /// # Arguments
    ///
    /// `f: &mut Mp4File` -- The MP4 file to be read.
    ///
    /// # Returns
    ///
    ///
    ///
    /// # Errors
    ///
    ///
    ///
    /// # Panics
    ///
    ///
    ///
    /// # Examples
    ///
    ///
    ///
    pub fn parse(f: &mut Mp4File) -> Result<Self, &'static str> {
        let curr_offset = f.offset();
        let_ok!(size, f.read_u32(), "Unable to read size.");

        let kind_bytes: [u8; 4] = [
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
        ];

        let_ok!(
            kind,
            Kind::from_bytes(kind_bytes),
            "Unable to read file kind."
        );

        let header_size = 8u64;
        let atom_size = u64::from(size);
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

        match size.cmp(&1u32) {
            Ordering::Equal => header.parse_largesize(f),
            Ordering::Greater => header.data_size = atom_size - header_size,
            Ordering::Less => return Err("Cannot parse this mp4 file."),
        }

        Ok(header)
    }

    /// Parses the largesize part of the MP4 file
    ///
    /// # Arguments
    ///
    /// `f: &mut Mp4File` -- The file to be parsed.
    ///
    /// # Returns
    ///
    /// Nothing.
    ///
    /// # Errors
    ///
    /// None.
    ///
    /// # Panics
    ///
    /// If `self.size != 1`. If unable to read the `largesize`;
    ///
    /// # Examples
    ///
    ///
    ///
    pub fn parse_largesize(&mut self, f: &mut Mp4File) {
        assert_eq!(self.size, 1u32);

        let largesize = f.read_u64().expect("Unable to read largesize.");
        self.atom_size = largesize;
        self.header_size += 8;
        self.data_size = largesize - self.header_size;

        self.largesize = Some(largesize);
        f.offset_inc(8);
    }

    pub fn parse_usertype(&mut self, f: &mut Mp4File) {
        let usertype: [u8; 16] = [
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
        ];
        self.usertype = Some(usertype);

        self.header_size += 16;
        self.data_size = self.atom_size - self.header_size;
        f.offset_inc(16);
    }

    pub fn parse_version(&mut self, f: &mut Mp4File) {
        let version = f.read_u8().expect("Unable to read version information.");
        self.version = Some(version);

        self.header_size += 1;
        self.data_size = self.atom_size - self.header_size;
        f.offset_inc(1);
    }

    pub fn parse_flags(&mut self, f: &mut Mp4File) {
        let flags: [u8; 3] = [
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
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
    #[allow(dead_code)]
    fn parse_kind(f: &mut Mp4File) -> Result<Kind, &'static str> {
        let kind_bytes: [u8; 4] = [
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
        ];
        Kind::from_bytes(kind_bytes)
    }

    /// Parse the `Atom` from a file
    ///
    /// # Arguments
    ///
    /// `f: &mut Mp4File` -- The file to be parsed
    ///
    /// # Returns
    ///
    ///
    ///
    /// # Errors
    ///
    /// If unable to read the Header, we return an error.
    ///
    /// # Panics
    ///
    /// If any of the detailed parsing fails, it bails.
    ///
    /// # Examples
    ///
    ///
    ///
    // TODO: Return a proper Result here, so we can stop panicing and handle things properly.
    #[allow(clippy::too_many_lines)]
    pub fn parse(f: &mut Mp4File) -> Result<Self, &'static str> {
        let_ok!(header, Header::parse(f), "Unable to read Atom header.");

        let data = match header.kind {
            Kind::Bxml => Ok(Self::Bxml(
                Bxml::parse(f, header).expect("Unable to parse Bxml"),
            )),
            Kind::Co64 => Ok(Self::Co64(
                Co64::parse(f, header).expect("Unable to parse Kind::Co64"),
            )),
            Kind::Cslg => Ok(Self::Cslg(Cslg::parse(f, header))),
            // Kind::cprt => ,
            Kind::Ctts => Ok(Self::Ctts(
                Ctts::parse(f, header).expect("Unable to parse Kind::Ctts"),
            )),
            // Kind::dinf => ,
            // Kind::dref => ,
            // Kind::edts => ,
            // Kind::elst => ,
            // Kind::fecr => ,
            // Kind::fiin => ,
            // Kind::fpar => ,
            Kind::Free => Ok(Self::Free(Free::parse(f, header))),
            // Kind::frma => ,
            Kind::Ftyp => Ok(Self::Ftyp(
                Ftyp::parse(f, header).expect("Unable to parse Kind::Ftyp"),
            )),
            Kind::Hdlr => Ok(Self::Hdlr(
                Hdlr::parse(f, header).expect("Unable to parse Kind::Hdlr"),
            )),
            Kind::Hmhd => Ok(Self::Hmhd(
                Hmhd::parse(f, header).expect("Unable to parse Kind::Hmhd"),
            )),
            // Kind::iinf => ,
            // Kind::iloc => ,
            // Kind::imif => ,
            // Kind::ipmc => ,
            // Kind::ipro => ,
            // Kind::itn  => ,
            Kind::Mdat => Ok(Self::Mdat(
                Mdat::parse(f, header).expect("Unable to parse Kind::Mdat"),
            )),
            Kind::Mdhd => Ok(Self::Mdhd(
                Mdhd::parse(f, header).expect("Unable to parse Kind::Mdhd"),
            )),
            Kind::Mdia => Ok(Self::Mdia(Mdia::parse(f, header))),
            Kind::Meco => Ok(Self::Meco(Meco::parse(f, header))),
            Kind::Mehd => Ok(Self::Mehd(
                Mehd::parse(f, header).expect("Unable to parse Kind::Mehd"),
            )),
            Kind::Mere => Ok(Self::Mere(
                Mere::parse(f, header).expect("Unable to parse Kind::Mere"),
            )),
            Kind::Meta => Ok(Self::Meta(
                Meta::parse(f, header).expect("Unable to parse Kind::Meta"),
            )),
            Kind::Mfhd => Ok(Self::Mfhd(
                Mfhd::parse(f, header).expect("Unable to parse Kind::Mfhd"),
            )),
            Kind::Mfra => Ok(Self::Mfra(Mfra::parse(f, header))),
            Kind::Mfro => Ok(Self::Mfro(
                Mfro::parse(f, header).expect("Unable to parse Kind::Mfro"),
            )),
            Kind::Minf => Ok(Self::Minf(Minf::parse(f, header))),
            Kind::Moof => Ok(Self::Moof(Moof::parse(f, header))),
            Kind::Moov => Ok(Self::Moov(Moov::parse(f, header))),
            Kind::Mvex => Ok(Self::Mvex(Mvex::parse(f, header))),
            Kind::Mvhd => Ok(Self::Mvhd(
                Mvhd::parse(f, header).expect("Unable to parse Kind::Mvhd"),
            )),
            Kind::Mmhd => Ok(Self::Mmhd(Nmhd::parse(f, header))),
            Kind::Padb => Ok(Self::Padb(
                Padb::parse(f, header).expect("Unable to parse Kind::Padb"),
            )),
            // Kind::paen => ,
            Kind::Pdin => Ok(Self::Pdin(
                Pdin::parse(f, header).expect("Unable to parse Kind::Pdin"),
            )),
            // Kind::pitm => ,
            // Kind::sbgp => ,
            // Kind::schi => ,
            // Kind::schm => ,
            Kind::Sdtp => Ok(Self::Sdtp(Sdtp::parse(f, header))),
            // Kind::sgpd => ,
            // Kind::sinf => ,
            Kind::Skip => Ok(Self::Skip(Skip::parse(f, header))),
            Kind::Smhd => Ok(Self::Smhd(
                Smhd::parse(f, header).expect("Unable to parse Kind::Smhd"),
            )),
            Kind::Stbl => Ok(Self::Stbl(Stbl::parse(f, header))),
            Kind::Stco => Ok(Self::Stco(
                Stco::parse(f, header).expect("Unable to parse Kind::Stco"),
            )),
            Kind::Stdp => Ok(Self::Stdp(Stdp::parse(f, header))),
            Kind::Stsc => Ok(Self::Stsc(
                Stsc::parse(f, header).expect("Unable to parse Kind::Stsc"),
            )),
            Kind::Stsd => Ok(Self::Stsd(Stsd::parse(f, header))),
            Kind::Stsh => Ok(Self::Stsh(Stsh::parse(f, header))),
            Kind::Stss => Ok(Self::Stss(Stss::parse(f, header))),
            Kind::Stsz => Ok(Self::Stsz(
                Stsz::parse(f, header).expect("Unable to parse Kind::Stsz"),
            )),
            Kind::Stts => Ok(Self::Stts(
                Stts::parse(f, header).expect("Unable to parse Kind::Stts"),
            )),
            Kind::Stz2 => Ok(Self::Stz2(
                Stz2::parse(f, header).expect("Unable to parse Kind::Stz2"),
            )),
            // Kind::subs => ,
            Kind::Tfhd => Ok(Self::Tfhd(
                Tfhd::parse(f, header).expect("Unable to parse Kind::Tfhd"),
            )),
            Kind::Tfra => Ok(Self::Tfra(
                Tfra::parse(f, header).expect("Unable to parse Kind::Tfra"),
            )),
            Kind::Tkhd => Ok(Self::Tkhd(Tkhd::parse(f, header))),
            Kind::Traf => Ok(Self::Traf(Traf::parse(f, header))),
            Kind::Trak => Ok(Self::Trak(Trak::parse(f, header))),
            Kind::Tref => Ok(Self::Tref(Tref::parse(f, header))),
            Kind::Trex => Ok(Self::Trex(Trex::parse(f, header))),
            Kind::Trun => Ok(Self::Trun(
                Trun::parse(f, header).expect("Unable to parse Kind::Trun"),
            )),
            // Kind::tsel => ,
            // Kind::udta => ,
            Kind::Uuid => Ok(Self::Uuid(Uuid::parse(f, header))),
            Kind::Vmhd => Ok(Self::Vmhd(
                Vmhd::parse(f, header).expect("Unable to parse Kind::Vmhd"),
            )),
            Kind::Xml => Ok(Self::Xml(
                Xml::parse(f, header).expect("Unable to parse Kind::Xml"),
            )),
            // Kind::strk => ,
            // Kind::stri => ,
            // Kind::strd =>
            Kind::Unrecognized(_) => Ok(Self::Unrecognized(Unrecognized::parse(f, header))),
            _ => Ok(Self::Ignore(
                Ignore::parse(f, header).expect("Unable to parse Kind::Ignore"),
            )),
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
