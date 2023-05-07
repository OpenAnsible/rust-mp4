//! # rust-mp4
//!
//! A library for parsing MP4 files. This library is a work in progress, and is not yet ready for use.
//! The goal is to be able to parse MP4 files and extract the metadata from them.
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! mp4 = "0.1.0"
//! ```
//!
//! ## Examples
//!
//! ```ignore
//! use mp4;
//!
//! fn main() {
//!    let res = mp4::parse_file("EvenSolberg_20230325_004746___0003.MP4");
//!    dbg!(res);
//! }
extern crate byteorder;

pub mod atom;
mod macros;
pub mod matrix;
pub mod mp4file;
