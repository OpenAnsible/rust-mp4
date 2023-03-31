// Metadata container

use super::{Atom, Entry, Header, Mp4File};
use crate::Matrix;

use std::string::String;

/*

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

    See the moov.md
*/

#[derive(Debug, Clone)]
pub struct Moov {
    header: Header,
    children: Vec<Atom>,
}

impl Moov {
    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str> {
        let children: Vec<Atom> = Atom::parse_children(f);
        Ok(Self { header, children })
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn children(&self) -> &Vec<Atom> {
        &self.children
    }
}

// See mvhd.md for notes
#[derive(Debug, Clone)]
pub struct Mvhd {
    pub header: Header,
    pub creation_time: u64,
    pub modification_time: u64,
    pub timescale: u32,
    pub duration: u64,

    pub rate: f64,
    pub volume: f64,
    pub matrix: Matrix,
    pub next_track_id: u32,
}

impl Mvhd {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let curr_offset = f.offset();

        let mut length = 0u64;

        let creation_time;
        let modification_time;
        let timescale;
        let duration;

        assert!(header.version.is_some());

        if header.version.unwrap() == 1u8 {
            creation_time = f.read_u64().unwrap();
            modification_time = f.read_u64().unwrap();
            timescale = f.read_u32().unwrap();
            duration = f.read_u64().unwrap();
            length += 28;
        } else {
            // header version == 0
            creation_time = u64::from(f.read_u32().unwrap());
            modification_time = u64::from(f.read_u32().unwrap());
            timescale = f.read_u32().unwrap();
            duration = u64::from(f.read_u32().unwrap());
            length += 16;
        }
        // fixed point 16.16 number
        let rate = f.read_fixed_point(16, 16).unwrap(); // u32
        length += 4;

        // fixed point 8.8 number
        let volume = f.read_fixed_point(8, 8).unwrap(); // u16
        length += 2;

        // 10 Bytes reserved
        length += 10;

        let _ = f.seek(curr_offset + length);
        // matrix
        let matrix: Matrix = f.read_matrix().unwrap(); // 36 Bytes
        length += 36;

        // 24 Bytes
        length += 24;
        let _ = f.seek(curr_offset + length);

        let next_track_id = f.read_u32().unwrap();
        length += 4;

        f.offset_inc(length);

        Ok(Self {
            header,
            creation_time,
            modification_time,
            timescale,
            duration,
            rate,
            volume,
            matrix,
            next_track_id,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Trak {
    pub header: Header,
    pub children: Vec<Atom>,
}

impl Trak {
    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str> {
        let children: Vec<Atom> = Atom::parse_children(f);
        Ok(Self { header, children })
    }
}

#[derive(Debug, Clone)]
pub struct Tkhd {
    pub header: Header, // creation_time: u64,
                        // modification_time: u64,
                        // track_id: u32,
                        // duration: u64,

                        // layer: i16,
                        // alternate_group: i16,
                        // // fixed 8.8 value
                        // volume: f64, // {if track_is_audio 0x0100 else 0};

                        // matrix: Matrix,
                        // // fixed-point 16.16 values
                        // width: f64,
                        // height: f64
}

impl Tkhd {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let curr_offset = f.offset();
        let _ = f.seek(curr_offset + header.data_size);
        f.offset_inc(header.data_size);
        Ok(Self { header })
    }
}

#[derive(Debug, Clone)]
pub struct Tref {
    pub header: Header,
}

impl Tref {
    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str> {
        let curr_offset = f.offset();
        let _ = f.seek(curr_offset + header.data_size);
        f.offset_inc(header.data_size);
        Ok(Self { header })
    }
}

#[derive(Debug, Clone)]
pub struct Trgr {
    pub header: Header,
}

impl Trgr {
    #[allow(dead_code)]
    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str> {
        let curr_offset = f.offset();
        let _ = f.seek(curr_offset + header.data_size);
        f.offset_inc(header.data_size);
        Ok(Self { header })
    }
}

#[derive(Debug, Clone)]
pub struct Mdia {
    pub header: Header,
    pub children: Vec<Atom>,
}

impl Mdia {
    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str> {
        let children: Vec<Atom> = Atom::parse_children(f);
        Ok(Self { header, children })
    }
}

#[derive(Debug, Clone)]
pub struct Mdhd {
    pub header: Header,
    pub creation_time: u64,
    pub modification_time: u64,
    pub timescale: u32,
    pub duration: u64,
    pub language: String,
}

impl Mdhd {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let curr_offset = f.offset();

        let mut length = 0u64;

        let creation_time;
        let modification_time;
        let timescale;
        let duration;
        assert!(header.version.is_some());

        if header.version.unwrap() == 1u8 {
            creation_time = f.read_u64().unwrap();
            modification_time = f.read_u64().unwrap();
            timescale = f.read_u32().unwrap();
            duration = f.read_u64().unwrap();
            length += 28;
        } else {
            // header version == 0
            creation_time = u64::from(f.read_u32().unwrap());
            modification_time = u64::from(f.read_u32().unwrap());
            timescale = f.read_u32().unwrap();
            duration = u64::from(f.read_u32().unwrap());
            length += 16;
        }

        // 16 Bytes
        // pad: 1 Bit
        // language: 15 Bit;
        let language = f.read_iso639_code().unwrap(); // 2 Bytes, u16
        length += 2;

        // unsigned int(16) pre_defined = 0;
        length += 2;
        let _ = f.seek(curr_offset + length);
        f.offset_inc(length);

        Ok(Self {
            header,
            creation_time,
            modification_time,
            timescale,
            duration,
            language,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Hdlr {
    pub header: Header,
    pub handler_type: String,
    pub name: String,
}

impl Hdlr {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        // u32 = [u8, u8, u8, u8]
        let handler_type_bytes: [u8; 4] = [
            f.read_u8().unwrap(),
            f.read_u8().unwrap(),
            f.read_u8().unwrap(),
            f.read_u8().unwrap(),
        ];
        let handler_type = String::from_utf8(handler_type_bytes.to_vec()).unwrap();
        // reserved
        f.read_u32().unwrap();
        f.read_u32().unwrap();
        f.read_u32().unwrap();

        let name_length = header.data_size - 20;
        let mut name_bytes = Vec::new();
        for _ in 0..name_length {
            name_bytes.push(f.read_u8().unwrap());
        }
        let name = String::from_utf8(name_bytes).unwrap();

        f.offset_inc(header.data_size);
        Ok(Self {
            header,
            handler_type,
            name,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Minf {
    pub header: Header,
    pub children: Vec<Atom>, // Box Types: ‘vmhd’, ‘smhd’, ’hmhd’, ‘nmhd’
}

impl Minf {
    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str> {
        let children: Vec<Atom> = Atom::parse_children(f);
        Ok(Self { header, children })
    }
}

#[derive(Debug, Clone)]
pub struct Vmhd {
    pub header: Header,
    pub graphicsmode: u16,
    pub opcolor: [u16; 3],
}

impl Vmhd {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let graphicsmode = f.read_u16().unwrap();

        // red, greenm blue
        let opcolor: [u16; 3] = [
            f.read_u16().unwrap(),
            f.read_u16().unwrap(),
            f.read_u16().unwrap(),
        ];

        f.offset_inc(8);

        Ok(Self {
            header,
            graphicsmode,
            opcolor,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Smhd {
    pub header: Header,
    pub balance: f64, // fixed-point 8.8 number
}

impl Smhd {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let balance = f.read_fixed_point(8, 8).unwrap(); // 2 Bytes
                                                         // reserved
        f.read_u16().unwrap();

        f.offset_inc(4);

        Ok(Self { header, balance })
    }
}

#[derive(Debug, Clone)]
pub struct Hmhd {
    pub header: Header,
    pub max_pdu_size: u16,
    pub avg_pdu_size: u16,
    pub max_bitrate: u32,
    pub avg_bitrate: u32,
}

impl Hmhd {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let max_pdu_size = f.read_u16().unwrap();
        let avg_pdu_size = f.read_u16().unwrap();
        let max_bitrate = f.read_u32().unwrap();
        let avg_bitrate = f.read_u32().unwrap();
        // reserved
        f.read_u32().unwrap();

        f.offset_inc(16);

        Ok(Self {
            header,
            max_pdu_size,
            avg_pdu_size,
            max_bitrate,
            avg_bitrate,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Nmhd {
    pub header: Header,
}

impl Nmhd {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        Ok(Self { header })
    }
}

#[derive(Debug, Clone)]
pub struct Stbl {
    pub header: Header,
    pub children: Vec<Atom>,
}

impl Stbl {
    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str> {
        let children: Vec<Atom> = Atom::parse_children(f);
        Ok(Self { header, children })
    }
}

#[derive(Debug, Clone)]
pub struct Stsz {
    pub header: Header,
    pub sample_size: u32,
    pub sample_count: u32,
    pub entry_size: Option<Vec<u32>>,
}

impl Stsz {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        // let curr_offset = f.offset();
        // f.seek(curr_offset+header.data_size);
        let sample_size: u32 = f.read_u32().unwrap();
        let sample_count: u32 = f.read_u32().unwrap();
        let mut entry_size = None;

        if sample_size == 0u32 {
            let mut _entry_size: Vec<u32> = Vec::new();
            for _ in 0..sample_count {
                _entry_size.push(f.read_u32().unwrap());
            }
            entry_size = Some(_entry_size);
        }

        f.offset_inc(header.data_size);

        Ok(Self {
            header,
            sample_size,
            sample_count,
            entry_size,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Stz2 {
    pub header: Header,
    pub field_size: u8,
    pub sample_count: u32,
    pub entry_size: Vec<u32>,
}

impl Stz2 {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);
        // let curr_offset = f.offset();
        // f.seek(curr_offset+header.data_size);
        let _ = f.read_u32().unwrap();
        let field_size = f.read_u8().unwrap();
        let sample_count = f.read_u32().unwrap();
        // value 4, 8 or 16.
        assert!(field_size == 4u8 || field_size == 8u8 || field_size == 16u8);

        let mut entry_size: Vec<u32> = Vec::new();

        let mut next_val: Option<u32> = None;

        for _ in 0..sample_count {
            if field_size == 4u8 {
                if next_val.is_some() {
                    entry_size.push(next_val.unwrap());
                    next_val = None;
                } else {
                    let bits = format!("{:08b}", f.read_u8().unwrap());
                    entry_size.push(u32::from_str_radix(&bits[0..4], 2).unwrap());
                    next_val = Some(u32::from_str_radix(&bits[4..8], 2).unwrap());
                }
            } else if field_size == 8u8 {
                entry_size.push(u32::from(f.read_u8().unwrap()));
            } else if field_size == 16u8 {
                entry_size.push(u32::from(f.read_u16().unwrap()));
            } else {
                panic!("STZ2 parse error.");
            }
        }

        f.offset_inc(header.data_size);
        Ok(Self {
            header,
            field_size,
            sample_count,
            entry_size,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Stsc {
    pub header: Header,
    pub entry_count: u32,
    pub entries: Vec<Entry>,
}

impl Stsc {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);
        // let curr_offset = f.offset();
        // f.seek(curr_offset+header.data_size);

        let entry_count = f.read_u32().unwrap();
        let mut entries: Vec<Entry> = Vec::new();
        for _ in 0..entry_count {
            let entry = Entry {
                first_chunk: f.read_u32().unwrap(),
                samples_per_chunk: f.read_u32().unwrap(),
                sample_description_index: f.read_u32().unwrap(),
            };
            entries.push(entry);
        }

        f.offset_inc(header.data_size);
        Ok(Self {
            header,
            entry_count,
            entries,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Stco {
    pub header: Header,
    pub entry_count: u32,
    pub chunks: Vec<u32>,
}

impl Stco {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);
        // let curr_offset = f.offset();
        // f.seek(curr_offset+header.data_size);

        let entry_count = f.read_u32().unwrap();
        let mut chunks: Vec<u32> = Vec::new();

        for _ in 0..entry_count {
            chunks.push(f.read_u32().unwrap());
        }

        f.offset_inc(header.data_size);
        Ok(Self {
            header,
            entry_count,
            chunks,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Co64 {
    pub header: Header,
    pub entry_count: u32,
    pub chunks: Vec<u64>,
}

impl Co64 {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);
        // let curr_offset = f.offset();
        // f.seek(curr_offset+header.data_size);

        let entry_count = f.read_u32().unwrap();
        let mut chunks: Vec<u64> = Vec::new();

        for _ in 0..entry_count {
            chunks.push(f.read_u64().unwrap());
        }

        f.offset_inc(header.data_size);
        Ok(Self {
            header,
            entry_count,
            chunks,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Padb {
    pub header: Header,
    pub sample_count: u32,
}

impl Padb {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);
        let curr_offset = f.offset();

        let sample_count = f.read_u32().unwrap();
        // f.offset_inc(4);
        // for i in 0..((sample_count+1)/2) {
        //     let bits = format!("{:08b}", f.read_u8().unwrap());
        //     let pad1 = u32::from_str_radix(&bits[1..4], 2).unwrap();
        //     let pad2 = u32::from_str_radix(&bits[5..8], 2).unwrap();
        // }

        let _ = f.seek(curr_offset + header.data_size);
        f.offset_inc(header.data_size);
        Ok(Self {
            header,
            sample_count,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Stsd {
    pub header: Header,
}

impl Stsd {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);
        let curr_offset = f.offset();
        let _ = f.seek(curr_offset + header.data_size);
        f.offset_inc(header.data_size);
        Ok(Self { header })
    }
}

#[derive(Debug, Clone)]
pub struct Stdp {
    pub header: Header,
}

impl Stdp {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);
        let curr_offset = f.offset();
        let _ = f.seek(curr_offset + header.data_size);
        f.offset_inc(header.data_size);
        Ok(Self { header })
    }
}

/**

**/

#[derive(Debug, Clone)]
pub struct SttsEntry {
    pub sample_count: u32,
    pub sample_delta: u32,
}

#[derive(Debug, Clone)]
pub struct Stts {
    pub header: Header,
    pub entry_count: u32,
    pub entries: Vec<SttsEntry>,
}

impl Stts {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);
        // let curr_offset = f.offset();
        // f.seek(curr_offset+header.data_size);
        let entry_count = f.read_u32().unwrap();
        let mut entries = Vec::new();

        for _ in 0..entry_count {
            let sample_count: u32 = f.read_u32().unwrap();
            let sample_delta: u32 = f.read_u32().unwrap();
            entries.push(SttsEntry {
                sample_count,
                sample_delta,
            });
        }

        f.offset_inc(header.data_size);
        Ok(Self {
            header,
            entry_count,
            entries,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CttsEntryOffset {
    pub sample_count: u32,
    pub sample_offset: i32,
}

#[derive(Debug, Clone)]
pub struct Ctts {
    pub header: Header,
    pub entry_count: u32,
    pub entries: Vec<CttsEntryOffset>,
}

impl Ctts {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);
        // let curr_offset = f.offset();
        // f.seek(curr_offset+header.data_size);

        let version: u8 = header.version.unwrap();

        let entry_count = f.read_u32().unwrap();
        let mut entries = Vec::new();

        for _ in 0..entry_count {
            let sample_count = f.read_u32().unwrap();
            let sample_offset = if version == 0u8 {
                f.read_u32().unwrap() as i32
            } else {
                f.read_i32().unwrap()
            };

            entries.push(CttsEntryOffset {
                sample_count,
                sample_offset,
            });
        }

        f.offset_inc(header.data_size);
        Ok(Self {
            header,
            entry_count,
            entries,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Cslg {
    pub header: Header,
}

impl Cslg {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);
        let curr_offset = f.offset();
        let _ = f.seek(curr_offset + header.data_size);
        f.offset_inc(header.data_size);
        Ok(Self { header })
    }
}

#[derive(Debug, Clone)]
pub struct Stss {
    pub header: Header,
}

impl Stss {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);
        let curr_offset = f.offset();
        let _ = f.seek(curr_offset + header.data_size);
        f.offset_inc(header.data_size);
        Ok(Self { header })
    }
}

#[derive(Debug, Clone)]
pub struct Stsh {
    pub header: Header,
}

impl Stsh {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);
        let curr_offset = f.offset();
        let _ = f.seek(curr_offset + header.data_size);
        f.offset_inc(header.data_size);
        Ok(Self { header })
    }
}

#[derive(Debug, Clone)]
pub struct Sdtp {
    pub header: Header,
}

impl Sdtp {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);
        let curr_offset = f.offset();
        let _ = f.seek(curr_offset + header.data_size);
        f.offset_inc(header.data_size);
        Ok(Self { header })
    }
}

#[derive(Debug, Clone)]
pub struct Mvex {
    pub header: Header,
    pub children: Vec<Atom>,
}

impl Mvex {
    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str> {
        let children: Vec<Atom> = Atom::parse_children(f);
        Ok(Self { header, children })
    }
}

/// The Movie Extends Header is optional, and provides the overall duration, including fragments, of a fragmented movie.
/// If this box is not present, the overall duration must be computed by examining each fragment.
#[derive(Debug, Clone)]
pub struct Mehd {
    pub header: Header,
    pub fragment_duration: u64,
}

impl Mehd {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let fragment_duration: u64 = if header.version.unwrap() == 1u8 {
            f.read_u64().unwrap()
        } else {
            u64::from(f.read_u32().unwrap())
        };

        // f.seek(curr_offset+header.data_size);
        f.offset_inc(header.data_size);
        Ok(Self {
            header,
            fragment_duration,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Trex {
    pub header: Header,
}

impl Trex {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);
        let curr_offset = f.offset();
        let _ = f.seek(curr_offset + header.data_size);
        f.offset_inc(header.data_size);
        Ok(Self { header })
    }
}
