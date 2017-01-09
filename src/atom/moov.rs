

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

use std::mem;
use ::Matrix;
use super::{Mp4File, Kind, Header, Atom};

/**

Box Type : ‘moov’
Container: File
Mandatory: Yes
Quantity : Exactly one

The metadata for a presentation is stored in the single Movie Box 
which occurs at the top-level of a file. Normally this box is close 
to the beginning or end of the file, though this is not required.


**/

#[derive(Debug, Clone)]
pub struct Moov {
    header: Header,
    children : Vec<Atom>

}

impl Moov {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str>{
        let children: Vec<Atom> = Atom::parse_children(f);
        Ok(Moov{
            header: header,
            children: children
        })
    }
}


/**

Box Type : ‘mvhd’
Container: Movie Box (‘moov’)
Mandatory: Yes
Quantity : Exactly one

This box defines overall information which is media-independent,
and relevant to the entire presentation considered as a whole.

aligned(8) class MovieHeaderBox extends FullBox(‘mvhd’, version, 0) {
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
    template int(32) rate = 0x00010000; // typically 1.0
    template int(16) volume = 0x0100;   // typically, full volume
    const bit(16) reserved = 0;
    const unsigned int(32)[2] reserved = 0;
    // Unity matrix
    template int(32)[9] matrix = { 0x00010000,0,0,0,0x00010000,0,0,0,0x40000000 };
    bit(32)[6]  pre_defined = 0;
    unsigned int(32)  next_track_ID;
}

8.2.2.3 Semantics

`version` is an integer that specifies the version of this box (0 or 1 in this specification)
`creation_time` is an integer that declares the creation time of the 
    presentation (in seconds since midnight, Jan. 1, 1904, in UTC time)
`modification_time` is an integer that declares the most recent time the
     presentation was modified (in seconds since midnight, Jan. 1, 1904, in UTC time)
`timescale` is an integer that specifies the time-scale for the entire presentation;
    this is the number of time units that pass in one second. For example, 
    a time coordinate system that measures time in sixtieths of a second has a time scale of 60.
`duration` is an integer that declares length of the presentation (in the indicated timescale). 
    This property is derived from the presentation’s tracks: 
    the value of this field corresponds to the duration of the longest track in 
    the presentation. If the duration cannot be determined then duration is set 
    to all 1s.
`rate` is a fixed point 16.16 number that indicates the preferred rate to play 
    the presentation; 1.0 (0x00010000) is normal forward playback

`volume` is a fixed point 8.8 number that indicates the preferred playback volume. 
    1.0 (0x0100) is full volume.
`matrix` provides a transformation matrix for the video; (u,v,w) are restricted here to (0,0,1), 
    hex values (0,0,0x40000000).
`next_track_ID` is a non-zero integer that indicates a value to use for the track ID of 
    the next track to be added to this presentation. Zero is not a valid track ID value. 
    The value of next_track_ID shall be larger than the largest track-ID in use. 
    If this value is equal to all 1s (32-bit maxint), and a new media track is to be added, 
    then a search must be made in the file for an unused track identifier.
**/

#[derive(Debug, Clone)]
pub struct Mvhd {
    header: Header,
    creation_time: u64,
    modification_time: u64,
    timescale: u32,
    duration: u64,

    rate: f64,
    volume: f64,
    matrix: Matrix,
    next_track_id: u32
}

impl Mvhd {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str>{
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
            creation_time = f.read_u32().unwrap() as u64;
            modification_time = f.read_u32().unwrap() as u64;
            timescale = f.read_u32().unwrap();
            duration = f.read_u32().unwrap() as u64;
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

        f.seek(curr_offset+length);
        // matrix
        let matrix: Matrix = f.read_matrix().unwrap(); // 36 Bytes
        length += 36;

        // 24 Bytes
        length += 24;
        f.seek(curr_offset+length);

        let next_track_id = f.read_u32().unwrap();
        length += 4;

        f.offset_inc(length);

        Ok(Mvhd{
            header: header,
            creation_time: creation_time,
            modification_time: modification_time,
            timescale: timescale,
            duration: duration,

            rate: rate,
            volume: volume,
            matrix: matrix,
            next_track_id: next_track_id
        })
    }

}

/**

Box Type : ‘trak’
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

**/

#[derive(Debug, Clone)]
pub struct Trak {
    header: Header,
    children: Vec<Atom>
}

impl Trak {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str>{
        let children: Vec<Atom> = Atom::parse_children(f);
        Ok(Trak{
            header: header,
            children: children
        })
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
Server hint tracks should have the `track_in_movie` and `track_in_preview` set to 0, 
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

`version` is an integer that specifies the version of this box (0 or 1 in this specification) 
`flags` is a 24-bit integer with flags; the following values are defined:
            Track_enabled: Indicates that the track is enabled.
                Flag value is 0x000001. 
                A disabled track (the low bit is zero) is treated as if it were not present.
            Track_in_movie: Indicates that the track is used in the presentation. 
                Flag value is 0x000002.
            Track_in_preview: Indicates that the track is used when previewing 
                the presentation. Flag value is 0x000004.

`creation_time` is an integer that declares the creation time of 
    this track (in seconds since midnight, Jan. 1, 1904, in UTC time)
`modification_time` is an integer that declares the most recent time 
    the track was modified (in seconds since midnight, Jan. 1, 1904, in UTC time)
`track_ID` is an integer that uniquely identifies this track over 
    the entire life-time of this presentation.
    Track IDs are never re-used and cannot be zero.
`duration` is an integer that indicates the duration of 
    this track (in the timescale indicated in the Movie Header Box). 
    The value of this field is equal to the sum of the durations 
    of all of the track’s edits. If there is no edit list, 
    then the duration is the sum of the sample durations, 
    converted into the timescale in the Movie Header Box. 
    If the duration of this track cannot be determined then duration is set to all 1s.
`layer` specifies the front-to-back ordering of video tracks; 
    tracks with lower numbers are closer to the viewer. 0 is the normal value,
    and -1 would be in front of track 0, and so on.
`alternate_group` is an integer that specifies a group or collection of tracks.
    If this field is 0 there is no information on possible relations to other tracks.
    If this field is not 0, it should be the same for tracks that contain alternate 
    data for one another and different for tracks belonging to different such groups.
    Only one track within an alternate group should be played or streamed at any one time, 
    and must be distinguishable from other tracks in the group via attributes such as bitrate, 
    codec, language, packet size etc. A group may have only one member.
`volume` is a fixed 8.8 value specifying the track's relative audio volume. 
    Full volume is 1.0 (0x0100) and is the normal value. 
    Its value is irrelevant for a purely visual track.
    Tracks may be composed by combining them according to their volume, 
    and then using the overall Movie Header Box volume setting; 
    or more complex audio composition (e.g. MPEG-4 BIFS) may be used.
`matrix` provides a transformation matrix for the video; (u,v,w) are restricted here 
    to (0,0,1), hex (0,0,0x40000000).
`width` and `height` specify the track's visual presentation size as fixed-point 16.16 values. 
    These need not be the same as the pixel dimensions of the images, 
    which is documented in the sample description(s); 
    all images in the sequence are scaled to this size, 
    before any overall transformation of the track represented by the matrix. 
    The pixel dimensions of the images are the default values.
**/

#[derive(Debug, Clone)]
pub struct Tkhd {
    header: Header
    // creation_time: u64,
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
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str>{
        header.parse_version(f);
        header.parse_flags(f);

        let curr_offset = f.offset();
        f.seek(curr_offset+header.data_size);
        f.offset_inc(header.data_size);
        Ok(Tkhd{
            header: header
        })
    }
}

/**
Box Type : `tref`
Container: Track Box(`trak`)
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

`track_ID` is an integer that provides a reference from the containing track 
    to another track in the presentation. track_IDs are never re-used and cannot be equal to zero.
The `reference_type` shall be set to one of the following values, or a value registered 
    or from a derived specification or registration:
        *   `hint` the referenced track(s) contain the original media for this hint track
        *   `cdsc` this track describes the referenced track.
        *   `hind` this track depends on the referenced hint track, i.e., 
             it should only be used if the referenced hint track is used.
        *   `vdep` this track contains auxiliary depth video information for the referenced video track
        *   `vplx` this track contains auxiliary parallax video information for the referenced video track

**/

#[derive(Debug, Clone)]
pub struct Tref {
    header: Header
}

impl Tref {
    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str>{
        let curr_offset = f.offset();
        f.seek(curr_offset+header.data_size);
        f.offset_inc(header.data_size);
        Ok(Tref{
            header: header
        })
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
    header: Header
}

impl Trgr {
    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str>{
        let curr_offset = f.offset();
        f.seek(curr_offset+header.data_size);
        f.offset_inc(header.data_size);
        Ok(Trgr{
            header: header
        })
    }
}

/**
Box Type : `mdia`
Container: Track Box (‘trak’)
Mandatory: Yes
Quantity : Exactly One

**/


#[derive(Debug, Clone)]
pub struct Mdia {
    header: Header,
    children: Vec<Atom>
}

impl Mdia {
    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str>{
        let children: Vec<Atom> = Atom::parse_children(f);
        Ok(Mdia{
            header: header,
            children: children
        })
    }
}

/**
Box Type : `mdhd`
Container: Media Box(`mdia`)
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
    header: Header,
    creation_time: u64,
    modification_time: u64,
    timescale: u32,
    duration: u64,

    language: String
}

impl Mdhd {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str>{
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
            creation_time = f.read_u32().unwrap() as u64;
            modification_time = f.read_u32().unwrap() as u64;
            timescale = f.read_u32().unwrap();
            duration = f.read_u32().unwrap() as u64;
            length += 16;
        }

        // 16 Bytes
        // pad: 1 Bit
        // language: 15 Bit;
        let language = f.read_iso639_code().unwrap(); // 2 Bytes, u16
        length += 2;

        // unsigned int(16) pre_defined = 0;
        length += 2;
        f.seek(curr_offset+length);
        f.offset_inc(length);

        Ok(Mdhd{
            header: header,
            creation_time: creation_time,
            modification_time: modification_time,
            timescale: timescale,
            duration: duration,
            language: language
        })
    }
}


#[derive(Debug, Clone)]
pub struct Hdlr {
    header: Header
}

impl Hdlr {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str>{
        header.parse_version(f);
        header.parse_flags(f);

        let curr_offset = f.offset();
        f.seek(curr_offset+header.data_size);
        f.offset_inc(header.data_size);
        Ok(Hdlr{
            header: header
        })
    }
}

#[derive(Debug, Clone)]
pub struct Minf {
    header: Header,
    children: Vec<Atom> // Box Types: ‘vmhd’, ‘smhd’, ’hmhd’, ‘nmhd’
}

impl Minf {
    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str>{
        let children: Vec<Atom> = Atom::parse_children(f);
        Ok(Minf{
            header: header,
            children: children
        })
    }
}

#[derive(Debug, Clone)]
pub struct Vmhd {
    header: Header,
    graphicsmode: u16,
    opcolor: [u16; 3]
}

impl Vmhd {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str>{
        header.parse_version(f);
        header.parse_flags(f);

        let curr_offset = f.offset();

        let graphicsmode = f.read_u16().unwrap();
        // red, greenm blue
        let opcolor: [u16; 3] = [
            f.read_u16().unwrap(), f.read_u16().unwrap(),
            f.read_u16().unwrap()
        ];

        f.offset_inc(8);

        Ok(Vmhd{
            header: header,
            graphicsmode: graphicsmode,
            opcolor: opcolor
        })
    }
}

#[derive(Debug, Clone)]
pub struct Smhd {
    header: Header,
    balance: f64  // fixed-point 8.8 number
}

impl Smhd {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str>{
        header.parse_version(f);
        header.parse_flags(f);

        let curr_offset = f.offset();

        let balance = f.read_fixed_point(8, 8).unwrap(); // 2 Bytes
        // reserved
        f.read_u16().unwrap();

        f.offset_inc(4);

        Ok(Smhd{
            header: header,
            balance: balance
        })
    }
}

#[derive(Debug, Clone)]
pub struct Hmhd {
    header: Header,
    max_pdu_size: u16,
    avg_pdu_size: u16,
    max_bitrate : u32,
    avg_bitrate : u32
}

impl Hmhd {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str>{
        header.parse_version(f);
        header.parse_flags(f);

        let curr_offset = f.offset();

        let max_pdu_size = f.read_u16().unwrap();
        let avg_pdu_size = f.read_u16().unwrap();
        let max_bitrate = f.read_u32().unwrap();
        let avg_bitrate = f.read_u32().unwrap();
        // reserved
        f.read_u32().unwrap();

        f.offset_inc(16);

        Ok(Hmhd{
            header: header,
            max_pdu_size: max_pdu_size,
            avg_pdu_size: avg_pdu_size,
            max_bitrate: max_bitrate,
            avg_bitrate: avg_bitrate
        })
    }
}

#[derive(Debug, Clone)]
pub struct Nmhd {
    header: Header
}

impl Nmhd {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str>{
        header.parse_version(f);
        header.parse_flags(f);

        Ok(Nmhd{
            header: header
        })
    }
}

#[derive(Debug, Clone)]
pub struct Stbl {
    header: Header,
    children: Vec<Atom>
}

impl Stbl {
    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str>{
        let children: Vec<Atom> = Atom::parse_children(f);
        Ok(Stbl{
            header: header,
            children: children
        })
    }
}

#[derive(Debug, Clone)]
pub struct Stsd {
    header: Header
}

impl Stsd {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str>{
        header.parse_version(f);
        header.parse_flags(f);
        let curr_offset = f.offset();
        f.seek(curr_offset+header.data_size);
        f.offset_inc(header.data_size);
        Ok(Stsd{
            header: header
        })
    }
}

#[derive(Debug, Clone)]
pub struct Stdp {
    header: Header
}

impl Stdp {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str>{
        header.parse_version(f);
        header.parse_flags(f);
        let curr_offset = f.offset();
        f.seek(curr_offset+header.data_size);
        f.offset_inc(header.data_size);
        Ok(Stdp{
            header: header
        })
    }
}

#[derive(Debug, Clone)]
pub struct Stts {
    header: Header
}

impl Stts {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str>{
        header.parse_version(f);
        header.parse_flags(f);
        let curr_offset = f.offset();
        f.seek(curr_offset+header.data_size);
        f.offset_inc(header.data_size);
        Ok(Stts{
            header: header
        })
    }
}

#[derive(Debug, Clone)]
pub struct Ctts {
    header: Header
}

impl Ctts {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str>{
        header.parse_version(f);
        header.parse_flags(f);
        let curr_offset = f.offset();
        f.seek(curr_offset+header.data_size);
        f.offset_inc(header.data_size);
        Ok(Ctts{
            header: header
        })
    }
}

#[derive(Debug, Clone)]
pub struct Cslg {
    header: Header
}

impl Cslg {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str>{
        header.parse_version(f);
        header.parse_flags(f);
        let curr_offset = f.offset();
        f.seek(curr_offset+header.data_size);
        f.offset_inc(header.data_size);
        Ok(Cslg{
            header: header
        })
    }
}

#[derive(Debug, Clone)]
pub struct Stss {
    header: Header
}

impl Stss {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str>{
        header.parse_version(f);
        header.parse_flags(f);
        let curr_offset = f.offset();
        f.seek(curr_offset+header.data_size);
        f.offset_inc(header.data_size);
        Ok(Stss{
            header: header
        })
    }
}

#[derive(Debug, Clone)]
pub struct Stsh {
    header: Header
}

impl Stsh {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str>{
        header.parse_version(f);
        header.parse_flags(f);
        let curr_offset = f.offset();
        f.seek(curr_offset+header.data_size);
        f.offset_inc(header.data_size);
        Ok(Stsh{
            header: header
        })
    }
}

#[derive(Debug, Clone)]
pub struct Sdtp {
    header: Header
}

impl Sdtp {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str>{
        header.parse_version(f);
        header.parse_flags(f);
        let curr_offset = f.offset();
        f.seek(curr_offset+header.data_size);
        f.offset_inc(header.data_size);
        Ok(Sdtp{
            header: header
        })
    }
}


#[derive(Debug, Clone)]
pub struct Mvex {
    header: Header,
    children: Vec<Atom>
}

impl Mvex {
    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str>{
        let children: Vec<Atom> = Atom::parse_children(f);
        Ok(Mvex{
            header: header,
            children: children
        })
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
    header: Header,
    fragment_duration: u64
}

impl Mehd {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str>{
        header.parse_version(f);
        header.parse_flags(f);
        // let curr_offset = f.offset();
        let mut fragment_duration: u64 = 0;
        if header.version == 1 {
            fragment_duration = f.read_u64().unwrap();
        } else {
            fragment_duration = f.read_u32().unwrap() as u64;
        }
        // f.seek(curr_offset+header.data_size);
        f.offset_inc(header.data_size);
        Ok(Mehd{
            header: header,
            fragment_duration: fragment_duration
        })
    }
}

#[derive(Debug, Clone)]
pub struct Trex {
    header: Header
}

impl Trex {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str>{
        header.parse_version(f);
        header.parse_flags(f);
        let curr_offset = f.offset();
        f.seek(curr_offset+header.data_size);
        f.offset_inc(header.data_size);
        Ok(Trex{
            header: header
        })
    }
}

