
// https://wiki.multimedia.cx/index.php/QuickTime_container
// http://developer.apple.com/documentation/QuickTime/QTFF/index.html

// box types: http://mp4ra.org/atoms.html

/**
	Box Struct:
	
		size(u32), type(u32), largesize(u64),
		data

其中, `size` 指明了整个 `box` 的大小, 包括 `header` 部分.
如果 `box` 大小超过了 `u32` 的最大数值, `size` 就被设置为 `1` ,
并用接下来的 `8位` u64 来存放大小。


Top level Box:

	ftyp
	moov
	mdat
**/

pub struct Box {
	header: {size: u32, type: u32, largesize: u64},
	// data
}

