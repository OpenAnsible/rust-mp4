use std::str;
use std::str::FromStr;
use std::string::ToString;

#[allow(clippy::doc_markdown)]
/**
Atom Types:
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


[
    'ftyp', 'pdin', 'moov', 'mvhd', 'trak', 'tkhd', 'mdia',
    'mdhd', 'hdlr', 'minf', 'stbl', 'stsd', 'stts', 'stsc',
    'stsz', 'stz2', 'stss', 'stco', 'co64', 'ctts', 'stsh',
    'padb', 'stdp', 'sdtp', 'sbgp', 'sgpd', 'subs', 'dinf',
    'dref', 'nmhd', 'hmhd', 'smhd', 'vmhd', 'tref', 'edts',
    'elst', 'mvex', 'mehd', 'trex', 'ipmc', 'moof', 'mfhd',
    'traf', 'tfhd', 'trun', 'sdtp', 'sbgp', 'subs', 'mfra',
    'tfra', 'mfro', 'mdat', 'free', 'skip', 'udta', 'cprt',
    'meta', 'hdlr', 'dinf', 'dref', 'ipmc', 'iloc', 'ipro',
    'sinf', 'frma', 'imif', 'schm', 'schi', 'iinf', 'xml',
    'bxml', 'pitm', 'fiin', 'paen', 'fpar', 'fecr', 'itn',
    'tsel', 'meco', 'mere', 'strk', 'stri', 'strd',
]

atoms.sort()
atoms = list(set(atoms))

xml, itn

Python Script:
sss = []
for atom in atoms:
    if len(atom) == 4:
        sss.append("\"%s\" => Ok(Self::%s)," % (atom, atom) )
    elif len(atom) == 3:
        sss.append("\"%s\" | \"%s\\u{0}\" => Ok(Self::%s)," % (atom, atom, atom) )
    else:
        print "error"
print "\n".join(sss)


sss = []
for atom in atoms:
    if len(atom) == 4:
        sss.append("Self::%s => \"%s\".to_owned()," % (atom, atom) )
    elif len(atom) == 3:
        sss.append("Self::%s => \"%s\u{0}\".to_owned()," % (atom, atom) )
    else:
        print "error"
print "\n".join(sss)


Container atom types:

    dinf
    edts
    ipro
    mdia
    meta
    mfra
    minf
    moof
    moov
    mvex
    sinf
    skip
    stbl
    traf
    trak

**/

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

impl ToString for Kind {
    fn to_string(&self) -> String {
        match *self {
            Self::Bxml => "bxml".to_owned(),
            Self::Co64 => "co64".to_owned(),
            Self::Cprt => "cprt".to_owned(),
            Self::Ctts => "ctts".to_owned(),
            Self::Cslg => "cslg".to_owned(),
            Self::Dinf => "dinf".to_owned(),
            Self::Dref => "dref".to_owned(),
            Self::Edts => "edts".to_owned(),
            Self::Elst => "elst".to_owned(),
            Self::Fecr => "fecr".to_owned(),
            Self::Fiin => "fiin".to_owned(),
            Self::Fpar => "fpar".to_owned(),
            Self::Free => "free".to_owned(),
            Self::Frma => "frma".to_owned(),
            Self::Ftyp => "ftyp".to_owned(),
            Self::Hdlr => "hdlr".to_owned(),
            Self::Hmhd => "hmhd".to_owned(),
            Self::Iinf => "iinf".to_owned(),
            Self::Iloc => "iloc".to_owned(),
            Self::Imif => "imif".to_owned(),
            Self::Ipmc => "ipmc".to_owned(),
            Self::Ipro => "ipro".to_owned(),
            Self::Itn => "itn\u{0}".to_owned(),
            Self::Mdat => "mdat".to_owned(),
            Self::Mdhd => "mdhd".to_owned(),
            Self::Mdia => "mdia".to_owned(),
            Self::Meco => "meco".to_owned(),
            Self::Mehd => "mehd".to_owned(),
            Self::Mere => "mere".to_owned(),
            Self::Meta => "meta".to_owned(),
            Self::Mfhd => "mfhd".to_owned(),
            Self::Mfra => "mfra".to_owned(),
            Self::Mfro => "mfro".to_owned(),
            Self::Minf => "minf".to_owned(),
            Self::Moof => "moof".to_owned(),
            Self::Moov => "moov".to_owned(),
            Self::Mvex => "mvex".to_owned(),
            Self::Mvhd => "mvhd".to_owned(),
            Self::Mmhd => "nmhd".to_owned(),
            Self::Padb => "padb".to_owned(),
            Self::Paen => "paen".to_owned(),
            Self::Pdin => "pdin".to_owned(),
            Self::Pitm => "pitm".to_owned(),
            Self::Sbgp => "sbgp".to_owned(),
            Self::Schi => "schi".to_owned(),
            Self::Schm => "schm".to_owned(),
            Self::Sdtp => "sdtp".to_owned(),
            Self::Sgpd => "sgpd".to_owned(),
            Self::Sinf => "sinf".to_owned(),
            Self::Skip => "skip".to_owned(),
            Self::Smhd => "smhd".to_owned(),
            Self::Stbl => "stbl".to_owned(),
            Self::Stco => "stco".to_owned(),
            Self::Stdp => "stdp".to_owned(),
            Self::Stsc => "stsc".to_owned(),
            Self::Stsd => "stsd".to_owned(),
            Self::Stsh => "stsh".to_owned(),
            Self::Stss => "stss".to_owned(),
            Self::Stsz => "stsz".to_owned(),
            Self::Stts => "stts".to_owned(),
            Self::Stz2 => "stz2".to_owned(),
            Self::Subs => "subs".to_owned(),
            Self::Tfhd => "tfhd".to_owned(),
            Self::Tfra => "tfra".to_owned(),
            Self::Tkhd => "tkhd".to_owned(),
            Self::Traf => "traf".to_owned(),
            Self::Trak => "trak".to_owned(),
            Self::Tref => "tref".to_owned(),
            Self::Trex => "trex".to_owned(),
            Self::Trun => "trun".to_owned(),
            Self::Tsel => "tsel".to_owned(),
            Self::Udta => "udta".to_owned(),
            Self::Uuid => "uuid".to_owned(),
            Self::Vmhd => "vmhd".to_owned(),
            Self::Xml => "xml\u{0}".to_owned(),
            Self::Strk => "strk".to_owned(),
            Self::Stri => "stri".to_owned(),
            Self::Strd => "strd".to_owned(),
            Self::Unrecognized(ref s) => s.clone(),
        }
    }
}

impl Kind {
    pub fn from_bytes(bytes: &[u8; 4]) -> Result<Self, &'static str> {
        let kind_str = match str::from_utf8(bytes) {
            Ok(s) => s,
            Err(_) => {
                println!("Atom Kind ({bytes:?}) parse error.");
                return Err("Atom Kind parse error.");
            }
        };
        Self::from_str(kind_str)
    }
    #[must_use] pub fn into_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}
