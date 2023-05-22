//! IPMP Control

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{generic_parse, retref};

/// May contain IPMP descriptors which may be referenced by any stream in the file.
/// The  IPMP_ToolListDescriptor  is  defined  in  14496-1,  which  conveys  the  list  of  IPMP  tools  required  to  access
/// the media streams in an ISO Base Media File or meta-box, and may include a list of alternate IPMP tools or
/// parametric descriptions of tools required to access the content.
///
/// The presence of IPMP Descriptor in this IPMPControlBox indicates that media streams within the file or meta-
/// box are protected by the IPMP Tool described in the IPMP Descriptor. More than one IPMP Descriptors can
/// be carried here, if there are more than one IPMP Tools providing the global governance.
///
/// - Box Types: `Ipmc`
/// - Container: Movie Box ([Moov](crate::atom::moov::Moov)) or Meta Box ([Meta](crate::atom::meta::Meta))
/// - Mandatory: No
/// - Quantity: Zero or One
///
/// NOTE: This box is defined in ISO/IEC 14496-12 2005, but is not found in later versions of the standard. It uses
/// constructs defined in ISO/IEC 14496-1.
///
/// Currently not implemented beyond being identified and having its header parsed.
#[derive(Debug, Clone)]
pub struct Ipmc {
    /// The header of the atom.
    pub header: Header,
}

impl Ipmc {
    generic_parse!(Ipmc);
    retref!(header, Header);
}
