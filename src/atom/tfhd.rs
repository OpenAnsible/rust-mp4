//! Track Fragment Header atom definition

use super::sample::Sample;
use super::{Header, Mp4File};
use crate::{let_ok, let_some, retref, retval};

#[derive(Debug, Clone)]
pub struct Tfhd {
    header: Header,
    track_id: u32,
    // all the following are optional fields
    base_data_offset: Option<u64>,
    sample: Sample,
}

impl Tfhd {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let base_data_offset_present: [u8; 3] = [0x00, 0x00, 0x01]; // 0x000001
        let sample_description_index_present: [u8; 3] = [0x00, 0x00, 0x02]; // 0x000002
        let default_sample_duration_present: [u8; 3] = [0x00, 0x00, 0x08]; // 0x000008

        let default_sample_size_present: [u8; 3] = [0x00, 0x00, 0x10]; // 0x000010
        let default_sample_flags_present: [u8; 3] = [0x00, 0x00, 0x20]; // 0x000020
                                                                        // let duration_is_empty: [u8; 3] = [0x01, 0x00, 0x00]; // 0x010000
                                                                        // let default_base_is_moof: [u8; 3] = [0x20, 0x00, 0x00]; //0x020000

        let_ok!(track_id, f.read_u32(), "Unable to read track ID");

        let mut base_data_offset = None;
        let mut sample_description_index = None;
        let mut default_sample_duration = None;
        let mut default_sample_size = None;
        let mut default_sample_flags = None;

        if header.flags.is_some() {
            let_some!(flags, header.flags, "Flags have no value set.");

            if flags == base_data_offset_present {
                let_ok!(bdo, f.read_u64(), "Unable to read base data offset.");
                base_data_offset = Some(bdo);
            } else if flags == sample_description_index_present {
                let_ok!(
                    sdi,
                    f.read_u32(),
                    "Unable to read sample description index."
                );
                sample_description_index = Some(sdi);
            } else if flags == default_sample_duration_present {
                let_ok!(dsd, f.read_u32(), "Unable to read default sample duration.");
                default_sample_duration = Some(dsd);
            } else if flags == default_sample_size_present {
                let_ok!(
                    dssp,
                    f.read_u32(),
                    "Unable to read default sample size present."
                );
                default_sample_size = Some(dssp);
            } else if flags == default_sample_flags_present {
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
}
