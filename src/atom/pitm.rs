//! Primary Item

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{read_version, retref, retval};

#[derive(Debug, Clone)]
pub struct Pitm {
    /// Header of the `Pitm` box
    pub header: Header,

    /// The identifier of the primary item. Version 1 should only be used when large `item_id`
    /// values (exceeding 65535) are required or expected to be required.
    pub item_id: u32,
}

impl Pitm {
    /// Parse `Pitm` box using the given `Header`.
    ///
    /// # Arguments
    ///
    /// * `f` - Mp4File to read from
    /// * `header` - Header of the `Pitm` box
    ///
    /// # Returns
    ///
    /// * `Result<Self, &'static str>` - The parsed `Pitm` box if successful, otherwise an error
    ///
    /// # Panics
    ///
    /// None.
    pub fn parse(f: &mut Mp4File, header: Header) -> Result<Self, &'static str> {
        read_version!(item_id, u32, f.read_u16(), f.read_u32(), header);
        f.offset_inc(header.data_size);

        Ok(Self { header, item_id })
    }

    retref!(header, Header);
    retval!(item_id, u32);
}
