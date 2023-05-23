//! Track Fragment Header atom definition

use super::sample::Sample;
use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{flag, let_ok, retref, retval};

#[derive(Debug, Clone)]
pub struct Tfhd {
    /// The atom header
    header: Header,

    /// Track ID
    track_id: u32,
    // all the following are optional fields
    /// Base data offset
    base_data_offset: Option<u64>,

    /// Sample
    sample: Sample,
}

/// Each movie fragment can add zero or more fragments to each track; and a track fragment can add zero
/// or more contiguous runs of samples. The track fragment header sets up information and defaults used
/// for those runs of samples.
///
/// The base‐data‐offset, if explicitly provided, is a data offset that is identical to a chunk offset in the Chunk
/// Offset Box, i.e. applying to the complete file (e.g. starting with a file‐type box and movie box). In
/// circumstances when the complete file does not exist or its size is unknown, it may be impossible to use
/// an explicit base‐data‐offset; then, offsets need to be established relative to the movie fragment.
///
/// The following flags are defined in the `tf_flags`:
///
/// - `0x000001 base‐data‐offset‐present`: indicates the presence of the `base‐data‐offset` field. This
/// provides an explicit anchor for the data offsets in each track run (see below). If not provided and
/// if the `default‐base‐is‐moof` flag is not set, the `base‐data‐offset` for the first track in the movie
/// fragment is the position of the first byte of the enclosing Movie Fragment Box, and for second
/// and subsequent track fragments, the default is the end of the data defined by the preceding
/// track fragment. Fragments 'inheriting' their offset in this way must all use the same data‐
/// reference (i.e., the data for these tracks must be in the same file)
/// - `0x000002 sample‐description‐index‐present`: indicates the presence of this field, which overrides,
/// in this fragment, the default set up in the Track Extends Box.
/// - `0x000008 default‐sample‐duration‐present`
/// - `0x000010 default‐sample‐size‐present`
/// - `0x000020 default‐sample‐flags‐present`
/// - `0x010000 duration‐is‐empty`: this indicates that the duration provided in either `default‐sample‐duration`,
/// or by the default‐duration in the Track Extends Box, is empty, i.e. that there are no
/// samples for this time interval. It is an error to make a presentation that has both edit lists in the
/// Movie Box, and `empty‐duration` fragments.
/// - `0x020000 default‐base‐is‐moof`: if `base‐data‐offset‐present` is `1`, this flag is ignored. If `base‐data‐offset‐present`
/// is zero, this indicates that the base‐data‐offset for this track fragment is the position of
/// the first byte of the enclosing Movie Fragment Box. Support for the `default‐base‐is‐moof`
/// flag is required under the ‘iso5’ brand, and it shall not be used in brands or compatible
/// brands earlier than iso5.
///
/// _NOTE: The use of the `default‐base‐is‐moof` flag breaks the compatibility to earlier brands of the file format, because it
/// sets the anchor point for offset calculation differently than earlier. Therefore, the `default‐base‐is‐moof` flag cannot be
/// set when earlier brands are included in the File Type box._
impl Tfhd {
    /// Parses a `Tfhd` atom from the given file. The header is already parsed and passed in.
    /// The file is advanced to the end of the atom.
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let_ok!(track_id, f.read_u32(), "Unable to read track ID");

        let mut base_data_offset = None;
        let mut sample_description_index = None;
        let mut default_sample_duration = None;
        let mut default_sample_size = None;
        let mut default_sample_flags = None;

        if header.flags.is_some() {
            if base_data_offset_is_present(&header) {
                let_ok!(bdo, f.read_u64(), "Unable to read base data offset.");
                base_data_offset = Some(bdo);
            } else if sample_description_index_is_present(&header) {
                let_ok!(
                    sdi,
                    f.read_u32(),
                    "Unable to read sample description index."
                );
                sample_description_index = Some(sdi);
            } else if default_sample_duration_is_present(&header) {
                let_ok!(dsd, f.read_u32(), "Unable to read default sample duration.");
                default_sample_duration = Some(dsd);
            } else if default_sample_size_is_present(&header) {
                let_ok!(
                    dssp,
                    f.read_u32(),
                    "Unable to read default sample size present."
                );
                default_sample_size = Some(dssp);
            } else if default_sample_flags_is_present(&header) {
                let_ok!(dsf, f.read_u32(), "Unable to read default sample flags.");
                default_sample_flags = Some(dsf);
            }
        }

        f.offset_inc(header.data_size);

        log::trace!("Tfhd::parse() -- header = {header:?}, track_id = {track_id}, base_data_offset = {base_data_offset:?}, default_sample_duration = {default_sample_duration:?}, flags = {default_sample_flags:?}, description_index = {sample_description_index:?}");

        Ok(Self {
            header,
            track_id,
            base_data_offset,
            sample: Sample {
                duration: default_sample_duration,
                size: default_sample_size,
                flags: default_sample_flags,
                composition_time_offset: None,
                description_index: sample_description_index,
            },
        })
    }

    retref!(header, Header);
    retval!(track_id, u32);
    retval!(base_data_offset, Option<u64>);
    retref!(sample, Sample);

    flag!(base_data_offset_present, 0x000001);
    flag!(sample_description_index_present, 0x000002);
    flag!(default_sample_duration_present, 0x000008);
    flag!(default_sample_size_present, 0x000010);
    flag!(default_sample_flags_present, 0x000020);
    flag!(duration_is_empty, 0x010000);
    flag!(default_base_is_moof, 0x020000);

    /// Returns the flags as a string.
    #[must_use]
    pub fn flags_str(&self) -> String {
        let mut flags = String::new();

        if self.base_data_offset_present() {
            flags.push_str("base_data_offset_present ");
        }

        if self.sample_description_index_present() {
            flags.push_str("sample_description_index_present ");
        }

        if self.default_sample_duration_present() {
            flags.push_str("default_sample_duration_present ");
        }

        if self.default_sample_size_present() {
            flags.push_str("default_sample_size_present ");
        }

        if self.default_sample_flags_present() {
            flags.push_str("default_sample_flags_present ");
        }

        if self.duration_is_empty() {
            flags.push_str("duration_is_empty ");
        }

        if self.default_base_is_moof() {
            flags.push_str("default_base_is_moof ");
        }

        flags
    }
}

/// Returns `true` if the base data offset is present in the supplied header.
#[must_use]
fn base_data_offset_is_present(header: &Header) -> bool {
    header.flags_to_u32() & 0x000001 == 0x000001
}

/// Returns `true` if the sample description index is present in the supplied header.
#[must_use]
fn sample_description_index_is_present(header: &Header) -> bool {
    header.flags_to_u32() & 0x000002 == 0x000002
}

/// Returns `true` if the default sample duration is present in the supplied header.
#[must_use]
fn default_sample_duration_is_present(header: &Header) -> bool {
    header.flags_to_u32() & 0x000008 == 0x000008
}

/// Returns `true` if the default sample size is present in the supplied header.
#[must_use]
fn default_sample_size_is_present(header: &Header) -> bool {
    header.flags_to_u32() & 0x000010 == 0x000010
}

/// Returns `true` if the default sample flags is present in the supplied header.
#[must_use]
fn default_sample_flags_is_present(header: &Header) -> bool {
    header.flags_to_u32() & 0x000020 == 0x000020
}
