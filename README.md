# Rust MP4 Codec Implementation -- My Fork

A Rust implementation of the MPEG-4 (MP4) container format. My fork cleans up some of the code (work in progress) and adds functionality to make data acccess easier. Note that this crate only contains the metadata information, and not the actual media.

Much work remains in documenting this and making it more robust.

## Example

```bash
wget "http://az29176.vo.msecnd.net/videocontent/GrizzlyPeakSF_NimiaRM_135375_1080_HD_ZH-CN.mp4" -O "test_adobe.mp4"
ffmpeg -i test_adobe.mp4 test.mp4
cargo run --example parse
```

## References

* [MPEG-4](http://mpeg.chiariglione.org/standards/mpeg-4)
* [ISO/IEC 14496-1:2010](http://www.iso.org/iso/iso_catalogue/catalogue_tc/catalogue_detail.htm?csnumber=55688) -- Information technology, Coding of audio-visual objects, Part 1: Systems
  * [MP4 Ftyps](http://www.ftyps.com)
  * [MP4 Atoms](http://mp4ra.org/atoms.html)
* [QuickTime Container](https://wiki.multimedia.cx/index.php/QuickTime_container)
* [Apple QuickTime](http://developer.apple.com/documentation/QuickTime/QTFF/index.html)
* [Adobe F4V](http://www.adobe.com/devnet/f4v.html)
