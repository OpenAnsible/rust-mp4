// #![allow(dead_code, unused_imports)]
// #![allow(unused_must_use, non_snake_case, unused_assignments, unused_parens)]

extern crate byteorder;

use byteorder::{BigEndian, ReadBytesExt};
use std::fs;
use std::fs::File;
use std::io::{Error, Seek, SeekFrom};

pub mod atom;

#[derive(Debug)]
pub struct Mp4File {
    file: File,
    file_size: u64,
    offset: u64,
    atoms: Vec<atom::Atom>,
}

impl Mp4File {
    pub fn new(filename: &str) -> Result<Self, &'static str> {
        let file = fs::OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(filename)
            .unwrap();
        let file_size = file.metadata().unwrap().len();

        let mp4 = Self {
            file,
            file_size,
            offset: 0,
            atoms: vec![],
        };
        Ok(mp4)
    }

    #[must_use]
    pub fn file(&self) -> &File {
        &self.file
    }

    #[must_use]
    pub fn file_size(&self) -> u64 {
        self.file_size
    }
    #[must_use]
    pub fn offset(&self) -> u64 {
        self.offset
    }
    pub fn offset_inc(&mut self, num: u64) -> u64 {
        self.offset += num;
        self.offset
    }
    #[must_use]
    pub fn atoms_as_ref(&self) -> &Vec<atom::Atom> {
        &self.atoms
    }
    #[must_use]
    pub fn atoms(&self) -> Vec<atom::Atom> {
        self.atoms.clone()
    }
    pub fn parse(&mut self) {
        let atoms = atom::Atom::parse_children(self);
        self.atoms = atoms;
    }
    // File Seek
    pub fn seek(&mut self, offset: u64) -> Result<u64, Error> {
        self.file.seek(SeekFrom::Start(offset))
    }
    // Byte Reader
    pub fn read_u8(&mut self) -> Result<u8, Error> {
        self.file.read_u8()
    }
    pub fn read_u16(&mut self) -> Result<u16, Error> {
        self.file.read_u16::<BigEndian>()
    }
    pub fn read_u32(&mut self) -> Result<u32, Error> {
        self.file.read_u32::<BigEndian>()
    }
    pub fn read_u64(&mut self) -> Result<u64, Error> {
        self.file.read_u64::<BigEndian>()
    }

    pub fn read_i8(&mut self) -> Result<i8, Error> {
        self.file.read_i8()
    }
    pub fn read_i16(&mut self) -> Result<i16, Error> {
        self.file.read_i16::<BigEndian>()
    }
    pub fn read_i32(&mut self) -> Result<i32, Error> {
        self.file.read_i32::<BigEndian>()
    }
    pub fn read_i64(&mut self) -> Result<i64, Error> {
        self.file.read_i64::<BigEndian>()
    }

    pub fn read_f32(&mut self) -> Result<f32, Error> {
        self.file.read_f32::<BigEndian>()
    }
    pub fn read_f64(&mut self) -> Result<f64, Error> {
        self.file.read_f64::<BigEndian>()
    }
    pub fn read_fixed_point(
        &mut self,
        integer_length: usize,
        fractional_length: usize,
    ) -> Result<f64, Error> {
        // https://en.wikipedia.org/wiki/Fixed_point_(mathematics)
        if integer_length + fractional_length == 16 {
            let n = self.read_u16().unwrap();
            let integer: u16 = n >> fractional_length as u16;
            let fractional_mask: u16 = 2u16.pow(fractional_length as u32) - 1;
            let fractional: u16 = (n & fractional_mask) / (1 << (fractional_length as u16));
            let result = f64::from(integer + fractional);
            Ok(result)
        } else {
            let n = self.read_u32().unwrap();
            let integer: u32 = n >> fractional_length as u32;
            let fractional_mask: u32 = 2u32.pow(fractional_length as u32) - 1;
            let fractional: u32 = (n & fractional_mask) / (1 << (fractional_length as u32));
            let result = f64::from(integer + fractional);
            Ok(result)
        }
    }
    pub fn read_matrix(&mut self) -> Result<Matrix, Error> {
        // length: u32 * 9 (  4*9 = 36 Bytes )
        let a = self.read_fixed_point(16, 16).unwrap();
        let b = self.read_fixed_point(16, 16).unwrap();
        let u = self.read_fixed_point(2, 30).unwrap();
        let c = self.read_fixed_point(16, 16).unwrap();
        let d = self.read_fixed_point(16, 16).unwrap();
        let v = self.read_fixed_point(2, 30).unwrap();
        let x = self.read_fixed_point(16, 16).unwrap();
        let y = self.read_fixed_point(16, 16).unwrap();
        let w = self.read_fixed_point(2, 30).unwrap();
        Ok(Matrix {
            a,
            b,
            u,
            c,
            d,
            v,
            x,
            y,
            w,
        })
    }
    pub fn read_iso639_code(&mut self) -> Result<String, Error> {
        // Note:
        //      pad   :  1 Bit
        //      string: 15 Bit
        let mut s = String::new();
        let n = self.read_u16().unwrap();
        let mut c1 = n & 0x7C00 >> 10; // Mask is 0111 1100 0000 0000
        let mut c2 = n & 0x03E0 >> 5; // Mask is 0000 0011 1110 0000
        let mut c3 = n & 0x001F; // Mask is 0000 0000 0001 1111
        c1 += 0x60;
        c2 += 0x60;
        c3 += 0x60;

        s.push((c1 as u8) as char);
        s.push((c2 as u8) as char);
        s.push((c3 as u8) as char);
        Ok(s)
    }
}

pub fn parse_file(filename: &str) -> Result<Mp4File, &'static str> {
    let mut mp4 = Mp4File::new(filename).unwrap();
    mp4.parse();
    for atom in mp4.atoms() {
        println!("Atom: \n\t{atom:?}");
    }
    Ok(mp4)
}

/// Converts the timestamp from the epoch used in the MPEG4 specification (seconds since 1904-01-01 00:00:00)
/// to the UNIX epoch time (seconds since 1970-01-01 00:00:00). This is done by subtracting 2,082,844,800 seconds from
/// the given time to return the new time.
/// There are 2,082,844,800 seconds from 1904-01-01 00:00:00 to 1970-01-01 00:00:00.
#[must_use]
pub fn mp4time_to_unix_time(time: u64) -> u64 {
    time - 2_082_844_800
}

macro_rules! mtx {
    ($id:ident) => {
        pub fn $id(&self) -> f64 {
            self.$id
        }
    };
}

#[derive(Debug, Clone)]
pub struct Matrix {
    a: f64,
    b: f64,
    u: f64,
    c: f64,
    d: f64,
    v: f64,
    x: f64,
    y: f64,
    w: f64,
}

impl Matrix {
    mtx!(a);
    mtx!(b);
    mtx!(u);
    mtx!(c);
    mtx!(d);
    mtx!(v);
    mtx!(x);
    mtx!(y);
    mtx!(w);
}
