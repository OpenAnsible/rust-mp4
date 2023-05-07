extern crate mp4;
use mp4::mp4file::parse_file;

fn main() {
    let res =
        parse_file("EvenSolberg_20230325_004746___0003.MP4").expect("Unable to read MP4 file.");
    dbg!(res);
}
