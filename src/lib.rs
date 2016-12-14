#![allow(dead_code, unused_imports, unused_variables, unused_mut, non_camel_case_types)]
#![allow(unused_must_use, unreachable_code, non_snake_case, unused_assignments, unused_parens)]

extern crate byteorder;

use std::fs;
use std::io::{Write, Read};
use std::ops::Add;

pub mod atom;

pub struct Offset(u64);
impl Add for Offset {
    type Output = Offset;
    fn add(self, rhs: Offset) -> Offset {
        Offset(self.0 + rhs.0)
    }
}

impl Offset {
    pub fn update(&mut self, num: u64){
        self.0 += num;
    }
    pub fn offset(&self) -> u64 {
        self.0
    }
}

pub fn parse_file(filename: &str){
    let mut file = fs::OpenOptions::new().read(true).write(false)
                .create(false).open(filename).unwrap();
    let file_size = file.metadata().unwrap().len();
    println!("File: {:?}", file);

    let mut offset: Offset = Offset(0);
    let mut atoms: Vec<atom::AtomBox> = Vec::new();
    loop {
        let atombox = match atom::parse(&mut file, &mut offset){
            Ok(atombox) => {
                atoms.push(atombox);
                if offset.offset() == file_size {
                    break;
                }
            },
            Err(_) => break
        };

    }
    for item in atoms.iter() {
        println!("{:?}", item);
    }
}


