//!

use crate::atom::header::Header;
use crate::let_ok;
use crate::mp4file::Mp4File;

#[derive(Debug, Clone)]
pub struct Mdhd {
    pub header: Header,
    pub creation_time: u64,
    pub modification_time: u64,
    pub timescale: u32,
    pub duration: u64,
    pub language: String,
}

impl Mdhd {
    pub fn parse(f: &mut Mp4File, mut header: Header) -> Result<Self, &'static str> {
        header.parse_version(f);
        header.parse_flags(f);

        let curr_offset = f.offset();

        let mut length = 0;

        let creation_time;
        let modification_time;
        let timescale;
        let duration;

        if header.version.is_none() {
            return Err("Unable to determine header version.");
        }

        if header.version.unwrap_or(0) == 1u8 {
            let_ok!(ct, f.read_u64(), "Unable to read creation time.");
            creation_time = ct;

            let_ok!(mt, f.read_u64(), "Unable to read modification time.");
            modification_time = mt;

            let_ok!(ts, f.read_u32(), "Unable to read time scale.");
            timescale = ts;

            let_ok!(du, f.read_u64(), "Unable to read duration.");
            duration = du;
            length += 28;
        } else {
            // header version == 0
            let_ok!(ct, f.read_u32(), "Unable to read creation time.");
            creation_time = u64::from(ct);

            let_ok!(mt, f.read_u32(), "Unable to read modification time.");
            modification_time = u64::from(mt);

            let_ok!(ts, f.read_u32(), "Unable to read time scale.");
            timescale = ts;

            let_ok!(du, f.read_u32(), "Unable to read duration.");
            duration = u64::from(du);
            length += 16;
        }

        // 16 Bytes
        // pad: 1 Bit
        // language: 15 Bit;
        let_ok!(
            language,
            f.read_iso639_code(),
            "Unable to read language from ISO639 code."
        );
        length += 2;

        // unsigned int(16) pre_defined = 0;
        length += 2;
        let _offset = f.seek(curr_offset + length);
        f.offset_inc(length);

        Ok(Self {
            header,
            creation_time,
            modification_time,
            timescale,
            duration,
            language,
        })
    }
}
