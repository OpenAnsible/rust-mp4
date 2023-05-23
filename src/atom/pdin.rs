//!

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, retref, retval};

/// Pdin atom (Progressive Download Information) as defined in ISO/IEC 14496-12:2015 § 8.7.4.
/// Describes the progressive download information for the file, meaning the rate at which the file
/// should be downloaded and the initial delay before playback should begin.
#[derive(Debug, Clone)]
pub struct Pdin {
    /// The header of the atom.
    header: Header,

    /// The rate at which the file should be downloaded.
    rate: u32,

    /// The initial delay before playback should begin.
    initial_delay: u32,
}

impl Pdin {
    /// Parses a Pdin atom from the given file. The header is already parsed and passed in.
    /// The file is advanced to the end of the atom.
    ///
    /// # Arguments
    ///
    /// * `f` - The file to read from.
    /// * `header` - The header of the atom.
    ///
    /// # Returns
    ///
    /// * `Self` - The parsed atom.
    ///
    /// # Errors
    ///
    /// * `Unable to read rate.` - If the rate cannot be read from the file.
    /// * `Unable to read initial delay.` - If the initial delay cannot be read from the file.
    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str> {
        let_ok!(rate, f.read_u32(), "Unable to read rate.");
        let_ok!(initial_delay, f.read_u32(), "Unable to read initial delay.");

        f.offset_inc(header.data_size);

        Ok(Self {
            header,
            rate,
            initial_delay,
        })
    }

    retref!(header, Header);
    retval!(rate, u32);
    retval!(initial_delay, u32);
}
