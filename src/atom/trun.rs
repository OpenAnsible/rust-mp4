//! Track Fragment Run Box `Trun` atom definition

use super::sample::Sample;
use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, let_some, retref, retval};

/// Track Fragment Run Box `Trun` atom definition
///
/// ISO/IEC 14496-12:2015 § 8.8.8
///
/// Within the Track Fragment Box, there are zero or more Track Run Boxes.
/// If the duration‐is‐empty flag is set in the `tf_flags`, there are no track runs.
/// A track run documents a contiguous set of samples for a track.
///
/// The number of optional fields is determined from the number of bits set in the
/// lower byte of the flags, and the size of a record from the bits set in the second
/// byte of the flags. This procedure shall be followed, to allow for new fields to be defined.
///
/// If the data‐offset is not present, then the data for this run starts immediately after
/// the data of the previous run, or at the base‐data‐offset defined by the track fragment
/// header if this is the first run in a track fragment, If the data‐offset is present,
/// it is relative to the base‐data‐offset established in the track fragment header
///
/// The following flags are defined:
///
/// | Flag | Description |
/// |:---:| --- |
/// | `0x000001` | data‐offset‐present |
/// | `0x000004` | first‐sample‐flags‐present; this over‐rides the default flags for the first sample only. This makes it possible to record a group of frames where the first is a key and the rest are difference frames, without supplying explicit flags for every sample. If this flag and field are used, sample‐ flags shall not be present. |
/// | `0x000100` | sample‐duration‐present: indicates that each sample has its own duration, otherwise the default is used.|
/// | `0x000200` | sample‐size‐present: each sample has its own size, otherwise the default is used.|
/// | `0x000400` | sample‐flags‐present; each sample has its own flags, otherwise the default is used.|
/// | `0x000800` | sample‐composition‐time‐offsets‐present; each sample has a composition time offset (e.g. as used for I/P/B video in MPEG).
///
/// The composition offset values in the composition time‐to‐sample box and in the track run box may be signed or unsigned.
/// The recommendations given in the composition time‐to‐sample box concerning the use of signed composition offsets also apply here.
///
/// - Box Type: `Trun`
/// - Container: Track Fragment Box`Traf`
/// - Mandatory: No
/// - Quantity: Zero or more
#[derive(Debug, Clone)]
pub struct Trun {
    /// The header of the atom.
    header: Header,

    /// The number of samples in the run.
    sample_count: u32,

    // the following are optional fields
    /// The offset from the start of the data of the first sample.
    data_offset: Option<i32>,

    /// The flags for the first sample.
    first_sample_flags: Option<u32>,

    // all fields in the following array are optional
    /// The samples in the run.
    samples: Vec<Sample>,
}

/// Implements relevant functionality for the `Trun` atom.
impl Trun {
    /// Parses the `Trun` atom, returning `Self`.
    ///
    /// # Arguments
    ///
    /// - `f` - `Mp4File` to read from.
    /// - `header` - `Header` of the `Trun` atom.
    ///
    /// # Returns
    ///
    /// * `Result<Self, &'static str>` - The parsed `Trun` atom.
    ///
    /// # Errors
    ///
    /// - `Err` - If the sample count cannot be read from the file.
    /// - `Err` - If the data offset cannot be read from the file.
    /// - `Err` - If the first sample flags cannot be read from the file.
    /// - `Err` - If the sample duration cannot be read from the file.
    /// - `Err` - If the sample flags cannot be read from the file.
    /// - `Err` - If the sample composition time offset cannot be read from the file.
    /// - `Err` - If there are no header flags.
    #[allow(clippy::cast_possible_wrap)]
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        if header.flags.is_none() {
            return Err("No header flags have been set.");
        }

        let data_offset_present: [u8; 3] = [0x00, 0x00, 0x01]; // 0x000001
        let first_sample_flags_present: [u8; 3] = [0x00, 0x00, 0x04]; // 0x000004
        let sample_duration_present: [u8; 3] = [0x00, 0x01, 0x00]; // 0x000100
        let sample_flags_present: [u8; 3] = [0x00, 0x04, 0x00]; // 0x000400
        let sample_composition_time_offsets_present: [u8; 3] = [0x00, 0x08, 0x00]; // 0x000800

        let_ok!(sample_count, f.read_u32(), "Unable to read sample count");

        let mut data_offset: Option<i32> = None;
        let mut first_sample_flags = None;
        let mut samples: Vec<Sample> = Vec::with_capacity(sample_count as usize);

        let_some!(flags, header.flags, "Header flags not set.");

        if flags == data_offset_present {
            let_ok!(doffs, f.read_i32(), "Unable to read data offset.");
            data_offset = Some(doffs);
        } else if flags == first_sample_flags_present {
            let_ok!(fsf, f.read_u32(), "Unable to read first sample flags.");
            first_sample_flags = Some(fsf);
        }
        // parse samples
        for _ in 0..sample_count {
            let sample_duration = if flags == sample_duration_present {
                let_ok!(sd, f.read_u32(), "Unable to read sample duration.");
                Some(sd)
            } else {
                None
            };

            let sample_flags = if flags == sample_flags_present {
                let_ok!(sf, f.read_u32(), "Unable to read sample flags.");
                Some(sf)
            } else {
                None
            };

            let sample_composition_time_offset = if flags == sample_composition_time_offsets_present
            {
                if header.version.unwrap_or(u8::MAX) == 0u8 {
                    let_ok!(
                        scto,
                        f.read_u32(),
                        "Unable to read sample composition time offset (u32)."
                    );
                    Some(scto as i32)
                } else {
                    let_ok!(
                        scto,
                        f.read_i32(),
                        "Unable to read sample composition time offset (i32)."
                    );
                    Some(scto)
                }
            } else {
                None
            };

            let sample_description_index = None;
            samples.push(Sample {
                duration: sample_duration,
                size: sample_flags,
                flags: sample_flags,
                composition_time_offset: sample_composition_time_offset,
                description_index: sample_description_index,
            });
        }
        // f.seek(curr_offset+header.data_size);

        f.offset_inc(header.data_size);

        log::trace!("Trun::parse() -- header = {header:?}, sample_count = {sample_count}, data_offset = {data_offset:?}, first_sample_flags = {first_sample_flags:?}, samples = {samples:?}");

        Ok(Self {
            header,
            sample_count,
            data_offset,
            first_sample_flags,
            samples,
        })
    }

    retref!(header, Header);
    retval!(sample_count, u32);
    retval!(data_offset, Option<i32>);
    retval!(first_sample_flags, Option<u32>);
    retref!(samples, Vec<Sample>);
}
