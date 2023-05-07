//! Defines the header type for atoms.

use std::cmp::Ordering;

use super::kind::Kind;
use crate::let_ok;
use crate::mp4file::Mp4File;

#[derive(Debug, Clone)]
pub struct Header {
    /// The size in bytes of the atom, including the header and the data.
    pub size: u32,

    /// The type of atom. Refer to the `Kind` enum for more information.
    pub kind: Kind, // atom type

    // Optional
    /// The size in bytes of the atom, including the header and the data.
    pub largesize: Option<u64>,

    /// The user type of the atom. This is used for custom atoms. Example: `uuid`.
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

        let_ok!(b1, f.read_u8(), "Unable to read header byte 1.");
        let_ok!(b2, f.read_u8(), "Unable to read header byte 2.");
        let_ok!(b3, f.read_u8(), "Unable to read header byte 3.");
        let_ok!(b4, f.read_u8(), "Unable to read header byte 4.");

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
}
