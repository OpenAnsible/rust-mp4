
use std::str;
use std::str::FromStr;
use std::string::ToString;
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
        sss.append("\"%s\" => Ok(Kind::%s)," % (atom, atom) )
    elif len(atom) == 3:
        sss.append("\"%s\" | \"%s\\u{0}\" => Ok(Kind::%s)," % (atom, atom, atom) )
    else:
        print "error"
print "\n".join(sss)


sss = []
for atom in atoms:
    if len(atom) == 4:
        sss.append("Kind::%s => \"%s\".to_owned()," % (atom, atom) )
    elif len(atom) == 3:
        sss.append("Kind::%s => \"%s\u{0}\".to_owned()," % (atom, atom) )
    else:
        print "error"
print "\n".join(sss)
**/

#[derive(Debug, Clone)]
pub enum Kind{
    bxml,
    co64,
    cprt,
    ctts,
    dinf,
    dref,
    edts,
    elst,
    fecr,
    fiin,
    fpar,
    free,
    frma,
    ftyp,
    hdlr,
    hmhd,
    iinf,
    iloc,
    imif,
    ipmc,
    ipro,
    itn,
    mdat,
    mdhd,
    mdia,
    meco,
    mehd,
    mere,
    meta,
    mfhd,
    mfra,
    mfro,
    minf,
    moof,
    moov,
    mvex,
    mvhd,
    nmhd,
    padb,
    paen,
    pdin,
    pitm,
    sbgp,
    schi,
    schm,
    sdtp,
    sgpd,
    sinf,
    skip,
    smhd,
    stbl,
    stco,
    stdp,
    stsc,
    stsd,
    stsh,
    stss,
    stsz,
    stts,
    stz2,
    subs,
    tfhd,
    tfra,
    tkhd,
    traf,
    trak,
    tref,
    trex,
    trun,
    tsel,
    udta,
    vmhd,
    xml,
    strk,
    stri,
    strd
}

impl FromStr for Kind {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err>{
        match s {
            "bxml" => Ok(Kind::bxml),
            "co64" => Ok(Kind::co64),
            "cprt" => Ok(Kind::cprt),
            "ctts" => Ok(Kind::ctts),
            "dinf" => Ok(Kind::dinf),
            "dref" => Ok(Kind::dref),
            "edts" => Ok(Kind::edts),
            "elst" => Ok(Kind::elst),
            "fecr" => Ok(Kind::fecr),
            "fiin" => Ok(Kind::fiin),
            "fpar" => Ok(Kind::fpar),
            "free" => Ok(Kind::free),
            "frma" => Ok(Kind::frma),
            "ftyp" => Ok(Kind::ftyp),
            "hdlr" => Ok(Kind::hdlr),
            "hmhd" => Ok(Kind::hmhd),
            "iinf" => Ok(Kind::iinf),
            "iloc" => Ok(Kind::iloc),
            "imif" => Ok(Kind::imif),
            "ipmc" => Ok(Kind::ipmc),
            "ipro" => Ok(Kind::ipro),
            "itn" | "itn\u{0}" => Ok(Kind::itn),
            "mdat" => Ok(Kind::mdat),
            "mdhd" => Ok(Kind::mdhd),
            "mdia" => Ok(Kind::mdia),
            "meco" => Ok(Kind::meco),
            "mehd" => Ok(Kind::mehd),
            "mere" => Ok(Kind::mere),
            "meta" => Ok(Kind::meta),
            "mfhd" => Ok(Kind::mfhd),
            "mfra" => Ok(Kind::mfra),
            "mfro" => Ok(Kind::mfro),
            "minf" => Ok(Kind::minf),
            "moof" => Ok(Kind::moof),
            "moov" => Ok(Kind::moov),
            "mvex" => Ok(Kind::mvex),
            "mvhd" => Ok(Kind::mvhd),
            "nmhd" => Ok(Kind::nmhd),
            "padb" => Ok(Kind::padb),
            "paen" => Ok(Kind::paen),
            "pdin" => Ok(Kind::pdin),
            "pitm" => Ok(Kind::pitm),
            "sbgp" => Ok(Kind::sbgp),
            "schi" => Ok(Kind::schi),
            "schm" => Ok(Kind::schm),
            "sdtp" => Ok(Kind::sdtp),
            "sgpd" => Ok(Kind::sgpd),
            "sinf" => Ok(Kind::sinf),
            "skip" => Ok(Kind::skip),
            "smhd" => Ok(Kind::smhd),
            "stbl" => Ok(Kind::stbl),
            "stco" => Ok(Kind::stco),
            "stdp" => Ok(Kind::stdp),
            "stsc" => Ok(Kind::stsc),
            "stsd" => Ok(Kind::stsd),
            "stsh" => Ok(Kind::stsh),
            "stss" => Ok(Kind::stss),
            "stsz" => Ok(Kind::stsz),
            "stts" => Ok(Kind::stts),
            "stz2" => Ok(Kind::stz2),
            "subs" => Ok(Kind::subs),
            "tfhd" => Ok(Kind::tfhd),
            "tfra" => Ok(Kind::tfra),
            "tkhd" => Ok(Kind::tkhd),
            "traf" => Ok(Kind::traf),
            "trak" => Ok(Kind::trak),
            "tref" => Ok(Kind::tref),
            "trex" => Ok(Kind::trex),
            "trun" => Ok(Kind::trun),
            "tsel" => Ok(Kind::tsel),
            "udta" => Ok(Kind::udta),
            "vmhd" => Ok(Kind::vmhd),
            "xml" | "xml\u{0}" => Ok(Kind::xml),
            "strk" => Ok(Kind::strk),
            "stri" => Ok(Kind::stri),
            "strd" => Ok(Kind::strd),
            _ => {
                println!("Unknow Kind: {:?}", s);
                Err("unknow Kind")
            }
        }
    }
}

impl ToString for Kind {
    fn to_string(&self) -> String {
        match *self {
            Kind::bxml => "bxml".to_owned(),
            Kind::co64 => "co64".to_owned(),
            Kind::cprt => "cprt".to_owned(),
            Kind::ctts => "ctts".to_owned(),
            Kind::dinf => "dinf".to_owned(),
            Kind::dref => "dref".to_owned(),
            Kind::edts => "edts".to_owned(),
            Kind::elst => "elst".to_owned(),
            Kind::fecr => "fecr".to_owned(),
            Kind::fiin => "fiin".to_owned(),
            Kind::fpar => "fpar".to_owned(),
            Kind::free => "free".to_owned(),
            Kind::frma => "frma".to_owned(),
            Kind::ftyp => "ftyp".to_owned(),
            Kind::hdlr => "hdlr".to_owned(),
            Kind::hmhd => "hmhd".to_owned(),
            Kind::iinf => "iinf".to_owned(),
            Kind::iloc => "iloc".to_owned(),
            Kind::imif => "imif".to_owned(),
            Kind::ipmc => "ipmc".to_owned(),
            Kind::ipro => "ipro".to_owned(),
            Kind::itn  => "itn\u{0}".to_owned(),
            Kind::mdat => "mdat".to_owned(),
            Kind::mdhd => "mdhd".to_owned(),
            Kind::mdia => "mdia".to_owned(),
            Kind::meco => "meco".to_owned(),
            Kind::mehd => "mehd".to_owned(),
            Kind::mere => "mere".to_owned(),
            Kind::meta => "meta".to_owned(),
            Kind::mfhd => "mfhd".to_owned(),
            Kind::mfra => "mfra".to_owned(),
            Kind::mfro => "mfro".to_owned(),
            Kind::minf => "minf".to_owned(),
            Kind::moof => "moof".to_owned(),
            Kind::moov => "moov".to_owned(),
            Kind::mvex => "mvex".to_owned(),
            Kind::mvhd => "mvhd".to_owned(),
            Kind::nmhd => "nmhd".to_owned(),
            Kind::padb => "padb".to_owned(),
            Kind::paen => "paen".to_owned(),
            Kind::pdin => "pdin".to_owned(),
            Kind::pitm => "pitm".to_owned(),
            Kind::sbgp => "sbgp".to_owned(),
            Kind::schi => "schi".to_owned(),
            Kind::schm => "schm".to_owned(),
            Kind::sdtp => "sdtp".to_owned(),
            Kind::sgpd => "sgpd".to_owned(),
            Kind::sinf => "sinf".to_owned(),
            Kind::skip => "skip".to_owned(),
            Kind::smhd => "smhd".to_owned(),
            Kind::stbl => "stbl".to_owned(),
            Kind::stco => "stco".to_owned(),
            Kind::stdp => "stdp".to_owned(),
            Kind::stsc => "stsc".to_owned(),
            Kind::stsd => "stsd".to_owned(),
            Kind::stsh => "stsh".to_owned(),
            Kind::stss => "stss".to_owned(),
            Kind::stsz => "stsz".to_owned(),
            Kind::stts => "stts".to_owned(),
            Kind::stz2 => "stz2".to_owned(),
            Kind::subs => "subs".to_owned(),
            Kind::tfhd => "tfhd".to_owned(),
            Kind::tfra => "tfra".to_owned(),
            Kind::tkhd => "tkhd".to_owned(),
            Kind::traf => "traf".to_owned(),
            Kind::trak => "trak".to_owned(),
            Kind::tref => "tref".to_owned(),
            Kind::trex => "trex".to_owned(),
            Kind::trun => "trun".to_owned(),
            Kind::tsel => "tsel".to_owned(),
            Kind::udta => "udta".to_owned(),
            Kind::vmhd => "vmhd".to_owned(),
            Kind::xml  => "xml\u{0}".to_owned(),
            Kind::strk => "strk".to_owned(),
            Kind::stri => "stri".to_owned(),
            Kind::strd => "strd".to_owned()
        }
    }
}

impl Kind {
    pub fn from_bytes(bytes: &[u8; 4]) -> Result<Self, &'static str> {
        let kind_str = match str::from_utf8(bytes) {
            Ok(s)  => s,
            Err(_) => {
                println!("Atom Kind ({:?}) parse error.", bytes);
                return Err("Atom Kind parse error.");
            }
        };
        Kind::from_str(kind_str)
    }
    pub fn into_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}