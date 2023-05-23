//! Protection Scheme Information

use crate::atom::atom::Atom;
use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{generic_parse_children, retref};

/// Contains all the information required both to understand the encryption transform applied and its parameters,
/// and also to find other information such as the kind and location of the key management system.
///
/// It also documents the original (unencrypted) format of the media.
/// The Protection Scheme Information Box is a container Box. It is mandatory in a sample entry
/// that uses a code indicating a protected stream.
///
/// When used in a protected sample entry, this box must contain the original format box to document the
/// original format. At least one of the following signalling methods must be used to identify the protection
/// applied:
///
/// 1. MPEG-4 systems with IPMP: no other boxes, when IPMP descriptors in MPEG-4 systems
/// streams are used;
/// 2. Scheme signalling: a `SchemeTypeBox` ([Schm](crate::atom::schm::Schm)) and `SchemeInformationBox` ([Schi](crate::atom::schi::Schi)]),
/// when these are used (either both must occur, or neither).
///
/// At least one protection scheme information box must occur in a protected sample entry. When more
/// than one occurs, they are equivalent, alternative, descriptions of the same protection. Readers should
/// choose one to process.
///
/// - Box Types: ‘sinf’
/// - Container: Protected Sample Entry, or Item Protection Box ([Ipro](crate::atom::ipro::Ipro))
/// - Mandatory: Yes
/// - Quantity:  One or More
#[derive(Debug, Clone)]
pub struct Sinf {
    /// The header of the atom.
    pub header: Header,

    /// The protection scheme types, which are used to identify the protection scheme.
    /// Can be one of the following:
    ///
    /// - [Schm](crate::atom::schm::Schm) - Scheme Type Box
    /// - [Schi](crate::atom::schi::Schi) - Scheme Information Box
    ///
    /// Note that either none or both of these should occur.
    pub children: Vec<Atom>,
}

impl Sinf {
    generic_parse_children!(Sinf);
    retref!(header, Header);
    retref!(children, Vec<Atom>);
}
