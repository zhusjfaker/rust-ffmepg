extern crate ffmpeg_dev;
extern crate libc;

use ffmpeg_dev::sys::avformat_open_input;
use ffmpeg_dev::sys::AVFormatContext;
use std::ffi::CString;
use std::ptr::null_mut;

extern "C" {
  fn double_input(input: libc::c_int) -> libc::c_int;
}

fn main() {
  let input = 4;
  let output = unsafe { double_input(input) };
  println!("{} * 2 = {}", input, output);

  let path = "/Users/zhushijie/Downloads/m3u8-demo/a.mp4";
  let c_path = CString::new(path).unwrap().as_ptr();

  let ifmt_ctx: *mut *mut AVFormatContext = null_mut();
  let res = unsafe { avformat_open_input(ifmt_ctx, c_path, null_mut(), null_mut()) };

  let durtime = unsafe { **ifmt_ctx };
  println!("res is {},time is {}", res, durtime.duration);
}
