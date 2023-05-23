//! Track Selection (`Tsel`) atom.
//!
//! A typical presentation stored in a file contains one alternate group per media type: one for video, one
//! for audio, etc. Such a file may include several video tracks, although, at any point in time, only one of
//! them should be played or streamed. This is achieved by assigning all video tracks to the same alternate
//! group. (See subclause 8.3.2 for the definition of alternate groups.)
//!
//! All tracks in an alternate group are candidates for media selection, but it may not make sense to switch
//! between some of those tracks during a session. One may for instance allow switching between video
//! tracks at different bitrates and keep frame size but not allow switching between tracks of different
//! frame size. In the same manner it may be desirable to enable selection – but not switching – between
//! tracks of different video codecs or different audio languages.
//!
//! The distinction between tracks for selection and switching is addressed by assigning tracks to switch
//! groups in addition to alternate groups. One alternate group may contain one or more switch groups. All
//! tracks in an alternate group are candidates for media selection, while tracks in a switch group are also
//! available for switching during a session. Different switch groups represent different operation points,
//! such as different frame size, high/low quality, etc.
//!
//! For the case of non‐scalable bitstreams, several tracks may be included in a switch group. The same also
//! applies to non‐layered scalable bitstreams, such as traditional AVC streams.
//!
//! By labelling tracks with attributes it is possible to characterize them. Each track can be labelled with a
//! list of attributes which can be used to describe tracks in a particular switch group or differentiate tracks
//! that belong to different switch groups.
//!
//! **Descriptive attributes:**
//!
//! |Name|Attribute|Description|
//! |----|---------|-----------|
//! |Temporal scalability|`tesc`|The track can be temporally scaled.|
//! |Fine-grain SNR scalability|`fssc`|The track can be scaled in terms of quality.|
//! |Coarse-grain SNR scalability|`cgsc`|The track can be scaled in terms of quality.|
//! |Spatial scalability|`spsc`|The track can be spatially scaled.|
//! |Region-of-interest scalability|`resc`|The track can be region‐of‐interest scaled.|
//! |View scalability|`vwsc`|The track can be scaled in terms of number of views.|
//!
//! **Differentiating Attributes:**
//!
//! |Name|Attribute|Description|
//! |----|---------|-----------|
//! |Codec|`cdec`|Sample Entry (in Sample Description box of media track).|
//! |Screen size|`scsz`|Width and height fields of Visual Sample Entries.|
//! |Max packet size|`mpsz`|`Maxpacketsize` field in RTP Hint Sample Entry.|
//! |Media Type|`mtyp`|`HandlerType` in Handler box (of media track).|
//! |Media language|`mela`|Language field in Media Header box.|
//! |Bitrate|`bitr`|Total size of the samples in the track divided by the duration in the track header box.|
//! |Frame rate|`frar`|Number of samples in the track divided by duration in the track header box.|
//! |Number of views|`nvws`|Number of views in the sub track.|
//!
//! Descriptive attributes characterize the tracks they modify, whereas differentiating attributes
//! differentiate between tracks that belong to the same alternate or switch groups. The pointer of a
//! differentiating attribute indicates the location of the information that differentiates the track from other
//! tracks with the same attribute.

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref, retval};

/// The track selection box is contained in the user data box of the track it modifies.
/// It describes the attributes of the track that are used for track selection.
#[derive(Debug, Clone)]
pub struct Tsel {
    /// The header of the atom.
    header: Header,

    /// The switch group ID. This should always be 0.
    switch_group: i32,

    /// The attribute list.
    attributes: Vec<u32>,
}

impl Tsel {
    /// Parses a `Tsel` atom from the given file. The header is already parsed and passed in.
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let_ok!(switch_group, f.read_i32(), "Unable to read switch group.");

        let mut attributes: Vec<u32> = Vec::new();
        for _n in 0..(header.data_size - 4) {
            let_ok!(attr, f.read_u32(), "Unable to read attribute.");
            attributes.push(attr);
        }

        Ok(Self {
            header,
            switch_group,
            attributes,
        })
    }

    retref!(header, Header);
    retval!(switch_group, i32);
    retref!(attributes, Vec<u32>);

    /// Returns the attributes as a vector of strings. See ISO/IEC 14496-12:2015(E) §8.10.3.5 for a description of the attributes.
    #[must_use]
    pub fn attributes_decoded(&self) -> Vec<String> {
        let mut decoded: Vec<String> = Vec::new();
        for attr in &self.attributes {
            let dec = u32::to_be_bytes(*attr);
            decoded.push(String::from_utf8_lossy(&dec).to_string());
        }

        decoded
    }
}
