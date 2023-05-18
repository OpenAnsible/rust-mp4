//! Edit List Atom (`Elst`) and its children.

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref, retval};

/// This box contains an explicit timeline map. Each entry defines part of the track time‐line: by mapping
/// part of the media time‐line, or by indicating ‘empty’ time, or by defining a ‘dwell’, where a single time‐
/// point in the media is held for a period.
///
/// _NOTE: Edits are not restricted to fall on sample times. This means that when entering an edit, it can be necessary
/// to (a) back up to a sync point, and pre‐roll from there and then (b) be careful about the duration of the first
/// sample — it might have been truncated if the edit enters it during its normal duration. If this is audio, that frame
/// might need to be decoded, and then the final slicing done. Likewise, the duration of the last sample in an edit
/// might need slicing._
///
/// Starting offsets for tracks (streams) are represented by an initial empty edit. For example, to play a
/// track from its start for 30 seconds, but at 10 seconds into the presentation, we have the following edit
/// list:
///
/// > Entry‐count = 2
/// >
/// > Segment‐duration = 10 seconds
/// > Media‐Time = ‐1
/// > Media‐Rate = 1
/// >
/// >  Segment‐duration = 30 seconds (could be the length of the whole track)
/// >  Media‐Time = 0 seconds
/// >  Media‐Rate = 1
///
/// A non‐empty edit may insert a portion of the media timeline that is not present in the initial movie, and
/// is present only in subsequent movie fragments. Particularly in an empty initial movie of a fragmented
/// movie file (when there are no media samples yet present), the segment_duration of this edit may be
/// zero, whereupon the edit provides the offset from media composition time to movie presentation time,
/// for the movie and subsequent movie fragments. It is recommended that such an edit be used to
/// establish a presentation time of 0 for the first presented sample, when composition offsets are used.
///
/// For example, if the composition time of the first composed frame is 20, then the edit that maps the
/// media time from 20 onwards to movie time 0 onwards, would read:
/// > Entry‐count = 1
/// >
/// > Segment‐duration = 0
/// > Media‐Time = 20
/// > Media‐Rate = 1
#[derive(Debug, Clone)]
pub struct Elst {
    /// The header of the atom.
    header: Header,

    /// The number of entries in the following table
    entry_count: u32,

    entries: Vec<ElstEntry>,
}

impl Elst {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let entry_count = f.read_u32().unwrap_or(0);

        let mut entries = Vec::new();
        let mut segment_duration;
        let mut media_time;

        for _ in 0..entry_count {
            if header.version() == Some(1) {
                let_ok!(sd, f.read_u64(), "Unable to read segment duration.");
                segment_duration = sd;

                let_ok!(mt, f.read_i64(), "Unable to read media time.");
                media_time = mt;
            } else {
                // version == 0
                let_ok!(sd, f.read_u32(), "Unable to read segment duration.");
                segment_duration = sd as u64;

                let_ok!(mt, f.read_i32(), "Unable to read media time.");
                media_time = mt as i64;
            }

            let_ok!(
                media_rate_integer,
                f.read_u16(),
                "Unable to read media rate."
            );
            let_ok!(
                media_rate_fraction,
                f.read_u16(),
                "Unable to read media rate."
            );

            entries.push(ElstEntry {
                segment_duration,
                media_time,
                media_rate_integer,
                media_rate_fraction,
            });
        }

        Ok(Self {
            header,
            entry_count,
            entries,
        })
    }

    retref!(header, Header);
    retval!(entry_count, u32);
    retref!(entries, Vec<ElstEntry>);
}

#[derive(Debug, Clone)]
pub struct ElstEntry {
    /// The duration of this edit segment in units of the timescale in the Movie Header Box.
    /// If this field is set to 0, it is an empty edit. The last edit in a track shall never be an empty edit.
    segment_duration: u64,

    /// The starting time within the media of this edit segment (in media time scale units, in composition time).
    /// If this field is set to –1, it is an empty edit with implicit duration. The last edit in a track shall never be an empty edit.
    media_time: i64,

    /// A time value that indicates the duration of this edit segment (in media time scale units).
    /// If this field is set to 0, it is an empty edit. The last edit in a track shall never be an empty edit.
    media_rate_integer: u16,

    /// A time value that indicates the duration of this edit segment (in media time scale units).
    /// If this field is set to 0, it is an empty edit. The last edit in a track shall never be an empty edit.
    media_rate_fraction: u16,
}

impl ElstEntry {
    retval!(segment_duration, u64);
    retval!(media_time, i64);
    retval!(media_rate_integer, u16);
    retval!(media_rate_fraction, u16);
}
