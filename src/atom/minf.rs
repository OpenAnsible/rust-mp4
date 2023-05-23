//! Media Information Box (`Minf`)

use crate::atom::atom::Atom;
use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{generic_parse_children, retref};

/// Contains objects that declare characteristic information of the media in the track. The information
/// declares characteristics of the media in a track. For example, a video track would contain media
/// information about video such as visual dimensions and compression scheme used, while an audio track
/// would contain media information about audio such as sampling rate and compression scheme used.
///
/// There is a different media information header for each track type (corresponding to the media handler‐
/// type); the matching header shall be present, which may be one of those defined in clause 12, or one
/// defined in a derived specification.
///
/// The type of media header is used is determined by the definition of the media type and must match the
/// media handler.
///
/// This box type can contain the following children:
///
/// - Video Media Header Box - [Vmhd](crate::atom::vmhd::Vmhd)
/// - Sound Media Header Box - [Smhd](crate::atom::smhd::Smhd)
/// - Hint Media Header Box - [Hmhd](crate::atom::hmhd::Hmhd)
/// - Null Media Header Box - [Nmhd](crate::atom::nmhd::Nmhd)
/// - Data Information Box - [Dinf](crate::atom::dinf::Dinf)
/// - Sample Table Box - [Stbl](crate::atom::stbl::Stbl)
#[derive(Debug, Clone)]
pub struct Minf {
    pub header: Header,
    pub children: Vec<Atom>, // Box Types: ‘vmhd’, ‘smhd’, ’hmhd’, ‘nmhd’, `dinf’, ‘stbl’
}

impl Minf {
    generic_parse_children!(Minf);
    retref!(header, Header);
    retref!(children, Vec<Atom>);
}
