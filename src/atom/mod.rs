//! Defines the `Atom` type, which is the main type for parsing MP4 files.

use std::str;

use crate::let_ok;
use crate::mp4file::Mp4File;

mod entry;
mod sample;

pub mod bxml;
pub mod freespace;
pub mod ftyp;
pub mod header;
pub mod ignore;
pub mod kind;
pub mod mdat;
pub mod meco;
pub mod mehd;
pub mod mere;
pub mod meta;
pub mod mfhd;
pub mod mfra;
pub mod mfro;
pub mod moof;
pub mod moov;
pub mod mvex;
pub mod pdin;
pub mod sdtp;
pub mod stdp;
pub mod tfhd;
pub mod tfra;
pub mod traf;
pub mod trex;
pub mod trun;
pub mod unrecognized;
pub mod uuid;
pub mod xml;

// These are all used in the Atom enum below.
use self::bxml::Bxml;
use self::freespace::{Free, Skip};
use self::ftyp::Ftyp;
use self::header::Header;
use self::ignore::Ignore;
use self::kind::Kind;
use self::mdat::Mdat;
use self::meco::Meco;
use self::mehd::Mehd;
use self::mere::Mere;
use self::meta::Meta;
use self::mfhd::Mfhd;
use self::mfra::Mfra;
use self::mfro::Mfro;
use self::moof::Moof;
use self::moov::{
    Co64, Cslg, Ctts, Hdlr, Hmhd, Mdhd, Mdia, Minf, Moov, Mvhd, Nmhd, Padb, Smhd, Stbl, Stco, Stsc,
    Stsd, Stsh, Stss, Stsz, Stts, Stz2, Tkhd, Trak, Tref, Vmhd,
};
use self::mvex::Mvex;
use self::pdin::Pdin;
use self::sdtp::Sdtp;
use self::stdp::Stdp;
use self::tfhd::Tfhd;
use self::tfra::Tfra;
use self::traf::Traf;
use self::trex::Trex;
use self::trun::Trun;
use self::unrecognized::Unrecognized;
use self::uuid::Uuid;
use self::xml::Xml;

/// An atom is a container for data in an MP4 file.
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

        match header.kind {
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
        }
    }

    /// Parse the children of an atom.
    /// This will parse all of the children of the atom, until the end of the file.
    ///
    /// # Arguments
    ///
    /// `f: &mut Mp4File` -- The file to be parsed.
    ///
    /// # Returns
    ///
    /// A vector of `Atom`s that are the children of the atom.
    ///
    /// # Errors
    ///
    /// None.
    ///
    /// # Panics
    ///
    /// None.
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
                    log::trace!("Atom::parse_children - parse error: ({e:?}), file: {f:?}");
                    break;
                }
            }
        }
        atoms
    }
}
