//! Defines the `Kind` enum, which represents the different types of atoms that can be in an MP4 file.

use std::str;
use std::str::FromStr;
use std::string::ToString;

use crate::let_ok;

/// Represents the different types of atoms that can be in an MP4 file.
/// This is used to determine which atom to parse.
#[allow(clippy::doc_markdown)]
#[derive(Debug, Clone)]
pub enum Kind {
    Bxml,
    Co64,
    Cprt,
    Ctts,
    Cslg,
    Dinf,
    Dref,
    Edts,
    Elst,
    Fecr,
    Fiin,
    Fpar,
    Free,
    Frma,
    Ftyp,
    Hdlr,
    Hmhd,
    Iinf,
    Iloc,
    Imif,
    Ipmc,
    Ipro,
    Itn,
    Mdat,
    Mdhd,
    Mdia,
    Meco,
    Mehd,
    Mere,
    Meta,
    Mfhd,
    Mfra,
    Mfro,
    Minf,
    Moof,
    Moov,
    Mvex,
    Mvhd,
    Mmhd,
    Padb,
    Paen,
    Pdin,
    Pitm,
    Sbgp,
    Schi,
    Schm,
    Sdtp,
    Sgpd,
    Sinf,
    Skip,
    Smhd,
    Stbl,
    Stco,
    Stdp,
    Stsc,
    Stsd,
    Stsh,
    Stss,
    Stsz,
    Stts,
    Stz2,
    Subs,
    Tfhd,
    Tfra,
    Tkhd,
    Traf,
    Trak,
    Tref,
    Trex,
    Trun,
    Tsel,
    Udta,
    Uuid,
    Vmhd,
    Xml,
    Strk,
    Stri,
    Strd,
    Unrecognized(String),
}

impl FromStr for Kind {
    type Err = &'static str;

    /// Converts a string to an `Kind` enum.
    ///
    /// # Arguments
    ///
    /// * `s: &str` - The string to convert.
    ///
    /// # Returns
    ///
    /// * `Result<Self, Self::Err>` - The result of the conversion.
    ///
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bxml" => Ok(Self::Bxml),
            "co64" => Ok(Self::Co64),
            "cprt" => Ok(Self::Cprt),
            "ctts" => Ok(Self::Ctts),
            "cslg" => Ok(Self::Cslg),
            "dinf" => Ok(Self::Dinf),
            "dref" => Ok(Self::Dref),
            "edts" => Ok(Self::Edts),
            "elst" => Ok(Self::Elst),
            "fecr" => Ok(Self::Fecr),
            "fiin" => Ok(Self::Fiin),
            "fpar" => Ok(Self::Fpar),
            "free" => Ok(Self::Free),
            "frma" => Ok(Self::Frma),
            "ftyp" => Ok(Self::Ftyp),
            "hdlr" => Ok(Self::Hdlr),
            "hmhd" => Ok(Self::Hmhd),
            "iinf" => Ok(Self::Iinf),
            "iloc" => Ok(Self::Iloc),
            "imif" => Ok(Self::Imif),
            "ipmc" => Ok(Self::Ipmc),
            "ipro" => Ok(Self::Ipro),
            "itn" | "itn\u{0}" => Ok(Self::Itn),
            "mdat" => Ok(Self::Mdat),
            "mdhd" => Ok(Self::Mdhd),
            "mdia" => Ok(Self::Mdia),
            "meco" => Ok(Self::Meco),
            "mehd" => Ok(Self::Mehd),
            "mere" => Ok(Self::Mere),
            "meta" => Ok(Self::Meta),
            "mfhd" => Ok(Self::Mfhd),
            "mfra" => Ok(Self::Mfra),
            "mfro" => Ok(Self::Mfro),
            "minf" => Ok(Self::Minf),
            "moof" => Ok(Self::Moof),
            "moov" => Ok(Self::Moov),
            "mvex" => Ok(Self::Mvex),
            "mvhd" => Ok(Self::Mvhd),
            "nmhd" => Ok(Self::Mmhd),
            "padb" => Ok(Self::Padb),
            "paen" => Ok(Self::Paen),
            "pdin" => Ok(Self::Pdin),
            "pitm" => Ok(Self::Pitm),
            "sbgp" => Ok(Self::Sbgp),
            "schi" => Ok(Self::Schi),
            "schm" => Ok(Self::Schm),
            "sdtp" => Ok(Self::Sdtp),
            "sgpd" => Ok(Self::Sgpd),
            "sinf" => Ok(Self::Sinf),
            "skip" => Ok(Self::Skip),
            "smhd" => Ok(Self::Smhd),
            "stbl" => Ok(Self::Stbl),
            "stco" => Ok(Self::Stco),
            "stdp" => Ok(Self::Stdp),
            "stsc" => Ok(Self::Stsc),
            "stsd" => Ok(Self::Stsd),
            "stsh" => Ok(Self::Stsh),
            "stss" => Ok(Self::Stss),
            "stsz" => Ok(Self::Stsz),
            "stts" => Ok(Self::Stts),
            "stz2" => Ok(Self::Stz2),
            "subs" => Ok(Self::Subs),
            "tfhd" => Ok(Self::Tfhd),
            "tfra" => Ok(Self::Tfra),
            "tkhd" => Ok(Self::Tkhd),
            "traf" => Ok(Self::Traf),
            "trak" => Ok(Self::Trak),
            "tref" => Ok(Self::Tref),
            "trex" => Ok(Self::Trex),
            "trun" => Ok(Self::Trun),
            "tsel" => Ok(Self::Tsel),
            "udta" => Ok(Self::Udta),
            "uuid" => Ok(Self::Uuid),
            "vmhd" => Ok(Self::Vmhd),
            "xml" | "xml\u{0}" => Ok(Self::Xml),
            "strk" => Ok(Self::Strk),
            "stri" => Ok(Self::Stri),
            "strd" => Ok(Self::Strd),
            _ => Ok(Self::Unrecognized(s.to_owned())),
        }
    }
}

/// Implements the `Display` trait for the `Kind` enum.
impl std::fmt::Display for Kind {
    /// Converts the `Kind` enum to a string for display.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Bxml => "Bxml",
            Self::Co64 => "Co64",
            Self::Cprt => "Cprt",
            Self::Ctts => "Ctts",
            Self::Cslg => "Cslg",
            Self::Dinf => "Dinf",
            Self::Dref => "Dref",
            Self::Edts => "Edts",
            Self::Elst => "Elst",
            Self::Fecr => "Fecr",
            Self::Fiin => "Fiin",
            Self::Fpar => "Fpar",
            Self::Free => "Free",
            Self::Frma => "Frma",
            Self::Ftyp => "Ftyp",
            Self::Hdlr => "Hdlr",
            Self::Hmhd => "Hmhd",
            Self::Iinf => "Iinf",
            Self::Iloc => "Iloc",
            Self::Imif => "Imif",
            Self::Ipmc => "Ipmc",
            Self::Ipro => "Ipro",
            Self::Itn => "Itn",
            Self::Mdat => "Mdat",
            Self::Mdhd => "Mdhd",
            Self::Mdia => "Mdia",
            Self::Meco => "Meco",
            Self::Mehd => "Mehd",
            Self::Mere => "Mere",
            Self::Meta => "Meta",
            Self::Mfhd => "Mfhd",
            Self::Mfra => "Mfra",
            Self::Mfro => "Mfro",
            Self::Minf => "Minf",
            Self::Moof => "Moof",
            Self::Moov => "Moov",
            Self::Mvex => "Mvex",
            Self::Mvhd => "Mvhd",
            Self::Mmhd => "Mmhd",
            Self::Padb => "Padb",
            Self::Paen => "Paen",
            Self::Pdin => "Pdin",
            Self::Pitm => "Pitm",
            Self::Sbgp => "Sbgp",
            Self::Schi => "Schi",
            Self::Schm => "Schm",
            Self::Sdtp => "Sdtp",
            Self::Sgpd => "Sgpd",
            Self::Sinf => "Sinf",
            Self::Skip => "Skip",
            Self::Smhd => "Smhd",
            Self::Stbl => "Stbl",
            Self::Stco => "Stco",
            Self::Stdp => "Stdp",
            Self::Stsc => "Stsc",
            Self::Stsd => "Stsd",
            Self::Stsh => "Stsh",
            Self::Stss => "Stss",
            Self::Stsz => "Stsz",
            Self::Stts => "Stts",
            Self::Stz2 => "Stz2",
            Self::Subs => "Subs",
            Self::Tfhd => "Tfhd",
            Self::Tfra => "Tfra",
            Self::Tkhd => "Tkhd",
            Self::Traf => "Traf",
            Self::Trak => "Trak",
            Self::Tref => "Tref",
            Self::Trex => "Trex",
            Self::Trun => "Trun",
            Self::Tsel => "Tsel",
            Self::Udta => "Udta",
            Self::Uuid => "Uuid",
            Self::Vmhd => "Vmhd",
            Self::Xml => "Xml",
            Self::Strk => "Strk",
            Self::Stri => "Stri",
            Self::Strd => "Strd",
            Self::Unrecognized(_) => "Unrecognized",
        };
        write!(f, "{s}")
    }
}

impl Kind {
    /// Returns the file type based on the bytes supplied.
    ///
    /// # Arguments
    ///
    /// - `bytes: [u8; 4]` -- a 4-byte array containing the file identifier
    ///
    /// # Returns
    ///
    /// * `Result<Self, &'static str>` - The result of the conversion.
    ///
    /// # Errors
    ///
    /// * `Err(&'static str)` - If the file identifier is not recognized.
    pub fn from_bytes(bytes: [u8; 4]) -> Result<Self, &'static str> {
        let_ok!(kind_str, str::from_utf8(&bytes), "Atom Kind parse error.");
        log::trace!("Kind::from_bytes -- kind_str = '{kind_str}'");
        Self::from_str(kind_str)
    }

    /// Returns the byte array representation of the Kind enum.
    ///
    /// # Returns
    ///
    /// * `Vec<u8>` - The byte array representation of the Kind enum.
    #[must_use]
    pub fn into_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}
