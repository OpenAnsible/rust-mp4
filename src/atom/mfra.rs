/**

mfra
    tfra
    mfro

**/

use super::{Mp4File, Kind, Header, Atom};

#[derive(Debug, Clone)]
pub struct Mfra {
    header: Header,
    children: Vec<Atom>
}

impl Mfra {
    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str>{
        let children: Vec<Atom> = Atom::parse_children(f);
        Ok(Mfra{
            header: header,
            children: children
        })
    }
}

/**
aligned(8) class TrackFragmentRandomAccessBox extends FullBox(‘tfra’, version, 0) {
    unsigned int(32)  track_ID;
    const unsigned int(26)  reserved = 0;
    unsigned int(2)   length_size_of_traf_num;
    unsigned int(2)   length_size_of_trun_num;
    unsigned int(2)   length_size_of_sample_num;
    unsigned int(32)  number_of_entry;
    for(i=1; i <= number_of_entry; i++){
        if(version==1){
           unsigned int(64)  time;
           unsigned int(64)  moof_offset;
        }else{
           unsigned int(32)  time;
           unsigned int(32)  moof_offset;
        }
    }
    unsigned int((length_size_of_traf_num+1)*8)     traf_number;
    unsigned int((length_size_of_trun_num+1)*8)     trun_number;
    unsigned int((length_size_of_sample_num+1) * 8) sample_number;

`track_ID` is an integer identifying the track_ID.
`length_size_of_traf_num` indicates the length in byte of the traf_number field minus one. 
`length_size_of_trun_num` indicates the length in byte of the trun_number field minus one. 
`length_size_of_sample_num` indicates the length in byte of the sample_number field minus one. 
`number_of_entry` is an integer that gives the number of the entries for this track. 
    If this value is zero, it indicates that every sample is a sync sample and no table entry follows.
`time` is 32 or 64 bits integer that indicates the presentation time of the sync sample in units defined in
    the ‘mdhd’ of the associated track.
`moof_offset` is 32 or 64 bits integer that gives the offset of the ‘moof’ used in this entry. 
    Offset is the byte-offset between the beginning of the file and the beginning of the ‘moof’.
`traf_number` indicates the ‘traf’ number that contains the sync sample. The number ranges from 1
    (the first ‘traf’ is numbered 1) in each ‘moof’.
`trun_number` indicates the ‘trun’ number that contains the sync sample. The number ranges from 1 in each ‘traf’.
`sample_number` indicates the sample number of the sync sample. The number ranges from 1 in each ‘trun’.

**/

#[derive(Debug, Clone)]
pub struct Tfra {
    header: Header,
    sequence_number: u32
}

impl Tfra {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str>{
        header.parse_version(f);
        header.parse_flags(f);
        let curr_offset = f.offset();
        // f.seek(curr_offset+header.data_size);
        let sequence_number: u32 = f.read_u32().unwrap();
        f.offset_inc(header.data_size);
        Ok(Tfra{
            header: header,
            sequence_number: sequence_number
        })
    }
}