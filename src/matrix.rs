//! Defines the `Matrix` struct, which represents the transformation matrix for the video.

/// Used in the `Matrix` atom to describe the transformation matrix for the video.
///
/// The matrix is used to describe the transformation from the video's display aspect ratio to the video's pixel aspect ratio.
/// In other words, the matrix is used to describe how the video is displayed on the screen.
/// Defined in [ISO/IEC 14496-12](https://www.iso.org/standard/68960.html). Section 6.2.2 gives a decent description of how the matrix is used.
/// Note that the matrix is a 3x3 matrix, but the last row is always 0, 0, 1, so we only store the first two rows.
/// Also note the "odd" way the matrix is stored in the file. The matrix is stored as a 32-bit fixed point number, but the first 16 bits are the integer part,
/// and the last 16 bits are the fractional part. So, for example, the number 1.0 is stored as 0x00010000, and the number 1.5 is stored as 0x00018000.
///
/// The matrix is stored in the file as follows:
///
/// ```ignore
/// a  b  u
/// c  d  v
/// x  y  w
/// ```
///
/// where a, b, c, d, x, and y are 16-bit fixed point numbers, and u, v, and w are 2-bit fixed point numbers.
///
/// Hence the strange ordering of the fields in the struct.
#[derive(Debug, Clone)]
pub struct Matrix {
    pub a: f64,
    pub b: f64,
    pub u: f64,
    pub c: f64,
    pub d: f64,
    pub v: f64,
    pub x: f64,
    pub y: f64,
    pub w: f64,
}
