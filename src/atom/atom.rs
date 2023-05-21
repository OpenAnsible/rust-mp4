//! Defines the `Atom` type, which is the main type for parsing MP4 files.

use crate::let_ok;
use crate::mp4file::Mp4File;
use std::str;

// These are all used in the Atom enum below.
use super::bxml::Bxml;
use super::co64::Co64;
use super::cprt::Cprt;
use super::cslg::Cslg;
use super::ctts::Ctts;
use super::dinf::Dinf;
use super::dref::Dref;
use super::edts::Edts;
use super::elst::Elst;
use super::fdel::Fdel;
use super::fecr::Fecr;
use super::fiin::Fiin;
use super::fpar::Fpar;
use super::freespace::{Free, Skip};
use super::frma::Frma;
use super::ftyp::Ftyp;
use super::hdlr::Hdlr;
use super::header::Header;
use super::hmhd::Hmhd;
use super::ignore::Ignore;
use super::iinf::Iinf;
use super::iloc::Iloc;
use super::infe::Infe;
use super::ipro::Ipro;
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
use super::pitm::Pitm;
use super::sbgp::Sbgp;
use super::schi::Schi;
use super::schm::Schm;
use super::sdtp::Sdtp;
use super::sinf::Sinf;
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
use super::tsel::Tsel;
use super::udta::Udta;
use super::unrecognized::Unrecognized;
use super::url::Url;
use super::urn::Urn;
use super::uuid::Uuid;
use super::vmhd::Vmhd;
use super::xml::Xml;

/// An atom is a container for data in an MP4 file.
#[derive(Debug, Clone)]
pub enum Atom {
    Bxml(Bxml), // Meta
    Co64(Co64), // Stbl
    Cprt(Cprt), // Udta
    Cslg(Cslg), // Stbl
    Ctts(Ctts), // Stbl
    Dinf(Dinf), // Minf
    Dref(Dref), // Minf
    Edts(Edts), // Moov
    Elst(Elst), // Moov
    Fdel(Fdel), // Iinf
    Fecr(Fecr), // Fiin
    Fiin(Fiin), // Iinf
    Fpar(Fpar), // Meta
    Free(Free),
    Frma(Frma), // Sinf, Rinf, Cinf
    Ftyp(Ftyp),
    Hdlr(Hdlr), // Moov
    Hmhd(Hmhd), // Moov
    Ignore(Ignore),
    Iinf(Iinf), // Meta
    Iloc(Iloc), // Meta
    Infe(Infe), // Meta
    Ipro(Ipro), // Meta
    Mdat(Mdat),
    Mdhd(Mdhd), // Moov
    Mdia(Mdia), // Moov
    Meco(Meco), // Meco
    Mehd(Mehd), // Moov
    Mere(Mere), // Meco
    Meta(Meta), // Meta
    Mfhd(Mfhd), // Moof
    Mfra(Mfra), // Mfra
    Mfro(Mfro), // Mfra
    Minf(Minf), // Moov
    Mmhd(Nmhd), // Moov
    Moof(Moof), // Moof
    Moov(Moov), // Moov
    Mvex(Mvex), // Moov
    Mvhd(Mvhd), // Moov
    Padb(Padb), // Stbl
    Pdin(Pdin),
    Pitm(Pitm), // Meta
    Sbgp(Sbgp), // Stbl
    Schi(Schi), // Meta
    Schm(Schm), // Meta
    Sdtp(Sdtp), // Stbl
    Sinf(Sinf), // Meta
    Skip(Skip),
    Smhd(Smhd), // Moov
    Stbl(Stbl), // Stbl
    Stco(Stco), // Stbl
    Stdp(Stdp), // Stbl
    Stsc(Stsc), // Stbl
    Stsd(Stsd), // Stbl
    Stsh(Stsh), // Stbl
    Stss(Stss), // Stbl
    Stsz(Stsz), // Stbl
    Stts(Stts), // Stbl
    Stz2(Stz2), // Stbl
    Tfhd(Tfhd), // Moof
    Tfra(Tfra), // Mfra
    Tkhd(Tkhd), // Moov
    Traf(Traf), // Moof
    Trak(Trak), // Moov
    Tref(Tref), // Moov
    Trex(Trex), // Moov
    Trgr(Trgr), // Moov
    Trun(Trun), // Moof
    Tsel(Tsel), // Udta
    Udta(Udta), // Udta
    Unrecognized(Unrecognized),
    Url(Url), // Minf
    Urn(Urn), // Minf
    Uuid(Uuid),
    Vmhd(Vmhd), // Moov
    Xml(Xml),   // Meta
}

impl Atom {
    #[allow(dead_code)]
    pub fn parse_kind(f: &mut Mp4File) -> Result<Kind, &'static str> {
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
            Kind::Cprt => Ok(Self::Cprt(
                Cprt::parse(f, header).expect("Unable to parse Kind::Cprt"),
            )),
            Kind::Ctts => Ok(Self::Ctts(
                Ctts::parse(f, header).expect("Unable to parse Kind::Ctts"),
            )),
            Kind::Dinf => Ok(Self::Dinf(Dinf::parse(f, header))),
            Kind::Dref => Ok(Self::Dref(
                Dref::parse(f, header).expect("Unable to parse Kind::Dref"),
            )),
            Kind::Edts => Ok(Self::Edts(Edts::parse(f, header))),
            Kind::Elst => Ok(Self::Elst(
                Elst::parse(f, header).expect("Unable to parse Kind::Elst"),
            )),
            Kind::Fdel => Ok(Self::Fdel(
                Fdel::parse(f, header).expect("Unable to parse Kind::Fdel"),
            )),
            Kind::Fecr => Ok(Self::Fecr(
                Fecr::parse(f, header).expect("Unable to parse Kind::Fecr"),
            )),
            Kind::Fiin => Ok(Self::Fiin(
                Fiin::parse(f, header).expect("Unable to parse Kind::Fiin"),
            )),
            Kind::Fpar => Ok(Self::Fpar(
                Fpar::parse(f, header).expect("Unable to parse Kind::Fpar"),
            )),
            Kind::Free => Ok(Self::Free(
                Free::parse(f, header).expect("Unable to parse Kind::Free"),
            )),
            Kind::Frma => Ok(Self::Frma(
                Frma::parse(f, header).expect("Unable to parse Kind::Frma"),
            )),
            Kind::Ftyp => Ok(Self::Ftyp(
                Ftyp::parse(f, header).expect("Unable to parse Kind::Ftyp"),
            )),
            Kind::Hdlr => Ok(Self::Hdlr(
                Hdlr::parse(f, header).expect("Unable to parse Kind::Hdlr"),
            )),
            Kind::Hmhd => Ok(Self::Hmhd(
                Hmhd::parse(f, header).expect("Unable to parse Kind::Hmhd"),
            )),
            Kind::Iinf => Ok(Self::Iinf(
                Iinf::parse(f, header).expect("Unable to parse Kind::Iinf"),
            )),
            Kind::Iloc => Ok(Self::Iloc(
                Iloc::parse(f, header).expect("Unable to parse Kind::Iloc"),
            )),
            // Kind::Imif => ,
            // Kind::Ipmc => ,
            Kind::Ipro => Ok(Self::Ipro(
                Ipro::parse(f, header).expect("Unable to parse Kind::Ipro"),
            )),
            // Kind::Itn  => ,
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
            // Kind::Paen => ,
            Kind::Pdin => Ok(Self::Pdin(
                Pdin::parse(f, header).expect("Unable to parse Kind::Pdin"),
            )),
            Kind::Pitm => Ok(Self::Pitm(
                Pitm::parse(f, header).expect("Unable to parse Kind::Pitm"),
            )),
            Kind::Sbgp => Ok(Self::Sbgp(
                Sbgp::parse(f, header).expect("Unable to parse Kind::Sbgp"),
            )),
            Kind::Schi => Ok(Self::Schi(Schi::parse(f, header))),
            Kind::Schm => Ok(Self::Schm(
                Schm::parse(f, header).expect("Unable to parse Kind::Schm"),
            )),
            Kind::Sdtp => Ok(Self::Sdtp(
                Sdtp::parse(f, header).expect("Unable to parse Kind::Sdtp"),
            )),
            // Kind::Sgpd => ,
            Kind::Sinf => Ok(Self::Sinf(Sinf::parse(f, header))),
            Kind::Skip => Ok(Self::Skip(
                Skip::parse(f, header).expect("Unable to parse Kind::Skip"),
            )),
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
            // Kind::Subs => ,
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
            Kind::Tsel => Ok(Self::Tsel(
                Tsel::parse(f, header).expect("Unable to parse Kind::Tsel"),
            )),
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
            // Kind::Strk => ,
            // Kind::Stri => ,
            // Kind::Strd => ,
            Kind::Url => Ok(Self::Url(
                Url::parse(f, header).expect("Unable to parse Kind::Url"),
            )),
            Kind::Urn => Ok(Self::Urn(
                Urn::parse(f, header).expect("Unable to parse Kind::Urn"),
            )),
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
