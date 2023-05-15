//! Defines the `Atom` type, which is the main type for parsing MP4 files.

use crate::let_ok;
use crate::mp4file::Mp4File;
use std::str;

// These are all used in the Atom enum below.
use super::bxml::Bxml;
use super::co64::Co64;
use super::cslg::Cslg;
use super::ctts::Ctts;
use super::freespace::{Free, Skip};
use super::ftyp::Ftyp;
use super::hdlr::Hdlr;
use super::header::Header;
use super::hmhd::Hmhd;
use super::ignore::Ignore;
use super::kind::Kind;
use super::mdat::Mdat;
use super::mdhd::Mdhd;
use super::mdia::Mdia;
use super::meco::Meco;
use super::mehd::Mehd;
use super::mere::Mere;
use super::meta::Meta;
use super::mfhd::Mfhd;
use super::mfra::Mfra;
use super::mfro::Mfro;
use super::minf::Minf;
use super::moof::Moof;
use super::moov::Moov;
use super::mvex::Mvex;
use super::mvhd::Mvhd;
use super::nmhd::Nmhd;
use super::padb::Padb;
use super::pdin::Pdin;
use super::sbgp::Sbgp;
use super::sdtp::Sdtp;
use super::smhd::Smhd;
use super::stbl::Stbl;
use super::stco::Stco;
use super::stdp::Stdp;
use super::stsc::Stsc;
use super::stsd::Stsd;
use super::stsh::Stsh;
use super::stss::Stss;
use super::stsz::Stsz;
use super::stts::Stts;
use super::stz2::Stz2;
use super::tfhd::Tfhd;
use super::tfra::Tfra;
use super::tkhd::Tkhd;
use super::traf::Traf;
use super::trak::Trak;
use super::tref::Tref;
use super::trex::Trex;
use super::trgr::Trgr;
use super::trun::Trun;
use super::udta::Udta;
use super::unrecognized::Unrecognized;
use super::uuid::Uuid;
use super::vmhd::Vmhd;
use super::xml::Xml;

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
    Trgr(Trgr),

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
    Sbgp(Sbgp),
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

    // Udta
    Udta(Udta),

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
                Bxml::parse(f, header).expect("Unable to parse Kind::Bxml"),
            )),
            Kind::Co64 => Ok(Self::Co64(
                Co64::parse(f, header).expect("Unable to parse Kind::Co64"),
            )),
            Kind::Cslg => Ok(Self::Cslg(
                Cslg::parse(f, header).expect("Unable to parse Kind::Cslg"),
            )),
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
            Kind::Sbgp => Ok(Self::Sbgp(
                Sbgp::parse(f, header).expect("Unable to parse Kind::Sbgp"),
            )),
            // Kind::schi => ,
            // Kind::schm => ,
            Kind::Sdtp => Ok(Self::Sdtp(
                Sdtp::parse(f, header).expect("Unable to parse Kind::Sdtp"),
            )),
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
            Kind::Stdp => Ok(Self::Stdp(
                Stdp::parse(f, header).expect("Unable to parse Kind::Stdp"),
            )),
            Kind::Stsc => Ok(Self::Stsc(
                Stsc::parse(f, header).expect("Unable to parse Kind::Stsc"),
            )),
            Kind::Stsd => Ok(Self::Stsd(
                Stsd::parse(f, header).expect("Unable to parse Kind::Stsd"),
            )),
            Kind::Stsh => Ok(Self::Stsh(
                Stsh::parse(f, header).expect("Unable to parse Kind::Stsh"),
            )),
            Kind::Stss => Ok(Self::Stss(
                Stss::parse(f, header).expect("Unable to parse Kind::Stss"),
            )),
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
            Kind::Tkhd => Ok(Self::Tkhd(
                Tkhd::parse(f, header).expect("Unable to parse Kind::Tkhd"),
            )),
            Kind::Traf => Ok(Self::Traf(Traf::parse(f, header))),
            Kind::Trak => Ok(Self::Trak(Trak::parse(f, header))),
            Kind::Tref => Ok(Self::Tref(
                Tref::parse(f, header).expect("Unable to parse Kind::Tref"),
            )),
            Kind::Trgr => Ok(Self::Trgr(
                Trgr::parse(f, header).expect("Unable to parse Kind::Trgr"),
            )),
            Kind::Trex => Ok(Self::Trex(
                Trex::parse(f, header).expect("Unable to parse Kind::Trex"),
            )),
            Kind::Trun => Ok(Self::Trun(
                Trun::parse(f, header).expect("Unable to parse Kind::Trun"),
            )),
            // Kind::tsel => ,
            Kind::Udta => Ok(Self::Udta(Udta::parse(f, header))),
            Kind::Uuid => Ok(Self::Uuid(
                Uuid::parse(f, header).expect("Unable to parse Kind::Uuid"),
            )),
            Kind::Vmhd => Ok(Self::Vmhd(
                Vmhd::parse(f, header).expect("Unable to parse Kind::Vmhd"),
            )),
            Kind::Xml => Ok(Self::Xml(
                Xml::parse(f, header).expect("Unable to parse Kind::Xml"),
            )),
            // Kind::strk => ,
            // Kind::stri => ,
            // Kind::strd =>
            Kind::Unrecognized(_) => Ok(Self::Unrecognized(
                Unrecognized::parse(f, header).expect("Unable to parse Kind::Unrecognized"),
            )),
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
