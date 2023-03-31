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

#[derive(Debug, Clone)]
pub struct Moof {
    header: Header,
    children: Vec<Atom>,
}

impl Moof {
    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str> {
        let children: Vec<Atom> = Atom::parse_children(f);
        Ok(Self { header, children })
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn children(&self) -> &Vec<Atom> {
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

        let sequence_number: u32 = f.read_u32().unwrap();
        f.offset_inc(header.data_size);

        Ok(Self {
            header,
            sequence_number,
        })
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn sequence_number(&self) -> u32 {
        self.sequence_number
    }
}

#[derive(Debug, Clone)]
pub struct Traf {
    header: Header,
    children: Vec<Atom>,
}

impl Traf {
    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str> {
        let children: Vec<Atom> = Atom::parse_children(f);
        Ok(Self { header, children })
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn children(&self) -> &Vec<Atom> {
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

        // let curr_offset = f.offset();
        // f.seek(curr_offset+header.data_size);
        let track_id: u32 = f.read_u32().unwrap();

        let mut base_data_offset: Option<u64> = None;
        let mut sample_description_index: Option<u32> = None;
        let mut default_sample_duration: Option<u32> = None;
        let mut default_sample_size: Option<u32> = None;
        let mut default_sample_flags: Option<u32> = None;

        if header.flags.is_some() {
            let flags = header.flags.unwrap();
            if flags == base_data_offset_present {
                base_data_offset = Some(f.read_u64().unwrap());
            } else if flags == sample_description_index_present {
                sample_description_index = Some(f.read_u32().unwrap());
            } else if flags == default_sample_duration_present {
                default_sample_duration = Some(f.read_u32().unwrap());
            } else if flags == default_sample_size_present {
                default_sample_size = Some(f.read_u32().unwrap());
            } else if flags == default_sample_flags_present {
                default_sample_flags = Some(f.read_u32().unwrap());
                // } else if flags == duration_is_empty {
                //     // Do nothing
                // } else if flags == default_base_is_moof {
                //     // Do nothing
                // } else {
                //     // unknowm
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

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn track_id(&self) -> u32 {
        self.track_id
    }

    pub fn base_data_offset(&self) -> Option<u64> {
        self.base_data_offset
    }

    pub fn sample(&self) -> &Sample {
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
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);
        // let curr_offset = f.offset();

        let data_offset_present: [u8; 3] = [0x00, 0x00, 0x01]; // 0x000001
        let first_sample_flags_present: [u8; 3] = [0x00, 0x00, 0x04]; // 0x000004
        let sample_duration_present: [u8; 3] = [0x00, 0x01, 0x00]; // 0x000100
        let sample_flags_present: [u8; 3] = [0x00, 0x04, 0x00]; // 0x000400
        let sample_composition_time_offsets_present: [u8; 3] = [0x00, 0x08, 0x00]; // 0x000800

        let sample_count: u32 = f.read_u32().unwrap();
        let mut data_offset: Option<i32> = None;
        let mut first_sample_flags: Option<u32> = None;
        let mut samples: Vec<Sample> = Vec::with_capacity(sample_count as usize);

        assert!(header.flags.is_some());

        let flags = header.flags.unwrap();
        if flags == data_offset_present {
            data_offset = Some(f.read_i32().unwrap());
        } else if flags == first_sample_flags_present {
            first_sample_flags = Some(f.read_u32().unwrap());
        }
        // parse samples
        for _ in 0..sample_count {
            let sample_duration = if flags == sample_duration_present {
                Some(f.read_u32().unwrap())
            } else {
                None
            };

            let sample_flags = if flags == sample_flags_present {
                Some(f.read_u32().unwrap())
            } else {
                None
            };

            let sample_composition_time_offset = if flags == sample_composition_time_offsets_present
            {
                if header.version.unwrap() == 0u8 {
                    Some(f.read_u32().unwrap() as i32)
                } else {
                    Some(f.read_i32().unwrap())
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

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn sample_count(&self) -> u32 {
        self.sample_count
    }

    pub fn data_offset(&self) -> Option<i32> {
        self.data_offset
    }

    pub fn first_sample_flags(&self) -> Option<u32> {
        self.first_sample_flags
    }

    pub fn samples(&self) -> &Vec<Sample> {
        &self.samples
    }
}
