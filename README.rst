Rust MP4 编解码实现 -- My Fork
=======================

:Date: 12/14 2016

.. contents::


介绍
------

`MPEG-4` (MP4) 容器格式的 `Rust` 代码实现。



测试
------

.. code:: bash

	wget "http://az29176.vo.msecnd.net/videocontent/GrizzlyPeakSF_NimiaRM_135375_1080_HD_ZH-CN.mp4"\
		 -O "test_adobe.mp4"
	ffmpeg -i test_adobe.mp4 test.mp4
	cargo run --example parse


参考
-------

*	`MPEG-4 <http://mpeg.chiariglione.org/standards/mpeg-4>`_
*	`ISO/IEC 14496-1:2010 <http://www.iso.org/iso/iso_catalogue/catalogue_tc/catalogue_detail.htm?csnumber=55688>`_ , Information technology -- Coding of audio-visual objects -- Part 1: Systems
*	`MP4 Ftyps <http://www.ftyps.com>`_
*	`MP4 Atoms <http://mp4ra.org/atoms.html>`_
*	`QuickTime Container <https://wiki.multimedia.cx/index.php/QuickTime_container>`_
*	`Apple QuickTime <http://developer.apple.com/documentation/QuickTime/QTFF/index.html>`_
*	`Adobe F4V <http://www.adobe.com/devnet/f4v.html>`_
