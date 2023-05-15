//! Media Engine Container Object (`Meco`) atom and its children.

use crate::atom::atom::Atom;
use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{generic_parse_children, retref};

/// Media Engine Container Object (`Meco`) atom.
///
/// The additional metadata container box includes one or more meta boxes.
/// It can be carried at the top level of the file,
/// in the Movie Box (`Moov`), or in the Track Box (`Trak`) and shall
/// only be present if it is accompanied by a meta box in the same container.
/// A meta box that is not contained in the additional metadata container box
/// is the preferred (primary) meta box. Meta boxes in the additional metadata
/// container box complement or give alternative metadata information.
/// The usage of multiple meta boxes may be desirable when,
/// e.g., a single handler is not capable of processing all metadata.
/// All meta boxes at a certain level, including the preferred one and those
/// contained in the additional metadata container box, must have different handler types.
///
/// A meta box contained in an additional metadata container box shall contain a primary
/// Item box or the primary data box required by the handler (e.g., an XML Box).
/// It shall not include boxes or syntax elements concerning items other
/// than the primary item indicated by the present primary item box or XML box.
/// URLs in a meta box contained in an additional metadata container box
/// are relative to the context of the preferred meta box.
///
/// - Box Type: `Meco`
/// - Container: File, Movie Box (`Moov`), or Track Box (`Trak`)
/// - Mandatory: No
/// - Quantity: Zero or one
#[derive(Debug, Clone)]
pub struct Meco {
    /// Header of the `Meco` atom.
    header: Header,

    /// Children of the `Meco` atom; zero or more `Mere` atoms.
    children: Vec<Atom>,
}

impl Meco {
    generic_parse_children!(Meco);
    retref!(header, Header);
    retref!(children, Vec<Atom>);
}
