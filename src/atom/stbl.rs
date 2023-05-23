//! Sample Table Box
//!
//! Box Type:  `Stbl`
//! Container: Media Information Box ([Minf](crate::atom::minf::Minf))
//! Mandatory: Yes
//! Quantity:  Exactly one
//!
//! The sample table contains all the time and data indexing of the media samples in a track. Using the
//! tables here, it is possible to locate samples in time, determine their type (e.g. I‐frame or not), and
//! determine their size, container, and offset into that container.
//!
//! If the track that contains the Sample Table Box references no data, then the Sample Table Box does not
//! need to contain any sub‐boxes (this is not a very useful media track).
//!
//! If the track that the Sample Table Box is contained in does reference data, then the following sub‐boxes
//! are required: Sample Description, Sample Size, Sample To Chunk, and Chunk Offset. Further, the Sample
//! Description Box shall contain at least one entry. A Sample Description Box is required because it
//! contains the data reference index field which indicates which Data Reference Box to use to retrieve the
//! media samples. Without the Sample Description, it is not possible to determine where the media
//! samples are stored. The Sync Sample Box is optional. If the Sync Sample Box is not present, all samples
//! are sync samples.
//!
//! A.7 provides a narrative description of random access using the structures defined in the Sample Table Box.

use crate::atom::atom::Atom;
use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{generic_parse_children, retref};

/// Contains all the time and data indexing of the media samples in a track. Using the
/// tables here, it is possible to locate samples in time, determine their type (e.g. I‐frame or not), and
/// determine their size, container, and offset into that container.
#[derive(Debug, Clone)]
pub struct Stbl {
    /// The header of the atom.
    pub header: Header,

    /// A list of atoms contained in this atom. The following atoms may be found within the `Stbl` atom:
    ///
    /// - [Co64](crate::atom::co64::Co64)
    /// - [Ctts](crate::atom::ctts::Ctts)
    /// - [Padb](crate::atom::padb::Padb)
    /// - [Sbgp](crate::atom::sbgp::Sbgp)
    /// - [Sdtp](crate::atom::sdtp::Sdtp)
    /// - [Sgpd](crate::atom::sgpd::Sgpd)
    /// - [Stco](crate::atom::stco::Stco)
    /// - [Stdp](crate::atom::stdp::Stdp)
    /// - [Stsc](crate::atom::stsc::Stsc)
    /// - [Stsd](crate::atom::stsd::Stsd)
    /// - [Stsh](crate::atom::stsh::Stsh)
    /// - [Stss](crate::atom::stss::Stss)
    /// - [Stsz](crate::atom::stsz::Stsz)
    /// - [Stts](crate::atom::stts::Stts)
    /// - [Stz2](crate::atom::stz2::Stz2)
    /// - [Subs](crate::atom::subs::Subs)
    pub children: Vec<Atom>,
}

impl Stbl {
    generic_parse_children!(Stbl);
    retref!(header, Header);
    retref!(children, Vec<Atom>);
}
