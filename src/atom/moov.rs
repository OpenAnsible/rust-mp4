// Metadata container

use super::{Atom, Entry, Header, Kind, Mp4File};
use crate::Matrix;
use std::mem;
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

    pub fn header_ref(&self) -> &Header {
        &self.header
    }

    pub fn children_ref(&self) -> &Vec<Atom> {
        &self.children
    }

    pub fn header(&self) -> Header {
        self.header.clone()
    }

    pub fn children(&self) -> Vec<Atom> {
        self.children.clone()
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

        let mut creation_time = 0u64;
        let mut modification_time = 0u64;
        let mut timescale = 0u32;
        let mut duration = 0u64;
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

        f.seek(curr_offset + length);
        // matrix
        let matrix: Matrix = f.read_matrix().unwrap(); // 36 Bytes
        length += 36;

        // 24 Bytes
        length += 24;
        f.seek(curr_offset + length);

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

/*

Box Type : trak
Container: Movie Box (‘moov’)
Mandatory: Yes
Quantity : One or more

This is a container box for a single track of a presentation.
A presentation consists of one or more tracks.
Each track is independent of the other tracks in the presentation
and carries its own temporal and spatial information.
Each track will contain its associated Media Box.

Tracks are used for two purposes: (a) to contain media data (media tracks)
and (b) to contain packetization information for streaming protocols (hint tracks).

There shall be at least one media track within an ISO file,
and all the media tracks that contributed to the hint tracks shall remain in the file,
even if the media data within them is not referenced by the hint tracks;
after deleting all hint tracks, the entire un-hinted presentation shall remain.

8.3.1.2 Syntax

aligned(8) class TrackBox extends Box(‘trak’) {

}

*/

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

/**
8.3.2.1

Box Type : ‘tkhd’
Container: Track Box (‘trak’)
Mandatory: Yes
Quantity : Exactly one


This box specifies the characteristics of a single track. Exactly one
Track Header Box is contained in a track.

In the absence of an edit list, the presentation of a track starts
at the beginning of the overall presentation. An empty edit is used
to offset the start time of a track.

The default value of the track header flags for media tracks
is 7 (track_enabled, track_in_movie, track_in_preview).
If in a presentation all tracks have neither track_in_movie nor track_in_preview set,
then all tracks shall be treated as if both flags were set on all tracks.
Server hint tracks should have the track_in_movie and track_in_preview set to 0,
so that they are ignored for local playback and preview.

Under the ‘iso3’ brand or brands that share its requirements,
the width and height in the track header are measured on a notional 'square' (uniform) grid.
Track video data is normalized to these dimensions (logically) before any transformation
or placement caused by a layup or composition system. Track (and movie) matrices, if used,
also operate in this uniformly-scaled space.

8.3.2.2 Syntax

aligned(8) class TrackHeaderBox
extends FullBox(‘tkhd’, version, flags){
    if (version==1) {
        unsigned int(64)  creation_time;
        unsigned int(64)  modification_time;
        unsigned int(32)  track_ID;
        const unsigned int(32)  reserved = 0;
        unsigned int(64)  duration;
    } else { // version==0
        unsigned int(32)  creation_time;
        unsigned int(32)  modification_time;
        unsigned int(32)  track_ID;
        const unsigned int(32)  reserved = 0;
        unsigned int(32)  duration;
    }
    const unsigned int(32)[2] reserved = 0;
    template int(16) layer = 0;
    template int(16) alternate_group = 0;
    template int(16) volume = {if track_is_audio 0x0100 else 0};
    const unsigned int(16) reserved = 0;
    // unity matrix
    template int(32)[9] matrix = { 0x00010000,0,0,0,0x00010000,0,0,0,0x40000000 };

    unsigned int(32) width;
    unsigned int(32) height;
}

8.3.2.3 Semantics

version is an integer that specifies the version of this box (0 or 1 in this specification)
flags is a 24-bit integer with flags; the following values are defined:
            Track_enabled: Indicates that the track is enabled.
                Flag value is 0x000001.
                A disabled track (the low bit is zero) is treated as if it were not present.
            Track_in_movie: Indicates that the track is used in the presentation.
                Flag value is 0x000002.
            Track_in_preview: Indicates that the track is used when previewing
                the presentation. Flag value is 0x000004.

creation_time is an integer that declares the creation time of
    this track (in seconds since midnight, Jan. 1, 1904, in UTC time)
modification_time is an integer that declares the most recent time
    the track was modified (in seconds since midnight, Jan. 1, 1904, in UTC time)
track_ID is an integer that uniquely identifies this track over
    the entire life-time of this presentation.
    Track IDs are never re-used and cannot be zero.
duration is an integer that indicates the duration of
    this track (in the timescale indicated in the Movie Header Box).
    The value of this field is equal to the sum of the durations
    of all of the track’s edits. If there is no edit list,
    then the duration is the sum of the sample durations,
    converted into the timescale in the Movie Header Box.
    If the duration of this track cannot be determined then duration is set to all 1s.
layer specifies the front-to-back ordering of video tracks;
    tracks with lower numbers are closer to the viewer. 0 is the normal value,
    and -1 would be in front of track 0, and so on.
alternate_group is an integer that specifies a group or collection of tracks.
    If this field is 0 there is no information on possible relations to other tracks.
    If this field is not 0, it should be the same for tracks that contain alternate
    data for one another and different for tracks belonging to different such groups.
    Only one track within an alternate group should be played or streamed at any one time,
    and must be distinguishable from other tracks in the group via attributes such as bitrate,
    codec, language, packet size etc. A group may have only one member.
volume is a fixed 8.8 value specifying the track's relative audio volume.
    Full volume is 1.0 (0x0100) and is the normal value.
    Its value is irrelevant for a purely visual track.
    Tracks may be composed by combining them according to their volume,
    and then using the overall Movie Header Box volume setting;
    or more complex audio composition (e.g. MPEG-4 BIFS) may be used.
matrix provides a transformation matrix for the video; (u,v,w) are restricted here
    to (0,0,1), hex (0,0,0x40000000).
width and height specify the track's visual presentation size as fixed-point 16.16 values.
    These need not be the same as the pixel dimensions of the images,
    which is documented in the sample description(s);
    all images in the sequence are scaled to this size,
    before any overall transformation of the track represented by the matrix.
    The pixel dimensions of the images are the default values.
**/

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
        f.seek(curr_offset + header.data_size);
        f.offset_inc(header.data_size);
        Ok(Self { header })
    }
}

/**
Box Type : tref
Container: Track Box(trak)
Mandatory: No
Quantity : Zero or one

8.3.3.2 Syntax

aligned(8) class TrackReferenceBox extends Box(‘tref’) {

}
aligned(8) class TrackReferenceTypeBox (unsigned int(32) reference_type) extends Box(reference_type) {
   unsigned int(32) track_IDs[];
}

8.3.3.3 Semantics

The Track Reference Box contains track reference type boxes.

track_ID is an integer that provides a reference from the containing track
    to another track in the presentation. track_IDs are never re-used and cannot be equal to zero.
The reference_type shall be set to one of the following values, or a value registered
    or from a derived specification or registration:
        *   hint the referenced track(s) contain the original media for this hint track
        *   cdsc this track describes the referenced track.
        *   hind this track depends on the referenced hint track, i.e.,
             it should only be used if the referenced hint track is used.
        *   vdep this track contains auxiliary depth video information for the referenced video track
        *   vplx this track contains auxiliary parallax video information for the referenced video track

**/

#[derive(Debug, Clone)]
pub struct Tref {
    pub header: Header,
}

impl Tref {
    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str> {
        let curr_offset = f.offset();
        f.seek(curr_offset + header.data_size);
        f.offset_inc(header.data_size);
        Ok(Self { header })
    }
}

/**
Box Type : ‘trgr’
Container: Track Box (‘trak’)
Mandatory: No
Quantity : Zero or one

**/

#[derive(Debug, Clone)]
pub struct Trgr {
    pub header: Header,
}

impl Trgr {
    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str> {
        let curr_offset = f.offset();
        f.seek(curr_offset + header.data_size);
        f.offset_inc(header.data_size);
        Ok(Self { header })
    }
}

/**
Box Type : mdia
Container: Track Box (‘trak’)
Mandatory: Yes
Quantity : Exactly One

**/

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

/**
Box Type : mdhd
Container: Media Box(mdia)
Mandatory: Yes
Quantity : Exactly one

8.4.2.2 Syntax
aligned(8) class MediaHeaderBox extends FullBox(‘mdhd’, version, 0) {
    if (version==1) {
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
    bit(1) pad = 0;
    unsigned int(5)[3] language; // ISO-639-2/T language code
    unsigned int(16) pre_defined = 0;
}

**/

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

        let mut creation_time = 0u64;
        let mut modification_time = 0u64;
        let mut timescale = 0u32;
        let mut duration = 0u64;
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
        f.seek(curr_offset + length);
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

/**
8.4.3.2 Syntax

aligned(8) class HandlerBox extends FullBox(‘hdlr’, version = 0, 0) {
    unsigned int(32) pre_defined = 0;
    unsigned int(32) handler_type;
    const unsigned int(32)[3] reserved = 0;
    string   name;
}

8.4.3.3 Semantics

version is an integer that specifies the version of this box
handler_type when present in a media box, is an integer containing one of the following values,
    or a value from a derived specification:
        ‘vide’ Video track
        ‘soun’ Audio track
        ‘hint’ Hint track
        ‘meta’ Timed Metadata track
        ‘auxv’ Auxiliary Video track

handler_type when present in a meta box, contains an appropriate value to
    indicate the format of the meta box contents. The value ‘null’ can be
    used in the primary meta box to indicate that it is merely being used to hold resources.
name is a null-terminated string in UTF-8 characters which gives a human-readable name
    for the track type (for debugging and inspection purposes).
**/

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

/**
aligned(8) class CompactSampleSizeBox extends FullBox(‘stz2’, version = 0, 0) {
    unsigned int(24) reserved = 0;
    unisgned int(8) field_size;
    unsigned int(32) sample_count;
    for (i=1; i <= sample_count; i++) {
        unsigned int(field_size)   entry_size;
    }
}

8.7.3.3.2 Semantics

version is an integer that specifies the version of this box
field_size is an integer specifying the size in bits of the entries in the following table;
    it shall take the value 4, 8 or 16.
    If the value 4 is used, then each byte contains two values: entry[i]<<4 + entry[i+1];
    if the sizes do not fill an integral number of bytes, the last byte is padded with zeros.
sample_count is an integer that gives the number of entries in the following table
entry_size is an integer specifying the size of a sample, indexed by its number.

**/

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

/**

8.7.4.2 Syntax

aligned(8) class SampleToChunkBox extends FullBox(‘stsc’, version = 0, 0) {
   unsigned int(32)  entry_count;
   for (i=1; i <= entry_count; i++) {
        unsigned int(32) first_chunk;
        unsigned int(32) samples_per_chunk; unsigned int(32) sample_description_index;
    }
}

8.7.4.3 Semantics
version is an integer that specifies the version of this box
entry_count is an integer that gives the number of entries in the following table
first_chunk is an integer that gives the index of the first chunk in this run of chunks
    that share the same samples-per-chunk and sample-description-index;
    the index of the first chunk in a track has the value 1 (the first_chunk field in the
    first record of this box has the value 1, identifying that the first sample maps to the first chunk).
samples_per_chunk is an integer that gives the number of samples in each of these chunks
    sample_description_index is an integer that gives the index of the sample entry
    that describes the samples in this chunk. The index ranges from 1 to the number
    of sample entries in the Sample Description Box


**/

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

/**
8.7.5 Chunk Offset Box
8.7.5.1 Definition


Box Type : ‘stco’, ‘co64’
Container: Sample Table Box (‘stbl’)
Mandatory: Yes
Quantity : Exactly one variant must be present

The chunk offset table gives the index of each chunk into the containing file.
There are two variants, permitting the use of 32-bit or 64-bit offsets.
The latter is useful when managing very large presentations. At most one of these
variants will occur in any single instance of a sample table.

Offsets are file offsets, not the offset into any box within the file (e.g. Media Data Box).
This permits referring to media data in files without any box structure. It does also mean
that care must be taken when constructing a self-contained ISO file with its metadata (Movie Box)
at the front, as the size of the Movie Box will affect the chunk offsets to the media data.

8.7.5.2 Syntax
aligned(8) class ChunkOffsetBox
   extends FullBox(‘stco’, version = 0, 0) {
   unsigned int(32)  entry_count;
   for (i=1; i <= entry_count; i++) {
      unsigned int(32)  chunk_offset;
   }
}
aligned(8) class ChunkLargeOffsetBox
   extends FullBox(‘co64’, version = 0, 0) {
   unsigned int(32)  entry_count;
   for (i=1; i <= entry_count; i++) {
      unsigned int(64)  chunk_offset;
   }
}

8.7.5.3 Semantics
version is an integer that specifies the version of this box
entry_count is an integer that gives the number of entries in the following table
chunk_offset is a 32 or 64 bit integer that gives the offset of the start
    of a chunk into its containing media file.

**/

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

/**
Box Type : padb
Container: Sample Table (‘stbl’)
Mandatory: No
Quantity : Zero or one


8.7.6.3 Semantics

In some streams the media samples do not occupy all bits of the bytes given by the sample size,
and are padded at the end to a byte boundary. In some cases, it is necessary to record
externally the number of padding bits used. This table supplies that information.

8.7.6.2 Syntax

aligned(8) class PaddingBitsBox extends FullBox(‘padb’, version = 0, 0) {
    unsigned int(32) sample_count;
    int i;
    for (i=0; i < ((sample_count + 1)/2); i++) {
        bit(1)   reserved = 0;
        bit(3)   pad1;
        bit(1)   reserved = 0;
        bit(3)   pad2;
    }
}

sample_count – counts the number of samples in the track;
    it should match the count in other tables
pad1 – a value from 0 to 7, indicating the number of
    bits at the end of sample (i*2)+1.
pad2 – a value from 0 to 7, indicating the number of
    bits at the end of sample (i*2)+2

**/

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

        f.seek(curr_offset + header.data_size);
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
        f.seek(curr_offset + header.data_size);
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
        f.seek(curr_offset + header.data_size);
        f.offset_inc(header.data_size);
        Ok(Self { header })
    }
}

/**

8.6.1.2
8.6.1.2.1 Decoding Time to Sample Box Definition
Box Type : stts
Container: Sample Table Box (‘stbl’)
Mandatory: Yes
Quantity : Exactly one

This box contains a compact version of a table that allows indexing from decoding time to sample number.
Other tables give sample sizes and pointers, from the sample number. Each entry in the table gives
the number of consecutive samples with the same time delta, and the delta of those samples.
By adding the deltas a complete time-to-sample map may be built.

The Decoding Time to Sample Box contains decode time delta's:
    DT(n+1) = DT(n) + STTS(n) where STTS(n) is the (uncompressed) table entry for sample n.

The sample entries are ordered by decoding time stamps; therefore the deltas are all non-negative.

The DT axis has a zero origin; DT(i) = SUM(for j=0 to i-1 of delta(j)),
and the sum of all deltas gives the length of the media in
the track (not mapped to the overall timescale, and not considering any edit list).

The Edit List Box provides the initial CT value if it is non-empty (non-zero).

8.6.1.2.2 Syntax

aligned(8) class TimeToSampleBox extends FullBox(’stts’, version = 0, 0) {
    unsigned int(32)  entry_count;
    int i;
    for (i=0; i < entry_count; i++) {
        unsigned int(32)  sample_count;
        unsigned int(32)  sample_delta;
    }
}

For example with Table 2, the entry would be:

+++++++++++++++++++++++++++++++
| Sample count | Sample-delta |
+++++++++++++++++++++++++++++++
|     14       |      10      |
+++++++++++++++++++++++++++++++


8.6.1.2.3 Semantics

version - is an integer that specifies the version of this box.
entry_count - is an integer that gives the number of entries in the following table.
sample_count - is an integer that counts the number of consecutive samples that have the given
duration.
sample_delta - is an integer that gives the delta of these samples in the time-scale of the media.


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

/**
8.6.1.3
8.6.1.3.1 Composition Time to Sample Box Definition
Box Type : ctts
Container: Sample Table Box (‘stbl’)
Mandatory: No
Quantity : Zero or one


This box provides the offset between decoding time and composition time.

In version 0 of this box the decoding time must be less than the composition time,
and the offsets are expressed as unsigned numbers such that CT(n) = DT(n) + CTTS(n)
where CTTS(n) is the (uncompressed) table entry for sample n.

In version 1 of this box, the composition timeline and the decoding timeline are
still derived from each other, but the offsets are signed.
It is recommended that for the computed composition timestamps,
there is exactly one with the value 0 (zero).

For either version of the box, each sample must have a unique composition timestamp value,
that is, the timestamp for two samples shall never be the same.

It may be true that there is no frame to compose at time 0; the handling of
this is unspecified (systems might display the first frame for longer, or a suitable fill colour).

When version 1 of this box is used, the CompositionToDecodeBox may also be present in
the sample table to relate the composition and decoding timelines.
When backwards-compatibility or compatibility with an unknown set
of readers is desired, version 0 of this box should be used when possible.
In either version of this box, but particularly under version 0,
if it is desired that the media start at track time 0, and the first media
sample does not have a composition time of 0, an edit list may be used to ‘shift’ the media to time 0.

The composition time to sample table is optional and must only be present
if DT and CT differ for any samples.

Hint tracks do not use this box.

For example in Table 2

+++++++++++++++++++++++++++++++
| Sample count | Sample_offset|
+++++++++++++++++++++++++++++++
|      1       |      10      |
-------------------------------
|      1       |      30      |
-------------------------------
|      2       |       0      |
-------------------------------
|      1       |      30      |
-------------------------------
|      2       |       0      |
-------------------------------
|      1       |      10      |
-------------------------------
|      1       |      30      |
-------------------------------
|      2       |       0      |
-------------------------------
|      1       |      30      |
-------------------------------
|      2       |       0      |
+++++++++++++++++++++++++++++++

8.6.1.3.2 Syntax

aligned(8) class CompositionOffsetBox extends FullBox(‘ctts’, version = 0, 0) {
    unsigned int(32) entry_count;
    int i;
    if (version==0) {
        for (i=0; i < entry_count; i++) {
            unsigned int(32)  sample_count;
            unsigned int(32)  sample_offset;
        }
    } else if (version == 1) {
        for (i=0; i < entry_count; i++) {
            unsigned int(32)  sample_count;
            signed   int(32)  sample_offset;
        }
    }
}

8.6.1.3.3 Semantics

version - is an integer that specifies the version of this box.
entry_count is an integer that gives the number of entries in the following table.
sample_count is an integer that counts the number of consecutive samples that have the given offset.
    sample_offset is an integer that gives the offset between CT and DT,
    such that CT(n) = DT(n) + CTTS(n).

**/

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
            let sample_count: u32 = f.read_u32().unwrap();
            let mut sample_offset: i32 = 0;

            if version == 0u8 {
                sample_offset = f.read_u32().unwrap() as i32;
            } else {
                sample_offset = f.read_i32().unwrap();
            }

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
        f.seek(curr_offset + header.data_size);
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
        f.seek(curr_offset + header.data_size);
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
        f.seek(curr_offset + header.data_size);
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
        f.seek(curr_offset + header.data_size);
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

/**
Box Type : ‘mehd’
Container: Movie Extends Box(‘mvex’)
Mandatory: No
Quantity : Zero or one

The Movie Extends Header is optional, and provides the overall duration,
including fragments, of a fragmented movie. If this box is not present,
the overall duration must be computed by examining each fragment.

aligned(8) class MovieExtendsHeaderBox extends FullBox(‘mehd’, version, 0) {
    if (version==1) {
        unsigned int(64)  fragment_duration;
   } else { // version==0
        unsigned int(32)  fragment_duration;
   }
}
**/

#[derive(Debug, Clone)]
pub struct Mehd {
    pub header: Header,
    pub fragment_duration: u64,
}

impl Mehd {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);
        // let curr_offset = f.offset();
        let mut fragment_duration: u64 = 0;
        if header.version.unwrap() == 1u8 {
            fragment_duration = f.read_u64().unwrap();
        } else {
            fragment_duration = u64::from(f.read_u32().unwrap());
        }
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
        f.seek(curr_offset + header.data_size);
        f.offset_inc(header.data_size);
        Ok(Self { header })
    }
}
