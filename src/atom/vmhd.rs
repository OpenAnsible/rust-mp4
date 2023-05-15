//!

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref, retval};

#[derive(Debug, Clone)]
pub struct Vmhd {
    pub header: Header,
    pub graphicsmode: u16,
    pub opcolor: [u16; 3],
}

impl Vmhd {
    /// Parse a `Vmhd` atom from the file.
    ///
    /// # Arguments
    ///
    /// * `f` - The file to read from.
    /// * `header` - The header of the atom.
    ///
    /// # Returns
    ///
    /// - `Result<Self, &'static str>` - The parsed atom.
    ///
    /// # Errors
    ///
    /// - `Err` - If the graphics mode cannot be read from the file.
    /// - `Err` - If the opcolor cannot be read from the file.
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let_ok!(
            graphicsmode,
            f.read_u16(),
            "Vmhd: Unable to read graphics mode."
        );

        // red, green, blue
        let_ok!(r, f.read_u16(), "Vmhd: Unable to read opcolor red");
        let_ok!(g, f.read_u16(), "Vmhd: Unable to read opcolor green");
        let_ok!(b, f.read_u16(), "Vmhd: Unable to read opcolor blue");
        let opcolor: [u16; 3] = [r, g, b];

        f.offset_inc(8);

        Ok(Self {
            header,
            graphicsmode,
            opcolor,
        })
    }

    retref!(header, Header);
    retval!(graphicsmode, u16);
    retref!(opcolor, [u16; 3]);
}
