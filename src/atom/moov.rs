

// Metadata container

/**

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
**/

use super::{Mp4File, Kind, Header, Atom};

pub struct Moov {

}

/**
aligned(8) class MovieHeaderBox extends FullBox(‘mvhd’, version, 0) { if (version==1) {
      unsigned int(64)  creation_time;
      unsigned int(64)  modification_time;
      unsigned int(32)  timescale;
      unsigned int(64)  duration;
   } else { // version==0
      unsigned int(32)  creation_time;
      unsigned int(32)  modification_time;
      unsigned int(32)  timescale;
      unsigned int(32)  duration;
}
template int(32) rate = 0x00010000; // typically 1.0
template int(16) volume = 0x0100; // typically, full volume const bit(16) reserved = 0;
const unsigned int(32)[2] reserved = 0;
template int(32)[9] matrix =
{ 0x00010000,0,0,0,0x00010000,0,0,0,0x40000000 };
      // Unity matrix
   bit(32)[6]  pre_defined = 0;
   unsigned int(32)  next_track_ID;
}
**/
pub struct mvhd {
    version: u8,
    creation_time: u64,
    modification_time: u64,
    timescale: u32,
    duration: u64,

    rate: u32,   // 0x00010000
    volume: u16, // 0x0100
    
}
