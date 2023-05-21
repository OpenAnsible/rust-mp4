//! Item Protection

use crate::atom::atom::Atom;
use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref, retval};

/// Represents the Item Protection atom, which contains information about the protection of items, as per ISO/IEC 14496-12:2015 § 8.11.1.
///
/// Provides an array of item protection information, for use by the Item Information Box.
///
/// - Box Type:  ‘ipro’
/// - Container:  Meta box (‘meta’)
/// - Mandatory: No
/// - Quantity:  Zero or one
#[derive(Debug, Clone)]
pub struct Ipro {
    /// The header of the atom.
    pub header: Header,

    /// The number of items in the protection scheme information array.
    pub protection_count: u16,

    /// The protection scheme information.
    pub scheme_info: Vec<Atom>,
}

impl Ipro {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let_ok!(
            protection_count,
            f.read_u16(),
            "Unable to read protection count."
        );

        let mut scheme_info: Vec<Atom> = Vec::with_capacity(protection_count as usize);
        for _ in 0..protection_count {
            let entry = Atom::parse(f)?;
            scheme_info.push(entry);
        }

        Ok(Self {
            header,
            protection_count,
            scheme_info,
        })
    }

    retref!(header, Header);
    retval!(protection_count, u16);
    retref!(scheme_info, Vec<Atom>);
}
