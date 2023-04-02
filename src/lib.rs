// #![allow(dead_code, unused_imports)]
// #![allow(unused_must_use, non_snake_case, unused_assignments, unused_parens)]

extern crate byteorder;

use byteorder::{BigEndian, ReadBytesExt};
use std::fs;
use std::fs::File;
use std::io::{Error, Seek, SeekFrom};

pub mod atom;

/// Set a value from an expression that returns a `Result`, return an error message if not `Ok`.
///
/// # Arguments
///
/// - `$var:ident` -- The variable we wish to extract the value from the function into.
/// - `$fun:expr` -- The function to run. This function _must_ return a `Result`.
/// - `$msg:literal` -- The clear-text error message to return if the `$fun` returns an error.
///
/// # Examples
///
/// ```
/// fn meaning_of_life(guess: u8) -> Result<u8, Error> {
///     if guess == 42 {
///         Ok(guess)
///     } else {
///         Err("Wrong!".into())
///     }
/// }
///
/// let_ok!(mol, meaning_of_life(42), "Meaning of life not found.");
///
/// assert_eq!(mol, 42);
/// ```
#[macro_export]
macro_rules! let_ok {
    ($var:ident, $fun:expr, $msg:literal) => {
        let Ok($var) = $fun else {
                                                                                    return Err($msg)
                                                                                };
    };
}

/// Set a value from an expression that returns an `Option`, return an error message if not `Some`.
#[macro_export]
macro_rules! let_some {
    ($var:ident, $fun:expr, $msg:literal) => {
        let Some($var) = $fun else {
                                                                                    return Err($msg)
                                                                                };
    };
}

/// Creates a `pub const fn` that returns a value from `self`.
#[macro_export]
macro_rules! retval {
    ($id:ident, $typ:ty) => {
        #[must_use]
        pub const fn $id(&self) -> $typ {
            self.$id
        }
    };
}

/// Creates a `pub const fn` that returns a reference to a value from `self`.
#[macro_export]
macro_rules! retref {
    ($id:ident, $typ:ty) => {
        pub const fn $id(&self) -> &$typ {
            &self.$id
        }
    };
}

#[derive(Debug)]
pub struct Mp4File {
    file: File,
    file_size: u64,
    offset: u64,
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
            atoms: vec![],
        };
        Ok(mp4)
    }

    #[must_use]
    pub const fn file(&self) -> &File {
        &self.file
    }

    retval!(file_size, u64);
    retval!(offset, u64);

    pub fn offset_inc(&mut self, num: u64) -> u64 {
        self.offset += num;
        self.offset
    }

    #[must_use]
    pub const fn atoms(&self) -> &Vec<atom::Atom> {
        &self.atoms
    }

    // TODO: This should return a Result.
    pub fn parse(&mut self) {
        self.atoms = atom::Atom::parse_children(self);
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
    ///
    ///
    /// # Examples
    ///
    ///
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

/// Converts the timestamp from the epoch used in the MPEG4 specification (seconds since 1904-01-01 00:00:00)
/// to the UNIX epoch time (seconds since 1970-01-01 00:00:00). This is done by subtracting 2,082,844,800 seconds from
/// the given time to return the new time as there are 2,082,844,800 seconds from 1904-01-01 00:00:00 to 1970-01-01 00:00:00.
#[must_use]
pub const fn mp4time_to_unix_time(time: u64) -> u64 {
    time - 2_082_844_800
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
    retval!(a, f64);
    retval!(b, f64);
    retval!(u, f64);
    retval!(c, f64);
    retval!(d, f64);
    retval!(v, f64);
    retval!(x, f64);
    retval!(y, f64);
    retval!(w, f64);
}
