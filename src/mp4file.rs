//! Defines the top-level `Mp4File` struct, which represents the MP4 file.
//!
//! Files are formed as a series of objects, called boxes in this specification. All data is contained in boxes;
//! there is no other data within the file. This includes any initial signature required by the specific file format.
//! All object‐structured files conformant to this section of this specification (all Object‐Structured files)
//! shall contain a File Type Box.

use crate::atom::atom;
use crate::matrix::Matrix;
use crate::{let_ok, retref, retval};

use byteorder::{BigEndian, ReadBytesExt};
use std::fs;
use std::fs::File;
use std::io::{Error, Seek, SeekFrom};

/// The main struct for the MP4 file.
///
/// It contains the file itself, the size of the file, the current offset into the file,
/// along with the atoms that have been parsed from the file. The atoms are stored in a `Vec` of `Atom` structs.
/// The `Mp4File` struct also contains methods for reading data from the file.
#[derive(Debug)]
pub struct Mp4File {
    /// The file we are reading from.
    file: File,

    /// The size of the file in bytes.
    file_size: u64,

    /// The current offset into the file.
    offset: u64,

    /// Indicates whether the file has been parsed or not.
    parsed: bool,

    /// The atoms we have parsed from the file.
    atoms: Vec<atom::Atom>,
}

impl Mp4File {
    /// Opens an MP4 file for reading, and returns an `Mp4File` struct with the name and size of the file filled.
    ///
    /// # Arguments
    ///
    /// `filename: &str` -- The name of the file.
    ///
    /// # Returns
    ///
    /// `Result<Self, &'static str>` -- a result containing filled `Mp4File` struct if successful, or an error message if not.
    ///
    /// # Errors
    ///
    /// Returns errors if unable to open the file or if unable to read the file metadata.
    ///
    /// # Panics
    ///
    /// None.
    ///
    pub fn new(filename: &str) -> Result<Self, &'static str> {
        let file_result = fs::OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(filename);
        let_ok!(file, file_result, "Unable to open the Options file.");

        let_ok!(fm, file.metadata(), "Unable to read file metadata.");
        let file_size = fm.len();

        let mp4 = Self {
            file,
            file_size,
            offset: 0,
            parsed: false,
            atoms: vec![],
        };
        Ok(mp4)
    }

    retref!(file, File);
    retval!(file_size, u64);
    retval!(offset, u64);

    /// Returns a reference to the vector of atoms.
    ///
    /// # Arguments
    ///
    /// None.
    ///
    /// # Returns
    ///
    /// A reference to the vector of atoms.
    /// This will return an empty vector if the file has not been parsed.
    #[must_use]
    pub const fn atoms(&self) -> &Vec<atom::Atom> {
        &self.atoms
    }

    /// Increment the offset by a given number of bytes.
    ///
    /// # Arguments
    ///
    /// `num: u64` -- The number of bytes to increment the offset by.
    ///
    /// # Returns
    ///
    /// The new offset.
    pub fn offset_inc(&mut self, num: u64) -> u64 {
        self.offset += num;
        self.offset
    }

    /// Parse the file into atoms. This will parse the file into a vector of atoms, and set the `parsed` flag to true.
    /// This function will only parse the file once, subsequent calls will return immediately.
    // TODO: This should return a Result.
    pub fn parse(&mut self) {
        if !self.parsed {
            self.atoms = atom::Atom::parse_children(self).unwrap_or_default();
            self.parsed = true;
        }
    }

    /// File Seek
    ///
    /// # Arguments
    ///
    /// `offset: u64` -- How far (in bytes) we want to seek into the file.
    ///
    /// # Returns
    ///
    /// `Result<u64, Error>` -- The resulting offset if successful.
    ///
    /// # Errors
    ///
    /// IO Error if unsuccessful.
    ///
    /// # Panics
    ///
    /// None.
    ///
    pub fn seek(&mut self, offset: u64) -> Result<u64, Error> {
        self.file.seek(SeekFrom::Start(offset))
    }

    /// Reads a `u8` from a file.
    ///
    /// # Arguments
    ///
    /// None.
    ///
    /// # Returns
    ///
    /// `Result<u8, Error>` -- the `u8` if successful, otherwise an IO Error.
    ///
    /// # Errors
    ///
    /// IO Error if unsucessul
    ///
    /// # Panics
    ///
    /// None.
    ///
    pub fn read_u8(&mut self) -> Result<u8, Error> {
        self.file.read_u8()
    }

    /// Reads a [Big Endian](https://en.wikipedia.org/wiki/Endianness) `u16` from a file.
    ///
    /// # Arguments
    ///
    /// None.
    ///
    /// # Returns
    ///
    /// `Result<u16, Error>` -- the `u16` if successful, otherwise an IO Error.
    ///
    /// # Errors
    ///
    /// IO Error if unsucessul
    ///
    /// # Panics
    ///
    /// None.
    ///
    pub fn read_u16(&mut self) -> Result<u16, Error> {
        self.file.read_u16::<BigEndian>()
    }

    /// Reads a [Big Endian](https://en.wikipedia.org/wiki/Endianness) `u32` from a file.
    ///
    /// # Arguments
    ///
    /// None.
    ///
    /// # Returns
    ///
    /// `Result<u32, Error>` -- the `u32` if successful, otherwise an IO Error.
    ///
    /// # Errors
    ///
    /// IO Error if unsucessul
    ///
    /// # Panics
    ///
    /// None.
    ///
    pub fn read_u32(&mut self) -> Result<u32, Error> {
        self.file.read_u32::<BigEndian>()
    }

    /// Reads a [Big Endian](https://en.wikipedia.org/wiki/Endianness) `u64` from a file.
    ///
    /// # Arguments
    ///
    /// None.
    ///
    /// # Returns
    ///
    /// `Result<u64, Error>` -- the `u64` if successful, otherwise an IO Error.
    ///
    /// # Errors
    ///
    /// IO Error if unsucessul
    ///
    /// # Panics
    ///
    /// None.
    ///
    pub fn read_u64(&mut self) -> Result<u64, Error> {
        self.file.read_u64::<BigEndian>()
    }

    /// Reads an `i8` from a file.
    ///
    /// # Arguments
    ///
    /// None.
    ///
    /// # Returns
    ///
    /// `Result<i8, Error>` -- the `i8` if successful, otherwise an IO Error.
    ///
    /// # Errors
    ///
    /// IO Error if unsucessul
    ///
    /// # Panics
    ///
    /// None.
    ///
    pub fn read_i8(&mut self) -> Result<i8, Error> {
        self.file.read_i8()
    }

    /// Reads a [Big Endian](https://en.wikipedia.org/wiki/Endianness) `i16` from a file.
    ///
    /// # Arguments
    ///
    /// None.
    ///
    /// # Returns
    ///
    /// `Result<i16, Error>` -- the `i16` if successful, otherwise an IO Error.
    ///
    /// # Errors
    ///
    /// IO Error if unsucessul
    ///
    /// # Panics
    ///
    /// None.
    ///
    pub fn read_i16(&mut self) -> Result<i16, Error> {
        self.file.read_i16::<BigEndian>()
    }

    /// Reads a [Big Endian](https://en.wikipedia.org/wiki/Endianness) `i32` from a file.
    ///
    /// # Arguments
    ///
    /// None.
    ///
    /// # Returns
    ///
    /// `Result<i32, Error>` -- the `i32` if successful, otherwise an IO Error.
    ///
    /// # Errors
    ///
    /// IO Error if unsucessul
    ///
    /// # Panics
    ///
    /// None.
    ///
    pub fn read_i32(&mut self) -> Result<i32, Error> {
        self.file.read_i32::<BigEndian>()
    }

    /// Reads a [Big Endian](https://en.wikipedia.org/wiki/Endianness) `i64` from a file.
    ///
    /// # Arguments
    ///
    /// None.
    ///
    /// # Returns
    ///
    /// `Result<i64, Error>` -- the `i64` if successful, otherwise an IO Error.
    ///
    /// # Errors
    ///
    /// IO Error if unsucessul
    ///
    /// # Panics
    ///
    /// None.
    ///
    pub fn read_i64(&mut self) -> Result<i64, Error> {
        self.file.read_i64::<BigEndian>()
    }

    /// Reads a [Big Endian](https://en.wikipedia.org/wiki/Endianness) `f32` from a file.
    ///
    /// # Arguments
    ///
    /// None.
    ///
    /// # Returns
    ///
    /// `Result<f32, Error>` -- the `f32` if successful, otherwise an IO Error.
    ///
    /// # Errors
    ///
    /// IO Error if unsucessul
    ///
    /// # Panics
    ///
    /// None.
    ///
    pub fn read_f32(&mut self) -> Result<f32, Error> {
        self.file.read_f32::<BigEndian>()
    }

    /// Reads a [Big Endian](https://en.wikipedia.org/wiki/Endianness) `f64` from a file.
    ///
    /// # Arguments
    ///
    /// None.
    ///
    /// # Returns
    ///
    /// `Result<f64, Error>` -- the `f64` if successful, otherwise an IO Error.
    ///
    /// # Errors
    ///
    /// IO Error if unsucessul
    ///
    /// # Panics
    ///
    /// None.
    ///
    pub fn read_f64(&mut self) -> Result<f64, Error> {
        self.file.read_f64::<BigEndian>()
    }

    /// Reads a [fixed point](https://en.wikipedia.org/wiki/Fixed_point_(mathematics)) data point.
    ///
    /// # Arguments
    ///
    /// - `integer_length: usize` -- The integer length
    /// - `fractional_length: usize` -- The fractional length
    ///
    /// # Returns
    ///
    /// `Result<f64, Error>` -- The fixed point number as an `f64` if successful, otherwise an error.
    ///
    /// # Errors
    ///
    /// Reading the data from the stream can fail with an IO error.
    ///
    /// # Panics
    ///
    /// None.
    ///
    #[allow(clippy::cast_possible_truncation)]
    pub fn read_fixed_point(
        &mut self,
        integer_length: usize,
        fractional_length: usize,
    ) -> Result<f64, Error> {
        let result = if integer_length + fractional_length == 16 {
            let n = self.read_u16()?;
            let integer = n >> fractional_length;
            let fractional_mask = 2u16.pow(fractional_length as u32) - 1;
            let fractional = (n & fractional_mask) / (1 << fractional_length);
            f64::from(integer + fractional)
        } else {
            let n = self.read_u32()?;
            let integer = n >> fractional_length;
            let fractional_mask = 2u32.pow(fractional_length as u32) - 1;
            let fractional = (n & fractional_mask) / (1 << (fractional_length as u32));
            f64::from(integer + fractional)
        };

        Ok(result)
    }

    /// Reads a zero-terminated string from the file.
    pub fn read_null_terminated_string(&mut self) -> Result<String, Error> {
        let mut s = String::new();
        let mut byte = self.read_u8().unwrap_or(0);
        while byte != 0 {
            s.push(byte as char);
            byte = self.read_u8().unwrap_or(0);
        }

        Ok(s)
    }

    /// Reads a matrix
    ///
    /// # Arguments
    ///
    /// None.
    ///
    /// # Returns
    ///
    /// `Result<Matrix, Error>` -- The matrix if successful, error otherwise.
    ///
    /// # Errors
    ///
    /// Reading the various parts of the matrix can fail.
    ///
    /// # Panics
    ///
    /// None.
    ///
    #[allow(clippy::many_single_char_names)]
    pub fn read_matrix(&mut self) -> Result<Matrix, Error> {
        // length: u32 * 9 (  4*9 = 36 Bytes )
        let a = self.read_fixed_point(16, 16)?;
        let b = self.read_fixed_point(16, 16)?;
        let u = self.read_fixed_point(2, 30)?;
        let c = self.read_fixed_point(16, 16)?;
        let d = self.read_fixed_point(16, 16)?;
        let v = self.read_fixed_point(2, 30)?;
        let x = self.read_fixed_point(16, 16)?;
        let y = self.read_fixed_point(16, 16)?;
        let w = self.read_fixed_point(2, 30)?;

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

    /// Reads the [ISO 639](https://en.wikipedia.org/wiki/ISO_639) language code and returns it as a String.
    ///
    /// # Arguments
    ///
    /// None.
    ///
    /// # Returns
    ///
    /// `Result<String, Error>` -- A string with the language detected, as described in ISO 639.
    ///
    /// # Errors
    ///
    /// Reading the data from the file may return an error.
    ///
    /// # Panics
    ///
    /// None.
    ///
    #[allow(clippy::cast_possible_truncation)]
    pub fn read_iso639_code(&mut self) -> Result<String, Error> {
        // Note:
        //      pad   :  1 Bit
        //      string: 15 Bit
        let mut s = String::new();

        let n = self.read_u16()?;

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

    /// Reads a `u32` as a 4-character string.
    pub fn read_4_char_string(&mut self) -> Result<String, Error> {
        let mut s = String::new();
        let n = self.read_u32()?;
        s.push((n >> 24) as u8 as char);
        s.push((n >> 16) as u8 as char);
        s.push((n >> 8) as u8 as char);
        s.push(n as u8 as char);

        Ok(s)
    }
}

/// Parses the MP4 file into a new `Mp4File` struct that contains all the information.
///
/// # Arguments
///
/// `filename: &str` -- The name of the file to be parsed.
///
/// # Returns
///
/// `Result<Mp4File, &'staatic str>` -- a new, filled `Mp4File` struct if successful.
///
/// # Errors
///
/// Returns an error message if unable to open the file.
///
/// # Panics
///
///
///
/// # Examples
///
///
///
pub fn parse_file(filename: &str) -> Result<Mp4File, &'static str> {
    // Don't try to let_ok this -- the result is mutable.
    let Ok(mut mp4) = Mp4File::new(filename) else {
        return Err("Unable to open file.")
    };

    mp4.parse();

    Ok(mp4)
}
