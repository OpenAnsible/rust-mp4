//! Sample to Group (`sbgp`) atom and its children.
//!
//! Box Type:  ‘sbgp’
//! Container: Sample Table Box (‘stbl’) or Track Fragment Box (‘traf’)
//! Mandatory: No
//! Quantity:  Zero or more.
//!
//! This table can be used to find the group that a sample belongs to and the associated description of that
//! sample group. The table is compactly coded with each entry giving the index of the first sample of a run
//! of samples with the same sample group descriptor. The sample group description ID is an index that
//! refers to a SampleGroupDescription box, which contains entries describing the characteristics of
//! each sample group.
//!
//! There may be multiple instances of this box if there is more than one sample grouping for the samples
//! in a track. Each instance of the SampleToGroup box has a type code that distinguishes different
//! sample groupings. There shall be at most one instance of this box with a particular grouping type in a
//! Sample Table Box or Track Fragment Box. The associated SampleGroupDescription shall indicate
//! the same value for the grouping type.
//!
//! Version 1 of this box should only be used if a grouping type parameter is needed.

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref, retval};

/// Can be used to find the group that a sample belongs to and the associated description of that sample group.
#[derive(Debug, Clone)]
pub struct Sbgp {
    /// The header of the atom.
    pub header: Header,

    /// Identifies the type (i.e. criterion used to form the sample groups) of the
    /// sample grouping and links it to its sample group description table with the same
    /// value  for  grouping  type.  At  most  one  occurrence  of  this  box  wit h  the  same  value  for
    /// grouping_type (and, if used, grouping_type_parameter) shall exist for a track.
    pub grouping_type: u32,

    /// An indication of the sub‐type of the grouping. Only present if header.version == 1
    pub grouping_type_parameter: u32,

    /// The number of entries in the entries table
    pub entry_count: u32,

    /// The actual entries in the table
    pub entries: Vec<SbgpEntry>,
}

impl Sbgp {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let_ok!(grouping_type, f.read_u32(), "Unable to read grouping type.");
        let grouping_type_parameter = if header.version() == Some(1) {
            let_ok!(gtp, f.read_u32(), "Unable to read grouping type parameter.");
            gtp
        } else {
            0
        };
        let_ok!(entry_count, f.read_u32(), "Unable to read entry count.");

        let mut entries: Vec<SbgpEntry> = Vec::new();
        for _entry in 0..entry_count {
            let_ok!(sample_count, f.read_u32(), "Unable to read sample count.");
            let_ok!(
                group_description_index,
                f.read_u32(),
                "Unable to read group description index."
            );

            entries.push(SbgpEntry {
                sample_count,
                group_description_index,
            });
        }

        // Advance the file offset by the size of the data.
        let _offset = f.offset_inc(header.data_size);

        Ok(Self {
            header,
            grouping_type,
            grouping_type_parameter,
            entry_count,
            entries,
        })
    }

    retref!(header, Header);
    retval!(grouping_type, u32);
    retval!(grouping_type_parameter, u32);
    retval!(entry_count, u32);
    retref!(entries, Vec<SbgpEntry>);
}

impl Default for Sbgp {
    fn default() -> Self {
        Self {
            header: Header::default(),
            grouping_type: 0,
            grouping_type_parameter: 0,
            entry_count: 0,
            entries: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SbgpEntry {
    /// Gives the number of consecutive samples with the same sample group descriptor.
    /// If the sum of the sample count in this box is less than the total sample count,
    /// or there is no sample‐to‐group box that applies to some samples (e.g. it is absent from a track
    /// fragment), then the reader should associates the samples that have no explicit group association
    /// with the default group defined in the SampleDescriptionGroup box, if any, or else with no group.
    /// It is an error for the total in this box to be greater than the sample_count documented
    /// elsewhere, and the reader behaviour would then be undefined.
    pub sample_count: u32,

    /// gives the index of the sample group entry which describes the samples in this group.
    /// The index ranges from 1 to the number of sample group entries in the SampleGroupDescription Box,
    /// or takes the value 0 to indicate that this sample is a member of no group of this type.
    pub group_description_index: u32,
}

impl SbgpEntry {
    retval!(sample_count, u32);
    retval!(group_description_index, u32);
}
