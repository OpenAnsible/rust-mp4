//! Defines the header type for atoms. This header type collects standard information about the atom.
//!
//! It mirrors the `Box` class as defined in the ISO/IEC 14496-12 standard (see § 4.2). It is a full atom, so it has a version and flags.
//!
//! Boxes start with a header which gives both size and type. The header permits compact or extended size
//! (32 or 64 bits) and compact or extended types (32 bits or full Universal Unique IDentifiers, i.e. UUIDs).
//! The standard boxes all use compact types (32‐bit) and most boxes will use the compact (32‐bit) size.
//! Typically only the Media Data Box(es) need the 64‐bit size.
//!
//! The size is the entire size of the box, including the size and type header, fields, and all contained boxes.
//! This facilitates general parsing of the file.
//!
//! The definitions of boxes are given in the syntax description language (SDL) defined in MPEG‐4 (see
//! reference in Clause 2). Comments in the code fragments in this specification indicate informative
//! material.
//!
//! The fields in the objects are stored with the most significant byte first, commonly known as network
//! byte order or big‐endian format. When fields smaller than a byte are defined, or fields span a byte
//! boundary, the bits are assigned from the most significant bits in each byte to the least significant. For
//! example, a field of two bits followed by a field of six bits has the two bits in the high order bits of the
//! byte.
//!
//! ```sdl
//! aligned(8) class Box (unsigned int(32) boxtype,
//!      optional unsigned int(8)[16] extended_type) {
//!   unsigned int(32) size;
//!   unsigned int(32) type = boxtype;
//!
//!   if (size==1) {
//!      unsigned int(64) largesize;
//!   } else if (size==0) {
//!      // box extends to end of file
//!   }
//!
//!   if (boxtype==‘uuid’) {
//!      unsigned int(8)[16] usertype = extended_type;
//!   }
//! }
//! ```
//!
//! The semantics of these two fields are:
//!
//! - `size` is an integer that specifies the number of bytes in this box, including all its fields and
//! contained boxes; if size is 1 then the actual size is in the field largesize; if size is 0, then this
//! box is the last one in the file, and its contents extend to the end of the file (normally only used
//! for a Media Data Box)
//! - `type` identifies the box type; standard boxes use a compact type, which is normally four printable
//! characters, to permit ease of identification, and is shown so in the boxes below. User extensions
//! use an extended type; in this case, the type field is set to ‘uuid’.
//!
//! Boxes with an unrecognized type shall be ignored and skipped.
//!
//! Many objects also contain a version number and flags field:
//!
//! ```sdl
//! aligned(8) class FullBox(unsigned int(32) boxtype, unsigned int(8) v, bit(24) f)
//!     extends Box(boxtype) {
//!   unsigned int(8) version = v;
//!   bit(24)         flags = f;
//! }
//! ```
//!
//! The semantics of these two fields are:
//!
//! - `version` is an integer that specifies the version of this format of the box.
//! - `flags` is a map of flags
//!
//! Boxes with an unrecognized version shall be ignored and skipped
use std::cmp::Ordering;

use super::kind::Kind;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref, retval};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    /// The size in bytes of the atom, including the header and the data.
    pub size: u32,

    /// The type of atom. Refer to the `Kind` enum for more information.
    pub kind: Kind, // atom type

    // Optional
    /// The size in bytes of the atom, including the header and the data.
    pub largesize: Option<u64>,

    /// The user type of the atom. This is used for custom atoms, specifically [uuid](crate::atom::uuid))
    pub usertype: Option<[u8; 16]>, // 128 Bits

    /// The version of the atom. This is used to determine how to parse the atom. Example: `0`, which is the default.
    pub version: Option<u8>,

    /// The flags of the atom. This is used to determine how to parse the atom. Example: `[0, 0, 0]`, which is the default.
    /// Other examples: `[0, 0, 1]`, `[0, 0, 3]`, which indicate that the atom is a movie atom.
    pub flags: Option<[u8; 3]>, // 24 Bits

    // custom abstraction
    /// Atom size, including header and data.
    pub atom_size: u64,

    /// Atom header size, not including data size.
    pub header_size: u64,

    /// Atom data size, not including header size.
    pub data_size: u64,

    /// File offset of atom, in bytes.
    pub offset: u64,
}

impl Header {
    /// Parses a file and reads the header information from it. This will not parse the data.
    /// This will also parse the largesize, usertype, version, and flags if they exist.
    ///
    /// # Arguments
    ///
    /// `f: &mut Mp4File` -- The MP4 file to be read. This must already be opened and ready to read.
    ///
    /// # Returns
    ///
    /// `Result<Self, &'static str>` -- a `Header` struct if successful, and an error message otherwise.
    ///
    /// # Errors
    ///
    /// If unable to read file kind. If unable to parse the file.
    ///
    /// # Panics
    ///
    /// None.
    ///
    pub fn parse(f: &mut Mp4File) -> Result<Self, &'static str> {
        let curr_offset = f.offset();
        let_ok!(size, f.read_u32(), "Unable to read size.");

        let_ok!(b1, f.read_u8(), "Unable to read box type (kind) byte 1.");
        let_ok!(b2, f.read_u8(), "Unable to read box type (kind) byte 2.");
        let_ok!(b3, f.read_u8(), "Unable to read box type (kind) byte 3.");
        let_ok!(b4, f.read_u8(), "Unable to read box type (kind) byte 4.");

        let kind_bytes: [u8; 4] = [b1, b2, b3, b4];

        let_ok!(
            kind,
            Kind::from_bytes(kind_bytes),
            "Unable to read file kind."
        );

        let header_size = 8u64;
        let atom_size = u64::from(size);
        let data_size = 0u64;

        f.offset_inc(header_size);

        let mut header = Self {
            size,
            kind,

            largesize: None,
            usertype: None,
            version: None,
            flags: None,

            atom_size,           // atom size , include header and data.
            header_size,         // atom header size, not include data size.
            data_size,           // atom data size , not include header size.
            offset: curr_offset, // file offset.
        };

        match size.cmp(&1u32) {
            Ordering::Equal => header.parse_largesize(f),
            Ordering::Greater => header.data_size = atom_size - header_size,
            Ordering::Less => return Err("Cannot parse this mp4 file."),
        }

        log::trace!("Header::parse() -- header = {header:?}");

        Ok(header)
    }

    /// Parses the largesize part of the MP4 file
    ///
    /// # Arguments
    ///
    /// `f: &mut Mp4File` -- The file to be parsed.
    ///
    /// # Returns
    ///
    /// Nothing. Updates the `Header` struct.
    ///
    /// # Errors
    ///
    /// None.
    ///
    /// # Panics
    ///
    /// If `self.size != 1`. If unable to read the `largesize`;
    ///
    pub fn parse_largesize(&mut self, f: &mut Mp4File) {
        assert_eq!(self.size, 1u32);

        let largesize = f.read_u64().expect("Unable to read largesize.");
        self.atom_size = largesize;
        self.header_size += 8;
        self.data_size = largesize - self.header_size;

        self.largesize = Some(largesize);
        f.offset_inc(8);
    }

    /// Parses the usertype part of the MP4 file
    ///
    /// # Arguments
    ///
    /// `f: &mut Mp4File` -- A mutable MP4 file that will be traversed.
    ///
    /// # Returns
    ///
    /// Nothing.
    ///
    /// # Errors
    ///
    /// None.
    ///
    /// # Panics
    ///
    /// None.
    ///
    pub fn parse_usertype(&mut self, f: &mut Mp4File) {
        let usertype: [u8; 16] = [
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
        ];

        self.usertype = Some(usertype);
        self.header_size += 16;
        self.data_size = self.atom_size - self.header_size;

        let _offset = f.offset_inc(16);
    }

    /// Parses the version information from the file.
    pub fn parse_version(&mut self, f: &mut Mp4File) {
        let version = f.read_u8().expect("Unable to read version information.");
        self.version = Some(version);

        self.header_size += 1;
        self.data_size = self.atom_size - self.header_size;
        let _offset = f.offset_inc(1);
    }

    /// Parses the flags from the file.
    pub fn parse_flags(&mut self, f: &mut Mp4File) {
        let flags: [u8; 3] = [
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
            f.read_u8().unwrap_or_default(),
        ];
        self.flags = Some(flags);

        self.header_size += 3;
        self.data_size = self.atom_size - self.header_size;
        let _offset = f.offset_inc(3);
    }

    /// Returns a new, empty `Header` struct by calling `Header::default()`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    retval!(size, u32);
    retref!(kind, Kind);
    retval!(largesize, Option<u64>);
    retval!(usertype, Option<[u8; 16]>);
    retval!(version, Option<u8>);
    retval!(flags, Option<[u8; 3]>);
    retval!(atom_size, u64);
    retval!(header_size, u64);
    retval!(data_size, u64);
    retval!(offset, u64);

    /// Converts the header flags to a u32 which can be used to check flags against certain values.
    /// See [Tkhd](crate::atom::tkhd) for examples of how this is used.
    pub fn flags_to_u32(&self) -> u32 {
        let flags: [u8; 3] = self.flags.unwrap_or_default();
        let mut res: [u8; 4] = [0, 0, 0, 0];

        for i in 0..3 {
            res[i + 1] = flags[i];
        }

        u32::from_be_bytes(res)
    }
}

impl std::default::Default for Header {
    /// Returns a new, empty `Header` struct.
    fn default() -> Self {
        Self {
            size: 0,
            kind: Kind::Skip,
            largesize: None,
            usertype: None,
            version: None,
            flags: None,
            atom_size: 0,
            header_size: 0,
            data_size: 0,
            offset: 0,
        }
    }
}
