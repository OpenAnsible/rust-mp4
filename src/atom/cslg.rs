//! Composition to Decode Box, used to relate the composition and decoding timelines.
//!
//! When signed composition offsets are used, this box may be used to relate the composition and decoding
//! timelines, and deal with some of the ambiguities that signed composition offsets introduce.
//!
//! Note  that  all  these  fields  apply  to  the  entire  media  (not  just  that  selected  by  any  edits).  It  is
//! recommended that any edits, explicit or implied, not select any portion of the composition timeline that
//! does not map to a sample. For example, if the smallest composition time is 1000, then the default edit
//! from 0 to the media duration leaves the period from 0 to 1000 associated with no media sample. Player
//! behaviour,  and  what  is  composed  in  this  interval,  is  undefined under  these  circumstances.  It  is
//! recommended that the smallest computed CTS be zero, or match the beginning of the first edit.
//!
//! The composition duration of the last sample in a track might be (often is) ambiguous or unclear; the
//! field for composition end time can be used to clarify this ambiguity and, with the composition start
//! time, establish a clear composition duration for the track.
//!
//! When  the  Composition  to  Decode  Box  is  included  in  the  Sample  Ta ble  Box,  it  documents  the
//! composition and decoding time relations of the samples in the Movie Box only, not including any
//! subsequent movie fragments. When the Composition to Decode Box is included in the Track Extension
//! Properties Box, it documents the composition and decoding time relations of the samples in all movie
//! fragments following the Movie Box.
//!
//! Version 1 of this box supports 64‐bit timestamps and should only be used if needed (at least one value
//! does not fit into 32 bits).
//!
//! - Box Type:  `cslg`
//! - Container:  Sample Table Box (`stbl`) or Track Extension Properties Box (`trep`)
//! - Mandatory: No
//! - Quantity:  Zero or one

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{read_version_int, retref, retval};

#[derive(Debug, Clone)]
pub struct Cslg {
    pub header: Header,
    pub composition_to_dtsshift: i64,
    pub least_decode_to_display_delta: i64,
    pub greatest_decode_to_display_delta: i64,
    pub composition_start_time: i64,
    pub composition_end_time: i64,
}

impl Cslg {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        read_version_int!(composition_to_dtsshift, f, header);
        read_version_int!(least_decode_to_display_delta, f, header);
        read_version_int!(greatest_decode_to_display_delta, f, header);
        read_version_int!(composition_start_time, f, header);
        read_version_int!(composition_end_time, f, header);

        f.offset_inc(header.data_size);

        Ok(Self {
            header,
            composition_to_dtsshift,
            least_decode_to_display_delta,
            greatest_decode_to_display_delta,
            composition_start_time,
            composition_end_time,
        })
    }

    // These are here for completeness, since the struct is public.
    retref!(header, Header);
    retval!(composition_to_dtsshift, i64);
    retval!(least_decode_to_display_delta, i64);
    retval!(greatest_decode_to_display_delta, i64);
    retval!(composition_start_time, i64);
    retval!(composition_end_time, i64);
}