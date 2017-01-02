#![allow(dead_code, unused_imports, unused_variables, unused_mut, non_camel_case_types)]
#![allow(unused_must_use, unreachable_code, non_snake_case, unused_assignments, unused_parens)]

extern crate byteorder;

use std::fs;
use std::fs::File;
use std::io::{Write, Read, ErrorKind, Error, SeekFrom, Seek};
use byteorder::{BigEndian, ReadBytesExt};

pub mod atom;

#[derive(Debug)]
pub struct Mp4File {
    file     : File,
    file_size: u64,
    offset   : u64,
    atoms    : Vec<atom::Atom>
}

impl Mp4File {
    pub fn new(filename: &str) -> Result<Self, &'static str> {
        let mut file = fs::OpenOptions::new().read(true).write(false)
                .create(false).open(filename).unwrap();
        let file_size = file.metadata().unwrap().len();

        let mp4 = Mp4File {
            file: file,
            file_size: file_size,
            offset: 0,
            atoms: vec![]
        };
        Ok(mp4)
    }
    pub fn file(&self) -> &File {
        &self.file
    }
    pub fn file_size(&self) -> u64 {
        self.file_size
    }
    pub fn offset(&self) -> u64 {
        self.offset
    }
    pub fn offset_inc(&mut self, num: u64) -> u64 {
        self.offset += num;
        self.offset
    }
    pub fn atoms(&self) -> &Vec<atom::Atom> {
        &self.atoms
    }
    pub fn parse(&mut self) {
        loop {
            match atom::Atom::parse(self) {
                Ok(atom) => {
                    self.atoms.push(atom);
                    if self.offset == self.file_size {
                        break;
                    }
                },
                Err(e) => {
                    println!("[ERROR] ATOM parse error ({:?})", e);
                    break;
                }
            }
        }
    }
    // File Seek
    pub fn seek(&mut self, offset: u64) -> Result<u64, Error> {
        self.file.seek(SeekFrom::Start(offset))
    }
    // Byte Reader
    pub fn read_u8(&mut self)-> Result<u8, Error> {
        self.file.read_u8()
    }
    pub fn read_u16(&mut self)-> Result<u16, Error> {
        self.file.read_u16::<BigEndian>()
    }
    pub fn read_u32(&mut self)-> Result<u32, Error> {
        self.file.read_u32::<BigEndian>()
    }
    pub fn read_u64(&mut self)-> Result<u64, Error> {
        self.file.read_u64::<BigEndian>()
    }

    pub fn read_i8(&mut self)-> Result<i8, Error> {
        self.file.read_i8()
    }
    pub fn read_i16(&mut self)-> Result<i16, Error> {
        self.file.read_i16::<BigEndian>()
    }
    pub fn read_i32(&mut self)-> Result<i32, Error> {
        self.file.read_i32::<BigEndian>()
    }
    pub fn read_i64(&mut self)-> Result<i64, Error> {
        self.file.read_i64::<BigEndian>()
    }

    pub fn read_f32(&mut self)-> Result<f32, Error> {
        self.file.read_f32::<BigEndian>()
    }
    pub fn read_f64(&mut self)-> Result<f64, Error> {
        self.file.read_f64::<BigEndian>()
    }

}

pub fn parse_file(filename: &str) -> Result<Mp4File, &'static str>{
    let mut mp4 = Mp4File::new(filename).unwrap();
    mp4.parse();
    for atom in mp4.atoms() {
        println!("Atom: \n\t{:?}", atom);
    }
    Ok(mp4)
}

