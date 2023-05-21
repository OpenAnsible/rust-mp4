//! Media Header Box

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, read_version, retref, retval};

/// Declares overall media‐independent information relevant to the characteristics of the media in a track.
#[derive(Debug, Clone)]
pub struct Mdhd {
    /// The header of the atom.
    pub header: Header,

    /// The creation time of the media in this track (in seconds since midnight, Jan. 1, 1904, in UTC time).
    /// Use the [time_to_utc](crate::time_to_utc) function to convert this value to a `DateTime`.
    pub creation_time: u64,

    /// The most recent time the media in this track was modified (in seconds since midnight, Jan. 1, 1904, in UTC time).
    /// Use the [time_to_utc](crate::time_to_utc) function to convert this value to a `DateTime`.
    pub modification_time: u64,

    /// The time‐scale for this media; this is the number of time units that pass in one second.
    /// For example, a time coordinate system that measures time in sixtieths of a second has a time scale of 60.
    pub timescale: u32,

    /// The duration of this media (in the scale of the timescale).
    /// If the duration cannot be determined then duration is set to all 1s (if the value is 32 bits) or all 1s (64 bits).
    /// See the [duration_secs](crate::utils::duration_secs) and [duration_seconds](crate::utils::duration_seconds)
    /// functions to convert this value to seconds.
    pub duration: u64,

    /// The language code for this media. See ISO 639‐2/T for the set of three
    /// character codes. Each character is packed as the difference between its ASCII value and 0x60.
    /// Since the code is confined to being three lower‐case letters, these values are strictly positive.
    pub language: String,
}

impl Mdhd {
    /// Parses the `Mdhd` atom, returning `Self`.
    ///
    /// # Arguments
    ///
    /// - `f` - `Mp4File` to read from.
    /// - `header` - `Header` of the `Mdhd` atom.
    ///
    /// # Returns
    ///
    /// * `Result<Self, &'static str>` - The parsed `Mdhd` atom.
    ///
    /// # Errors
    ///
    /// - Errors if the header version can't be determined.
    /// - Errors if the creation time can't be read from the file.
    /// - Errors if the modification time can't be read from the file.
    /// - Errors if the timescale can't be read from the file.
    /// - Errors if the duration can't be read from the file.
    /// - Errors if the language can't be read from the file.
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        if header.version.is_none() {
            return Err("Unable to determine header version.");
        }

        read_version!(creation_time, u64, f.read_u32(), f.read_u64(), header);
        read_version!(modification_time, u64, f.read_u32(), f.read_u64(), header);
        read_version!(timescale, u32, f.read_u32(), f.read_u32(), header);
        read_version!(duration, u64, f.read_u32(), f.read_u64(), header);

        // 2 Bytes
        // pad: 1 Bit
        // language: 15 Bit;
        let_ok!(
            language,
            f.read_iso639_code(),
            "Unable to read language from ISO639 code."
        );

        f.offset_inc(header.data_size);

        Ok(Self {
            header,
            creation_time,
            modification_time,
            timescale,
            duration,
            language,
        })
    }

    retref!(header, Header);
    retval!(creation_time, u64);
    retval!(modification_time, u64);
    retval!(timescale, u32);
    retval!(duration, u64);
    retref!(language, String);
}
