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
use super::{Atom, Header, Kind, Mp4File, Sample};

#[derive(Debug, Clone)]
pub struct Moof {
    header: Header,
    children: Vec<Atom>,
}

impl Moof {
    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str> {
        let children: Vec<Atom> = Atom::parse_children(f);
        Ok(Self {
            header,
            children,
        })
    }
}

/**


aligned(8) class `MovieFragmentHeaderBox` extends FullBox(‘mfhd’, 0, 0){
   unsigned int(32)  `sequence_number`;
}

**/

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
}

/**
8.8.7 Track Fragment Header Box
8.8.7.1 Definition
Box Type : ‘tfhd’
Container: Track Fragment Box ('traf')
Mandatory: Yes
Quantity : Exactly one

Each movie fragment can add zero or more fragments to each track;
and a track fragment can add zero or more contiguous runs of samples.
The track fragment header sets up information and defaults used for those runs of samples.

The following flags are defined in the `tf_flags`:
    0x000001 base-data-offset-present: indicates the presence of the base-data-offset field.
             This provides an explicit anchor for the data offsets in each track run (see below).
             If not provided, the base-data-offset for the first track in the movie fragment is
             the position of the first byte of the enclosing Movie Fragment Box, and for second
             and subsequent track fragments, the default is the end of the data defined by
             the preceding fragment. Fragments 'inheriting' their offset in this way must all use
             the same data-reference (i.e., the data for these tracks must be in the same file).
    0x000002 sample-description-index-present: indicates the presence of this field, which over-rides,
             in this fragment, the default set up in the Track Extends Box.
    0x000008 default-sample-duration-present
    0x000010 default-sample-size-present
    0x000020 default-sample-flags-present
    0x010000 duration-is-empty: this indicates that the duration provided in either default-sample-duration,
             or by the default-duration in the Track Extends Box, is empty, i.e. that there are no samples
             for this time interval. It is an error to make a presentation that has both edit lists in
             the Movie Box, and empty- duration fragments.
    0x020000 default-base-is-moof: if base-data-offset-present is zero, this indicates that the base-data-offset
             for this track fragment is the position of the first byte of the enclosing Movie Fragment Box.
             Support for the default-base-is-moof flag is required under the ‘iso5’ brand, and it shall not
             be used in brands or compatible brands earlier than iso5.

NOTE:
    The use of the default-base-is-moof flag breaks the compatibility to earlier brands of the file format,
    because it sets the anchor point for offset calculation differently than earlier. Therefore,
    the default-base-is-moof flag cannot be set when earlier brands are included in the File Type box.

8.8.7.2 Syntax
aligned(8) class `TrackFragmentHeaderBox` extends FullBox(‘tfhd’, 0, `tf_flags`){
    unsigned int(32) `track_ID`;
    // all the following are optional fields
    unsigned int(64) `base_data_offset`;
    unsigned int(32) `sample_description_index`;
    unsigned int(32) `default_sample_duration`;
    unsigned int(32) `default_sample_size`;
    unsigned int(32) `default_sample_flags`
}

8.8.7.3 Semantics
`base_data_offset` the base offset to use when calculating data offsets
**/

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
}

/**

8.8.8.1 Definition
Box Type : ‘trun’
Container: Track Fragment Box ('traf')
Mandatory: No
Quantity : Zero or more


Within the Track Fragment Box, there are zero or more Track Run Boxes.
If the duration-is-empty flag is set in the `tf_flags`, there are no track runs.
A track run documents a contiguous set of samples for a track.

The number of optional fields is determined from the number of bits set in the lower byte of the flags,
and the size of a record from the bits set in the second byte of the flags. This procedure shall be followed,
to allow for new fields to be defined.

If the data-offset is not present, then the data for this run starts immediately after
the data of the previous run, or at the base-data-offset defined by the track fragment header
if this is the first run in a track fragment, If the data-offset is present, it is relative
to the base-data-offset established in the track fragment header.
The following flags are defined:

    0x000001 data-offset-present.
    0x000004 first-sample-flags-present; this over-rides the default flags for the first sample only.
            This makes it possible to record a group of frames where the first is a key and the rest
            are difference frames, without supplying explicit flags for every sample. If this flag
            and field are used, sample-flags shall not be present.
    0x000100 sample-duration-present: indicates that each sample has its own duration, otherwise the default is used.
    0x000200 sample-size-present: each sample has its own size, otherwise the default is used.
    0x000400 sample-flags-present; each sample has its own flags, otherwise the default is used.
    0x000800 sample-composition-time-offsets-present;
             each sample has a composition time offset (e.g. as used for I/P/B video in MPEG).

The composition offset values in the composition time-to-sample box and in the track run box may be signed
or unsigned. The recommendations given in the composition time-to-sample box concerning the use of
signed composition offsets also apply here.


aligned(8) class `TrackRunBox` extends FullBox(‘trun’, version, `tr_flags`) {
    unsigned int(32) `sample_count`;
    // the following are optional fields
    signed int(32) `data_offset`;
    unsigned int(32) `first_sample_flags`;
    // all fields in the following array are optional
    {
      unsigned int(32)  `sample_duration`;
      unsigned int(32)  `sample_size`;
      unsigned int(32)  `sample_flags`
      if (version == 0) {
            unsigned int(32) `sample_composition_time_offset`;
      } else {
            signed int(32) `sample_composition_time_offset`;
      }[ `sample_count` ]
    }
}

`sample_count` the number of samples being added in this run;
    also the number of rows in the following table (the rows can be empty)
`data_offset` is added to the implicit or explicit `data_offset` established in the track fragment header.
`first_sample_flags` provides a set of flags for the first sample only of this run.

**/

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
}
