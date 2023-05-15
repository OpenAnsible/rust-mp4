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
//! mp4 = { version = "0.2.0", path = "path/to/mp4" }
//! ```
//!
//! ## Examples
//!
//! ```ignore
//! use mp4;
//!
//! fn main() {
//!    let res = mp4::parse_file("myfile.mp4");
//!    dbg!(res);
//! }
extern crate byteorder;

pub mod atom;
mod macros;
pub mod matrix;
pub mod mp4file;
pub mod utils;
// pub mod version_int;

pub use atom::atom::Atom;
pub use atom::bxml::Bxml;
pub use atom::co64::Co64;
pub use atom::cslg::Cslg;
pub use atom::ctts::Ctts;
pub use atom::stts::SttsEntry;
pub use mp4file::{parse_file, Mp4File};
pub use utils::{duration_seconds, time_to_utc};
// pub use version_int::VersionInt;
