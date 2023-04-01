/**

moof
    mfhd
    traf
        tfhd
        trun
        sdtp
        sbgp
        subs

**/
use super::{Atom, Header, Mp4File, Sample};
use crate::{let_ok, let_some};

#[derive(Debug, Clone)]
pub struct Moof {
    header: Header,
    children: Vec<Atom>,
}

impl Moof {
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

#[derive(Debug, Clone)]
pub struct Mfhd {
    header: Header,
    sequence_number: u32,
}

impl Mfhd {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let_ok!(
            sequence_number,
            f.read_u32(),
            "Unable to read sequence number."
        );
        let _offset = f.offset_inc(header.data_size);

        Ok(Self {
            header,
            sequence_number,
        })
    }

    pub const fn header(&self) -> &Header {
        &self.header
    }

    pub const fn sequence_number(&self) -> u32 {
        self.sequence_number
    }
}

#[derive(Debug, Clone)]
pub struct Traf {
    header: Header,
    children: Vec<Atom>,
}

impl Traf {
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

    pub const fn header(&self) -> &Header {
        &self.header
    }

    pub const fn track_id(&self) -> u32 {
        self.track_id
    }

    pub const fn base_data_offset(&self) -> Option<u64> {
        self.base_data_offset
    }

    pub const fn sample(&self) -> &Sample {
        &self.sample
    }
}

#[derive(Debug, Clone)]
pub struct Trun {
    header: Header,
    sample_count: u32,
    // the following are optional fields
    data_offset: Option<i32>,
    first_sample_flags: Option<u32>,
    // all fields in the following array are optional
    samples: Vec<Sample>,
}

impl Trun {
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
        Ok(Self {
            header,
            sample_count,
            data_offset,

            first_sample_flags,
            samples,
        })
    }

    pub const fn header(&self) -> &Header {
        &self.header
    }

    pub const fn sample_count(&self) -> u32 {
        self.sample_count
    }

    pub const fn data_offset(&self) -> Option<i32> {
        self.data_offset
    }

    pub const fn first_sample_flags(&self) -> Option<u32> {
        self.first_sample_flags
    }

    pub const fn samples(&self) -> &Vec<Sample> {
        &self.samples
    }
}
