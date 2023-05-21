//!

use crate::atom::header::Header;
use crate::matrix::Matrix;
use crate::mp4file::Mp4File;
use crate::utils::mp4_time_to_datetime_local;
use crate::{let_ok, retref, retval};

/// Defines overall information which is media-independent, and relevant to the entire presentation considered as a whole.
///
/// Represents the `Mvhd` atom, which contains the movie header information, as per ISO/IEC 14496-12:2015 § 8.2.2.
///
/// This atom is required to be present in a valid MP4 file. It is a full atom, so it has a version and flags.
/// The version determines the size of the `creation_time` and `modification_time` fields, which are 32-bit or 64-bit.
/// The flags are currently unused.
/// Known children: None
#[derive(Debug, Clone)]
pub struct Mvhd {
    /// The header of the atom.
    pub header: Header,

    /// The creation time of the presentation, in seconds since midnight, Jan. 1, 1904, in UTC time.
    pub creation_time: u64,

    /// The most recent time the presentation was modified, in seconds since midnight, Jan. 1, 1904, in UTC time.
    pub modification_time: u64,

    /// An integer that specifies the time scale for the entire presentation; this is the number of time units that pass in one second.
    pub timescale: u32,

    /// The length of the presentation (in the indicated timescale).
    pub duration: u64,

    /// A fixed-point 16.16 number that indicates the preferred rate to play the presentation; 1.0 (0x00010000) is normal forward playback.
    pub rate: f64,

    /// A fixed-point 8.8 number that indicates the preferred playback volume; 1.0 (0x0100) is full volume.
    pub volume: f64,

    /// A transformation matrix for the video; (u,v,w) are restricted here to (0,0,1).
    /// The matrix is stored in row-major order.
    pub matrix: Matrix,

    /// An integer that indicates a value to use for the track ID of the next track to be added to this presentation.
    pub next_track_id: u32,
}

impl Mvhd {
    /// Parses a `Mvhd` atom from the given file. The header is already parsed and passed in.
    ///
    /// The file is advanced to the end of the atom.
    /// In practice, the function reads the data from the atom and returns it as part of the Mvhd struct.
    // TODO: Refactor. Do we need the `length` variable, or can we use header.data_size?
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

        if header.version.unwrap_or(0) == 0 {
            let_ok!(ct, f.read_u32(), "Unable to read creation time.");
            creation_time = u64::from(ct);

            let_ok!(mt, f.read_u32(), "Unable to read modification time.");
            modification_time = u64::from(mt);

            let_ok!(ts, f.read_u32(), "Unable to read timescale.");
            timescale = ts;

            let_ok!(dur, f.read_u32(), "Unable to read duration.");
            duration = u64::from(dur);

            length += 16;
        } else {
            // header version == 1
            let_ok!(ct, f.read_u64(), "Unable to read creation time.");
            creation_time = ct;

            let_ok!(mt, f.read_u64(), "Unable to read modification time.");
            modification_time = mt;

            let_ok!(ts, f.read_u32(), "Unable to read timescale.");
            timescale = ts;

            let_ok!(dur, f.read_u64(), "Unable to read duration.");
            duration = dur;

            length += 28;
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

    /// Returns the creation time as a `chrono::DateTime<chrono::Local>`.
    pub fn creation_time_local(&self) -> chrono::DateTime<chrono::Local> {
        mp4_time_to_datetime_local(self.creation_time)
    }

    /// Returns the modification time as a `chrono::DateTime<chrono::Local>`.
    /// This is the most recent time the presentation was modified.
    pub fn modification_time_local(&self) -> chrono::DateTime<chrono::Local> {
        mp4_time_to_datetime_local(self.modification_time)
    }

    // These are here for consistency with the other atoms.
    retref!(header, Header);
    retval!(creation_time, u64);
    retval!(modification_time, u64);
    retval!(timescale, u32);
    retval!(duration, u64);
    retval!(rate, f64);
    retval!(volume, f64);
    retref!(matrix, Matrix);
    retval!(next_track_id, u32);
}
