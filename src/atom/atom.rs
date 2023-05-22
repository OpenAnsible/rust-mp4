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
use super::fire::Fire;
use super::fpar::Fpar;
use super::freespace::{Free, Skip};
use super::frma::Frma;
use super::ftyp::Ftyp;
use super::gitn::Gitn;
use super::hdlr::Hdlr;
use super::header::Header;
use super::hmhd::Hmhd;
use super::ignore::Ignore;
use super::iinf::Iinf;
use super::iloc::Iloc;
use super::imif::Imif;
use super::infe::Infe;
use super::ipmc::Ipmc;
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
    // Container
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
    Fire(Fire), // Meta
    Fpar(Fpar), // Meta
    Free(Free),
    Frma(Frma), // Sinf, Rinf, Cinf
    Ftyp(Ftyp),
    Gitn(Gitn), // Meta
    Hdlr(Hdlr), // Moov
    Hmhd(Hmhd), // Moov
    Ignore(Ignore),
    Iinf(Iinf), // Meta
    Iloc(Iloc), // Meta
    Imif(Imif), // Meta
    Infe(Infe), // Meta
    Ipmc(Ipmc), // Meta, Moov
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

        // Create the error message for the match statement.
        let p_err = format!("Unable to parse Kind::{}", header.kind);
        let pe = p_err.as_str();

        match header.kind {
            Kind::Bxml => Ok(Self::Bxml(Bxml::parse(f, header).expect(pe))),
            Kind::Co64 => Ok(Self::Co64(Co64::parse(f, header).expect(pe))),
            Kind::Cslg => Ok(Self::Cslg(Cslg::parse(f, header).expect(pe))),
            Kind::Cprt => Ok(Self::Cprt(Cprt::parse(f, header).expect(pe))),
            Kind::Ctts => Ok(Self::Ctts(Ctts::parse(f, header).expect(pe))),
            Kind::Dinf => Ok(Self::Dinf(Dinf::parse(f, header).expect(pe))),
            Kind::Dref => Ok(Self::Dref(Dref::parse(f, header).expect(pe))),
            Kind::Edts => Ok(Self::Edts(Edts::parse(f, header).expect(pe))),
            Kind::Elst => Ok(Self::Elst(Elst::parse(f, header).expect(pe))),
            Kind::Fdel => Ok(Self::Fdel(Fdel::parse(f, header).expect(pe))),
            Kind::Fecr => Ok(Self::Fecr(Fecr::parse(f, header).expect(pe))),
            Kind::Fiin => Ok(Self::Fiin(Fiin::parse(f, header).expect(pe))),
            Kind::Fire => Ok(Self::Fire(Fire::parse(f, header).expect(pe))),
            Kind::Fpar => Ok(Self::Fpar(Fpar::parse(f, header).expect(pe))),
            Kind::Free => Ok(Self::Free(Free::parse(f, header).expect(pe))),
            Kind::Frma => Ok(Self::Frma(Frma::parse(f, header).expect(pe))),
            Kind::Ftyp => Ok(Self::Ftyp(Ftyp::parse(f, header).expect(pe))),
            Kind::Gitn => Ok(Self::Gitn(Gitn::parse(f, header).expect(pe))),
            Kind::Hdlr => Ok(Self::Hdlr(Hdlr::parse(f, header).expect(pe))),
            Kind::Hmhd => Ok(Self::Hmhd(Hmhd::parse(f, header).expect(pe))),
            Kind::Iinf => Ok(Self::Iinf(Iinf::parse(f, header).expect(pe))),
            Kind::Iloc => Ok(Self::Iloc(Iloc::parse(f, header).expect(pe))),
            Kind::Imif => Ok(Self::Imif(Imif::parse(f, header).expect(pe))),
            Kind::Ipmc => Ok(Self::Ipmc(Ipmc::parse(f, header).expect(pe))),
            Kind::Ipro => Ok(Self::Ipro(Ipro::parse(f, header).expect(pe))),
            Kind::Mdat => Ok(Self::Mdat(Mdat::parse(f, header).expect(pe))),
            Kind::Mdhd => Ok(Self::Mdhd(Mdhd::parse(f, header).expect(pe))),
            Kind::Mdia => Ok(Self::Mdia(Mdia::parse(f, header).expect(pe))),
            Kind::Meco => Ok(Self::Meco(Meco::parse(f, header).expect(pe))),
            Kind::Mehd => Ok(Self::Mehd(Mehd::parse(f, header).expect(pe))),
            Kind::Mere => Ok(Self::Mere(Mere::parse(f, header).expect(pe))),
            Kind::Meta => Ok(Self::Meta(Meta::parse(f, header).expect(pe))),
            Kind::Mfhd => Ok(Self::Mfhd(Mfhd::parse(f, header).expect(pe))),
            Kind::Mfra => Ok(Self::Mfra(Mfra::parse(f, header).expect(pe))),
            Kind::Mfro => Ok(Self::Mfro(Mfro::parse(f, header).expect(pe))),
            Kind::Minf => Ok(Self::Minf(Minf::parse(f, header).expect(pe))),
            Kind::Moof => Ok(Self::Moof(Moof::parse(f, header).expect(pe))),
            Kind::Moov => Ok(Self::Moov(Moov::parse(f, header).expect(pe))),
            Kind::Mvex => Ok(Self::Mvex(Mvex::parse(f, header).expect(pe))),
            Kind::Mvhd => Ok(Self::Mvhd(Mvhd::parse(f, header).expect(pe))),
            Kind::Mmhd => Ok(Self::Mmhd(Nmhd::parse(f, header).expect(pe))),
            Kind::Padb => Ok(Self::Padb(Padb::parse(f, header).expect(pe))),
            // Kind::Paen => ,
            Kind::Pdin => Ok(Self::Pdin(Pdin::parse(f, header).expect(pe))),
            Kind::Pitm => Ok(Self::Pitm(Pitm::parse(f, header).expect(pe))),
            Kind::Sbgp => Ok(Self::Sbgp(Sbgp::parse(f, header).expect(pe))),
            Kind::Schi => Ok(Self::Schi(Schi::parse(f, header).expect(pe))),
            Kind::Schm => Ok(Self::Schm(Schm::parse(f, header).expect(pe))),
            Kind::Sdtp => Ok(Self::Sdtp(Sdtp::parse(f, header).expect(pe))),
            // Kind::Sgpd => ,
            Kind::Sinf => Ok(Self::Sinf(Sinf::parse(f, header).expect(pe))),
            Kind::Skip => Ok(Self::Skip(Skip::parse(f, header).expect(pe))),
            Kind::Smhd => Ok(Self::Smhd(Smhd::parse(f, header).expect(pe))),
            Kind::Stbl => Ok(Self::Stbl(Stbl::parse(f, header).expect(pe))),
            Kind::Stco => Ok(Self::Stco(Stco::parse(f, header).expect(pe))),
            Kind::Stdp => Ok(Self::Stdp(Stdp::parse(f, header).expect(pe))),
            Kind::Stsc => Ok(Self::Stsc(Stsc::parse(f, header).expect(pe))),
            Kind::Stsd => Ok(Self::Stsd(Stsd::parse(f, header).expect(pe))),
            Kind::Stsh => Ok(Self::Stsh(Stsh::parse(f, header).expect(pe))),
            Kind::Stss => Ok(Self::Stss(Stss::parse(f, header).expect(pe))),
            Kind::Stsz => Ok(Self::Stsz(Stsz::parse(f, header).expect(pe))),
            Kind::Stts => Ok(Self::Stts(Stts::parse(f, header).expect(pe))),
            Kind::Stz2 => Ok(Self::Stz2(Stz2::parse(f, header).expect(pe))),
            // Kind::Subs => ,
            Kind::Tfhd => Ok(Self::Tfhd(Tfhd::parse(f, header).expect(pe))),
            Kind::Tfra => Ok(Self::Tfra(Tfra::parse(f, header).expect(pe))),
            Kind::Tkhd => Ok(Self::Tkhd(Tkhd::parse(f, header).expect(pe))),
            Kind::Traf => Ok(Self::Traf(Traf::parse(f, header).expect(pe))),
            Kind::Trak => Ok(Self::Trak(Trak::parse(f, header).expect(pe))),
            Kind::Tref => Ok(Self::Tref(Tref::parse(f, header).expect(pe))),
            Kind::Trgr => Ok(Self::Trgr(Trgr::parse(f, header).expect(pe))),
            Kind::Trex => Ok(Self::Trex(Trex::parse(f, header).expect(pe))),
            Kind::Trun => Ok(Self::Trun(Trun::parse(f, header).expect(pe))),
            Kind::Tsel => Ok(Self::Tsel(Tsel::parse(f, header).expect(pe))),
            Kind::Udta => Ok(Self::Udta(Udta::parse(f, header).expect(pe))),
            Kind::Uuid => Ok(Self::Uuid(Uuid::parse(f, header).expect(pe))),
            Kind::Vmhd => Ok(Self::Vmhd(Vmhd::parse(f, header).expect(pe))),
            Kind::Xml => Ok(Self::Xml(Xml::parse(f, header).expect(pe))),
            // Kind::Strk => ,
            // Kind::Stri => ,
            // Kind::Strd => ,
            Kind::Url => Ok(Self::Url(Url::parse(f, header).expect(pe))),
            Kind::Urn => Ok(Self::Urn(Urn::parse(f, header).expect(pe))),
            Kind::Unrecognized(_) => Ok(Self::Unrecognized(
                Unrecognized::parse(f, header).expect(pe),
            )),
            _ => Ok(Self::Ignore(Ignore::parse(f, header).expect(pe))),
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
    pub fn parse_children(f: &mut Mp4File) -> Result<Vec<Self>, &'static str> {
        let mut atoms: Vec<Self> = Vec::new();
        loop {
            if f.offset() == f.file_size() {
                break;
            }

            let_ok!(atom, Self::parse(f), "Unable to parse atom.");
            atoms.push(atom);
        }
        Ok(atoms)
    }
}
