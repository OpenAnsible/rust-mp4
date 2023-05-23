//!

use crate::atom::header::Header;
use crate::mp4file::Mp4File;
use crate::{let_ok, let_some, retref, retval};

#[derive(Debug, Clone)]
pub struct Stz2 {
    pub header: Header,
    pub field_size: u8,
    pub sample_count: u32,
    pub entry_size: Vec<u32>,
}

impl Stz2 {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let_ok!(
            _throwaway,
            f.read_u32(),
            "Unable to read first 4 bytes of file."
        );
        let_ok!(field_size, f.read_u8(), "Unable to read field size.");
        let_ok!(sample_count, f.read_u32(), "Unable to read sample count.");

        if field_size != 4u8 && field_size != 8u8 && field_size != 16u8 {
            return Err("Wrong field size detected. Should be 4, 8 or 16.");
        }

        let mut entry_size: Vec<u32> = Vec::new();
        let mut next_val: Option<u32> = None;

        for _ in 0..sample_count {
            if field_size == 4u8 {
                if next_val.is_some() {
                    let_some!(nv, next_val, "Unable to read next value.");
                    entry_size.push(nv);
                    next_val = None;
                } else {
                    let_ok!(byte, f.read_u8(), "Unable to read bits.");
                    let bits = format!("{byte:08b}");

                    let_ok!(
                        es,
                        u32::from_str_radix(&bits[0..4], 2),
                        "Unable to read entry size."
                    );
                    entry_size.push(es);

                    let_ok!(
                        nv,
                        u32::from_str_radix(&bits[4..8], 2),
                        "Unable to read next value."
                    );
                    next_val = Some(nv);
                }
            } else if field_size == 8u8 {
                let_ok!(es, f.read_u8(), "Unable to read entry size.");
                entry_size.push(u32::from(es));
            } else if field_size == 16u8 {
                let_ok!(es, f.read_u16(), "Unable to read entry size.");
                entry_size.push(u32::from(es));
            } else {
                return Err("STZ2 parse error.");
            }
        }

        f.offset_inc(header.data_size);
        Ok(Self {
            header,
            field_size,
            sample_count,
            entry_size,
        })
    }

    retref!(header, Header);
    retval!(field_size, u8);
    retval!(sample_count, u32);
    retref!(entry_size, Vec<u32>);
}
