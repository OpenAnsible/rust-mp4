//! This box specifies the characteristics of a single track. Exactly one Track Header Box is contained in a track.
//!
//! In the absence of an edit list, the presentation of a track starts at the beginning of the overall
//! presentation. An empty edit is used to offset the start time of a track.
//!
//! The default value of the track header flags for media tracks is 7 (track_enabled, track_in_movie,
//! track_in_preview). If in a presentation all tracks have neither track_in_movie nor track_in_preview set,
//! then all tracks shall be treated as if both flags were set on all tracks. Server hint tracks should have the
//! `track_in_movie` and `track_in_preview` set to 0, so that they are ignored for local playback and preview.
//!
//! Under the ‘iso3’ brand or brands that share its requirements, the width and height in the track header
//! are measured on a notional 'square' (uniform) grid. Track video data is normalized to these dimensions
//! (logically) before any transformation or placement caused by a layup or composition system. Track
//! (and movie) matrices, if used, also operate in this uniformly‐scaled space.
//!
//! The duration field here does not include the duration of following movie fragments, if any, but only of
//! the media in the enclosing Movie Box. The Movie Extends Header box may be used to document the
//! duration including movie fragments, when desired and possible.

use crate::atom::header::Header;
use crate::matrix::Matrix;
use crate::mp4file::Mp4File;
use crate::utils::time_to_utc;
use crate::{let_ok, retref, retval};

/// Represents the Tkhd atom, which contains the track header information, as per ISO/IEC 14496-12:2015 § 8.3.2.
/// This atom is required to be present in a valid MP4 file. It is a full atom, so it has a version and flags.
/// The version determines the size of the `creation_time` and `modification_time` fields, which are 32-bit or 64-bit.
///
/// The following flags are deefined for `Tkhd`:
///
/// - `Track_enabled`: Indicates that the track is enabled. Flag value is `0x000001`. A disabled track (the
/// low bit is zero) is treated as if it were not present.
/// - `Track_in_movie`: Indicates that the track is used in the presentation. Flag value is `0x000002`.
/// - `Track_in_preview`: Indicates that the track is used when previewing the presentation. Flag value
/// is `0x000004`.
/// - `Track_size_is_aspect_ratio`: Indicates that the width and height fields are not expressed in
/// pixel units. The values have the same units but these units are not specified. The values are
/// only an indication of the desired aspect ratio. If the aspect ratios of this track and other
/// related tracks are not identical, then the respective positioning of the tracks is undefined,
/// possibly defined by external contexts. Flag value is `0x000008`.
#[derive(Debug, Clone)]
pub struct Tkhd {
    /// The header, as defined in the [Header](crate::atom::header::Header) struct.
    header: Header,

    /// The creation time of this track (in seconds since midnight, Jan. 1, 1904, in UTC time).
    creation_time: u64,

    /// The most recent time the track was modified (in seconds since midnight, Jan. 1, 1904, in UTC time).
    modification_time: u64,

    /// An integer that uniquely identifies this track over the entire life of the presentation.
    /// Track IDs are never re-used, and cannot be zero.
    track_id: u32,

    /// Not currently used. Should always be 0.
    reserved1: u32,

    /// The duration of this track (in the timescale indicated in the Movie Header Box).
    /// The value of this field is equal to the sum of the durations of all of the track’s edits.
    /// If there is no edit list, then the duration is the sum of the sample durations, converted into
    /// the timescale in the Movie Header Box. If the duration of this track cannot be determined then
    /// duration is set to all 1s.
    duration: u64,

    /// Not currently used. Should always be 0.
    reserved2: u32,

    /// Specifies the front‐to‐back ordering of video tracks; tracks with lower numbers are closer
    /// to the viewer. 0 is the normal value, and ‐1 would be in front of track 0, and so on.
    layer: i16,

    /// Specifies a group or collection of tracks. If this field is 0
    /// there is no information on possible relations to other tracks. If this field is not 0, it should be the
    /// same for tracks that contain alternate data for one another and different for tracks belonging to
    /// different such groups. Only one track within an alternate group should be played or streamed at
    /// any one time, and must be distinguishable from other tracks in the group via attributes such as
    /// bitrate, codec, language, packet size etc. A group may have only one member.
    alternate_group: i16,

    /// A fixed 8.8 value specifying the track's relative audio volume. Full volume is 1.0
    /// (0x0100) and is the normal value. Its value is irrelevant for a purely visual track. Tracks may be
    /// composed by combining them according to their volume, and then using the overall Movie
    /// Header Box volume setting; or more complex audio composition (e.g. MPEG‐4 BIFS) may be used.
    volume: f64, // {if track_is_audio 0x0100 else 0};

    /// Not currently used. Should always be 0.
    reserved3: u16,

    /// Provides a transformation matrix for the video; (u,v,w) are restricted here to (0,0,1), hex
    /// (0,0,0x40000000).
    matrix: Matrix,

    /// Fixed-point 16.16 values are track‐dependent as follows:
    ///
    /// For text and subtitle tracks, they may, depending on the coding format, describe the suggested
    /// size of the rendering area. For such tracks, the value 0x0 may also be used to indicate that the
    /// data may be rendered at any size, that no preferred size has been indicated and that the actual
    /// size may be determined by the external context or by reusing the width and height of another
    /// track. For those tracks, the flag track_size_is_aspect_ratio may also be used.
    ///
    /// For non‐visual tracks (e.g. audio), they should be set to zero.
    ///
    /// For all other tracks, they specify the track's visual presentation size. These need not be the same
    /// as the pixel dimensions of the images, which is documented in the sample description(s); all
    /// images in the sequence are scaled to this size, before any overall transformation of the track
    /// represented by the matrix. The pixel dimensions of the images are the default values.
    width: f64,

    /// See the description of `width` above.
    height: f64,
}

impl Tkhd {
    /// Parses a `Tkhd` atom from the given file. The header is already parsed and passed in.
    /// The header is updated with version and flags information from the file.
    ///
    /// # Arguments
    ///
    /// - `f` - The file to read from.
    /// - `mut header` - The header of the atom. Will be updated with version and flags information.
    ///
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        if header.version.is_none() {
            return Err("Header version is empty. Unable to continue.");
        }

        let creation_time: u64;
        let modification_time: u64;
        let track_id: u32;
        let reserved1: u32;
        let duration: u64;

        if header.version.unwrap_or(0) == 0 {
            let_ok!(ct, f.read_u32(), "Unable to read creation time.");
            creation_time = u64::from(ct);

            let_ok!(mt, f.read_u32(), "Unable to read modification time.");
            modification_time = u64::from(mt);

            let_ok!(tid, f.read_u32(), "Unable to read track ID.");
            track_id = tid;

            let_ok!(r, f.read_u32(), "Unable to read reserved field 1.");
            reserved1 = r;

            let_ok!(dur, f.read_u32(), "Unable to read duration.");
            duration = u64::from(dur);
        } else {
            // header version == 1
            let_ok!(ct, f.read_u64(), "Unable to read creation time.");
            creation_time = ct;

            let_ok!(mt, f.read_u64(), "Unable to read modification time.");
            modification_time = mt;

            let_ok!(tid, f.read_u32(), "Unable to read track ID.");
            track_id = tid;

            let_ok!(r, f.read_u32(), "Unable to read reserved field 1.");
            reserved1 = r;

            let_ok!(dur, f.read_u64(), "Unable to read duration.");
            duration = dur;
        }

        let_ok!(reserved2, f.read_u32(), "Unable to read reserved field 2.");

        let_ok!(layer, f.read_i16(), "Unable to read layer.");

        let_ok!(
            alternate_group,
            f.read_i16(),
            "Unable to read alternate group."
        );
        let_ok!(volume, f.read_f64(), "Unable to read volume.");
        let_ok!(reserved3, f.read_u16(), "Unable to read reserved field 3.");
        let_ok!(matrix, f.read_matrix(), "Unable to read matrix.");
        let_ok!(width, f.read_fixed_point(16, 16), "Unable to read width.");
        let_ok!(height, f.read_fixed_point(16, 16), "Unable to read height.");

        f.offset_inc(header.data_size);

        Ok(Self {
            header,
            creation_time,
            modification_time,
            track_id,
            reserved1,
            duration,
            reserved2,
            layer,
            alternate_group,
            volume,
            reserved3,
            matrix,
            width,
            height,
        })
    }

    /// Returns the creation time as a `chrono::DateTime<chrono::Utc>`.
    #[must_use]
    pub fn creation_time_utc(&self) -> chrono::DateTime<chrono::Utc> {
        time_to_utc(self.creation_time)
    }

    /// Returns the modification time as a `chrono::DateTime<chrono::Utc>`.
    /// This is the most recent time the presentation was modified.
    #[must_use]
    pub fn modification_time_utc(&self) -> chrono::DateTime<chrono::Utc> {
        time_to_utc(self.modification_time)
    }

    // Included for completeness.
    retref!(header, Header);
    retval!(creation_time, u64);
    retval!(modification_time, u64);
    retval!(track_id, u32);
    retval!(reserved1, u32);
    retval!(duration, u64);
    retval!(reserved2, u32);
    retval!(layer, i16);
    retval!(alternate_group, i16);
    retval!(volume, f64);
    retval!(reserved3, u16);
    retref!(matrix, Matrix);
    retval!(width, f64);
    retval!(height, f64);
}
