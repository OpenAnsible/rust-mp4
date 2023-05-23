//! IPMPInfoBox

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::retref;

/// Contains information about the IPMP (Intellectual Property Management and Protection) of the media.
///
/// The IPMPInfoBox contains IPMP Descriptors which document the protection applied to the stream.
///
/// IPMP_Descriptor is defined in 14496-1. This is a part of the MPEG-4 object descriptors (OD) that describe
/// how an object can be accessed and decoded. Here, in the ISO Base Media File Format, IPMP Descriptor can
/// be carried directly in IPMPInfoBox without the need for OD stream.
///
/// The presence of IPMP Descriptor in this IPMPInfoBox indicates the associated media stream is protected by
/// the IPMP Tool described in the IPMP Descriptor.
///
/// Each IPMP_Descriptor has an IPMP_ToolID, which identifies the required IPMP tool for protection. An
/// independent registration authority (RA) is used so any party can register its own IPMP Tool and identify this
/// without collisions.
///
/// The IPMP_Descriptor carries IPMP information for one or more IPMP Tool instances, it includes but not
/// limited to IPMP Rights Data, IPMP Key Data, Tool Configuration Data, etc.
/// More than one IPMP Descriptors can be carried in this IPMPInfoBox if this media stream is protected by more
/// than one IPMP Tools.
///
/// _Note: This box is defined in ISO/IEC 14496-12 2005, but is not found in later versions of the standard._
///
/// - Box Type: ‘imif’
/// - Container: Protection Scheme Information Box (‘sinf’)
/// - Mandatory: No
/// - Quantity: Exactly One
#[derive(Debug, Clone)]
pub struct Imif {
    /// The header of the atom.
    pub header: Header,

    /// The IPMP_Descriptor contained in this atom.
    pub ipmp_desc: Vec<IpmpDescriptor>,
}

impl Imif {
    /// Parses an `Imif` atom from the given MP4 file.
    ///
    /// The file is advanced to the end of the atom.
    ///
    /// # Arguments
    ///
    /// - `f` - The MP4 file to read from.
    /// - `header` - The header of the atom.
    ///
    /// # Returns
    ///
    /// - `Result<Self, &'static str>` - The parsed `Imif` atom.
    ///
    /// # Errors
    ///
    /// - None at this time since it doesn't actually parse anything.
    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str> {
        // Advance the file offset by the size of the data.
        f.offset_inc(header.data_size);

        Ok(Self {
            header,
            ipmp_desc: Vec::<IpmpDescriptor>::new(),
        })
    }

    // included for completeness, but not used
    retref!(header, Header);
    retref!(ipmp_desc, Vec<IpmpDescriptor>);
}

/// IPMP_Descriptor - defined in 14496-1
#[derive(Debug, Clone)]
pub struct IpmpDescriptor {
    // Currently empty
}
