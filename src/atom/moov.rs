// Metadata container

use super::{Atom, Entry, Header, Mp4File};
use crate::{let_ok, let_some, Matrix};

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
    pub fn parse(f: &mut Mp4File, header: Header) -> Self {
        let children: Vec<Atom> = Atom::parse_children(f);
        Self { header, children }
    }

    pub const fn header(&self) -> &Header {
        &self.header
    }

    pub const fn children(&self) -> &Vec<Atom> {
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

        if header.version.is_none() {
            return Err("Header version is empty. Unable to continue.");
        }

        let curr_offset = f.offset();
        let mut length = 0;
        let creation_time;
        let modification_time;
        let timescale;
        let duration;

        if header.version.unwrap_or(0) == 1u8 {
            let_ok!(ct, f.read_u64(), "Unable to read creation time.");
            creation_time = ct;

            let_ok!(mt, f.read_u64(), "Unable to read modification time.");
            modification_time = mt;

            let_ok!(ts, f.read_u32(), "Unable to read timescale.");
            timescale = ts;

            let_ok!(dur, f.read_u64(), "Unable to read duration.");
            duration = dur;

            length += 28;
        } else {
            // header version == 0
            let_ok!(ct, f.read_u32(), "Unable to read creation time.");
            creation_time = u64::from(ct);

            let_ok!(mt, f.read_u32(), "Unable to read modification time.");
            modification_time = u64::from(mt);

            let_ok!(ts, f.read_u32(), "Unable to read timescale.");
            timescale = ts;

            let_ok!(dur, f.read_u32(), "Unable to read duration.");
            duration = u64::from(dur);
            length += 16;
        }
        // fixed point 16.16 number
        let_ok!(rate, f.read_fixed_point(16, 16), "Unable to read rate.");
        length += 4;

        // fixed point 8.8 number
        let_ok!(volume, f.read_fixed_point(8, 8), "Unable to read volume.");
        length += 2;

        // 10 Bytes reserved
        length += 10;

        let _offset = f.seek(curr_offset + length);

        // matrix
        let_ok!(matrix, f.read_matrix(), "Unable to read matrix.");
        length += 36;

        // 24 Bytes
        length += 24;
        let _offset = f.seek(curr_offset + length);

        let_ok!(next_track_id, f.read_u32(), "Unable to read next track ID.");
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
    pub fn parse(f: &mut Mp4File, header: Header) -> Self {
        let children: Vec<Atom> = Atom::parse_children(f);
        Self { header, children }
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
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Self {
        header.parse_version(f);
        header.parse_flags(f);

        let curr_offset = f.offset();
        let _offset = f.seek(curr_offset + header.data_size);
        f.offset_inc(header.data_size);
        Self { header }
    }
}

#[derive(Debug, Clone)]
pub struct Tref {
    pub header: Header,
}

impl Tref {
    pub fn parse(f: &mut Mp4File, header: Header) -> Self {
        let curr_offset = f.offset();
        let _offset = f.seek(curr_offset + header.data_size);
        f.offset_inc(header.data_size);
        Self { header }
    }
}

#[derive(Debug, Clone)]
pub struct Trgr {
    pub header: Header,
}

impl Trgr {
    #[allow(dead_code)]
    pub fn parse(f: &mut Mp4File, header: Header) -> Self {
        let curr_offset = f.offset();
        let _offset = f.seek(curr_offset + header.data_size);
        f.offset_inc(header.data_size);
        Self { header }
    }
}

#[derive(Debug, Clone)]
pub struct Mdia {
    pub header: Header,
    pub children: Vec<Atom>,
}

impl Mdia {
    pub fn parse(f: &mut Mp4File, header: Header) -> Self {
        let children: Vec<Atom> = Atom::parse_children(f);
        Self { header, children }
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

        let mut length = 0;

        let creation_time;
        let modification_time;
        let timescale;
        let duration;

        if header.version.is_none() {
            return Err("Unable to determine header version.");
        }

        if header.version.unwrap_or(0) == 1u8 {
            let_ok!(ct, f.read_u64(), "Unable to read creation time.");
            creation_time = ct;

            let_ok!(mt, f.read_u64(), "Unable to read modification time.");
            modification_time = mt;

            let_ok!(ts, f.read_u32(), "Unable to read time scale.");
            timescale = ts;

            let_ok!(du, f.read_u64(), "Unable to read duration.");
            duration = du;
            length += 28;
        } else {
            // header version == 0
            let_ok!(ct, f.read_u32(), "Unable to read creation time.");
            creation_time = u64::from(ct);

            let_ok!(mt, f.read_u32(), "Unable to read modification time.");
            modification_time = u64::from(mt);

            let_ok!(ts, f.read_u32(), "Unable to read time scale.");
            timescale = ts;

            let_ok!(du, f.read_u32(), "Unable to read duration.");
            duration = u64::from(du);
            length += 16;
        }

        // 16 Bytes
        // pad: 1 Bit
        // language: 15 Bit;
        let_ok!(
            language,
            f.read_iso639_code(),
            "Unable to read language from ISO639 code."
        );
        length += 2;

        // unsigned int(16) pre_defined = 0;
        length += 2;
        let _offset = f.seek(curr_offset + length);
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
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
        ];

        let_ok!(
            handler_type,
            String::from_utf8(handler_type_bytes.to_vec()),
            "Unable to get the handler type string from bytes."
        );

        // reserved
        let _throwaway = f.read_u32().unwrap_or_default();
        let _throwaway = f.read_u32().unwrap_or_default();
        let _throwaway = f.read_u32().unwrap_or_default();

        let name_length = header.data_size - 20;
        let mut name_bytes = Vec::new();
        for _bytes in 0..name_length {
            let_ok!(byte, f.read_u8(), "Unable to read name bytes");
            name_bytes.push(byte);
        }

        let_ok!(
            name,
            String::from_utf8(name_bytes),
            "Unable to convert bytes to name."
        );

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
    pub fn parse(f: &mut Mp4File, header: Header) -> Self {
        let children: Vec<Atom> = Atom::parse_children(f);
        Self { header, children }
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

        let_ok!(graphicsmode, f.read_u16(), "Unable to read graphics mode.");

        // red, green, blue
        let opcolor: [u16; 3] = [
            f.read_u16().unwrap_or_default(),
            f.read_u16().unwrap_or_default(),
            f.read_u16().unwrap_or_default(),
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

        let_ok!(balance, f.read_fixed_point(8, 8), "Unable to read balance.");

        // reserved
        let _throwaway = f.read_u16().unwrap_or_default();

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

        let_ok!(max_pdu_size, f.read_u16(), "Unable to read max PDU size.");
        let_ok!(
            avg_pdu_size,
            f.read_u16(),
            "Unable to read average PDU size."
        );
        let_ok!(max_bitrate, f.read_u32(), "Unable to read max bitrate.");
        let_ok!(avg_bitrate, f.read_u32(), "Unable to read average bitrate.");

        // reserved
        let _throwaway = f.read_u32().unwrap_or_default();

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
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Self {
        header.parse_version(f);
        header.parse_flags(f);

        Self { header }
    }
}

#[derive(Debug, Clone)]
pub struct Stbl {
    pub header: Header,
    pub children: Vec<Atom>,
}

impl Stbl {
    pub fn parse(f: &mut Mp4File, header: Header) -> Self {
        let children: Vec<Atom> = Atom::parse_children(f);
        Self { header, children }
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

        let_ok!(sample_size, f.read_u32(), "Unable to read sample size.");
        let_ok!(sample_count, f.read_u32(), "Unable to read sample count.");

        let entry_size = if sample_size == 0u32 {
            let mut es: Vec<u32> = Vec::new();
            for _sample in 0..sample_count {
                let_ok!(s, f.read_u32(), "Unable to read samples.");
                es.push(s);
            }
            Some(es)
        } else {
            None
        };

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

        let_ok!(
            _throwaway,
            f.read_u32(),
            "Unable to read first 4 bytes of file."
        );
        let_ok!(field_size, f.read_u8(), "Unable to read field size.");
        let_ok!(sample_count, f.read_u32(), "Unable to read sample count.");

        if field_size != 4u8 && field_size != 8u8 && field_size != 16u8 {
            return Err("Wrong field size detected. Should be 4, 8 or 16.");
        }

        let mut entry_size: Vec<u32> = Vec::new();
        let mut next_val: Option<u32> = None;

        for _ in 0..sample_count {
            if field_size == 4u8 {
                if next_val.is_some() {
                    let_some!(nv, next_val, "Unable to read next value.");
                    entry_size.push(nv);
                    next_val = None;
                } else {
                    let_ok!(byte, f.read_u8(), "Unable to read bits.");
                    let bits = format!("{byte:08b}");

                    let_ok!(
                        es,
                        u32::from_str_radix(&bits[0..4], 2),
                        "Unable to read entry size."
                    );
                    entry_size.push(es);

                    let_ok!(
                        nv,
                        u32::from_str_radix(&bits[4..8], 2),
                        "Unable to read next value."
                    );
                    next_val = Some(nv);
                }
            } else if field_size == 8u8 {
                let_ok!(es, f.read_u8(), "Unable to read entry size.");
                entry_size.push(u32::from(es));
            } else if field_size == 16u8 {
                let_ok!(es, f.read_u16(), "Unable to read entry size.");
                entry_size.push(u32::from(es));
            } else {
                return Err("STZ2 parse error.");
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

        let_ok!(entry_count, f.read_u32(), "Unable to read entry count.");
        let mut entries: Vec<Entry> = Vec::new();
        for _entry in 0..entry_count {
            let_ok!(first_chunk, f.read_u32(), "Unable to read first chunk.");
            let_ok!(
                samples_per_chunk,
                f.read_u32(),
                "Unable to read samples per chunk."
            );
            let_ok!(
                sample_description_index,
                f.read_u32(),
                "Unable to read sample description index."
            );

            let entry = Entry {
                first_chunk,
                samples_per_chunk,
                sample_description_index,
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

        let_ok!(entry_count, f.read_u32(), "Unable to read entry count.");
        let mut chunks: Vec<u32> = Vec::new();

        for _entry in 0..entry_count {
            let_ok!(chunk, f.read_u32(), "Unable to read chunk.");
            chunks.push(chunk);
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

        let_ok!(entry_count, f.read_u32(), "Unable to read entry count.");
        let mut chunks: Vec<u64> = Vec::new();

        for _entry in 0..entry_count {
            let_ok!(chunk, f.read_u64(), "Unable to read chunk.");
            chunks.push(chunk);
        }

        let _offset = f.offset_inc(header.data_size);
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

        let_ok!(sample_count, f.read_u32(), "Unable to read sample count.");

        let _seek_res = f.seek(curr_offset + header.data_size);
        let _offset = f.offset_inc(header.data_size);

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
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Self {
        header.parse_version(f);
        header.parse_flags(f);

        let curr_offset = f.offset();
        let _seek_res = f.seek(curr_offset + header.data_size);
        let _offset = f.offset_inc(header.data_size);

        Self { header }
    }
}

#[derive(Debug, Clone)]
pub struct Stdp {
    pub header: Header,
}

impl Stdp {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Self {
        header.parse_version(f);
        header.parse_flags(f);

        let curr_offset = f.offset();
        let _seek_res = f.seek(curr_offset + header.data_size);
        let _offset = f.offset_inc(header.data_size);

        Self { header }
    }
}

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

        let_ok!(entry_count, f.read_u32(), "Unable to read entry count.");
        let mut entries = Vec::new();

        for _entry in 0..entry_count {
            let_ok!(sample_count, f.read_u32(), "Unable to read sample count.");
            let_ok!(sample_delta, f.read_u32(), "Unable to read sample delta.");
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
    #[allow(clippy::cast_possible_wrap)]
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let_some!(version, header.version, "No header version found.");

        let_ok!(entry_count, f.read_u32(), "Unable to read entry count.");
        let mut entries = Vec::new();

        for _ in 0..entry_count {
            let_ok!(sample_count, f.read_u32(), "Unable to read sample count.");
            let sample_offset = if version == 0u8 {
                let_ok!(so, f.read_u32(), "Unable to read sample offset (u32).");
                so as i32
            } else {
                let_ok!(so, f.read_i32(), "Unable to read sample offset (i32).");
                so
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
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Self {
        header.parse_version(f);
        header.parse_flags(f);

        let curr_offset = f.offset();
        let _seek_res = f.seek(curr_offset + header.data_size);
        let _offset = f.offset_inc(header.data_size);

        Self { header }
    }
}

#[derive(Debug, Clone)]
pub struct Stss {
    pub header: Header,
}

impl Stss {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Self {
        header.parse_version(f);
        header.parse_flags(f);

        let curr_offset = f.offset();
        let _seek_res = f.seek(curr_offset + header.data_size);
        let _offset = f.offset_inc(header.data_size);

        Self { header }
    }
}

#[derive(Debug, Clone)]
pub struct Stsh {
    pub header: Header,
}

impl Stsh {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Self {
        header.parse_version(f);
        header.parse_flags(f);

        let curr_offset = f.offset();
        let _seek_res = f.seek(curr_offset + header.data_size);
        let _offset = f.offset_inc(header.data_size);

        Self { header }
    }
}

#[derive(Debug, Clone)]
pub struct Sdtp {
    pub header: Header,
}

impl Sdtp {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Self {
        header.parse_version(f);
        header.parse_flags(f);

        let curr_offset = f.offset();
        let _seek_res = f.seek(curr_offset + header.data_size);
        let _offset = f.offset_inc(header.data_size);

        Self { header }
    }
}

#[derive(Debug, Clone)]
pub struct Mvex {
    pub header: Header,
    pub children: Vec<Atom>,
}

impl Mvex {
    pub fn parse(f: &mut Mp4File, header: Header) -> Self {
        let children: Vec<Atom> = Atom::parse_children(f);
        Self { header, children }
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

        let_some!(version, header.version, "No header version set.");
        let fragment_duration: u64 = if version == 1u8 {
            let_ok!(fd, f.read_u64(), "Unable to read fragment duration (u64).");
            fd
        } else {
            let_ok!(fd, f.read_u32(), "Unable to read fragment duration (u32).");
            u64::from(fd)
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
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Self {
        header.parse_version(f);
        header.parse_flags(f);

        let curr_offset = f.offset();
        let _seek_res = f.seek(curr_offset + header.data_size);
        let _offset = f.offset_inc(header.data_size);

        Self { header }
    }
}
