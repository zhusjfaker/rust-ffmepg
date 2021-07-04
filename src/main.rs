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

  let path = "/Users/zhushijie/Desktop/m3u8-demo/b.mp4";
  let c_path = CString::new(path).unwrap().as_ptr() as *const c_char;;

  let ifmt_ctx: *mut *mut AVFormatContext = null_mut();
  // let mut ifmt_ctx: *mut AVFormatContext = std::ptr::null_mut();
  let res = unsafe { avformat_open_input(ifmt_ctx, c_path, null_mut(), null_mut()) };

  if res < 0 {
    println!("找不到视频文件");
  } else {
    println!("视频打开成功");
  }
  let durtime = unsafe { **ifmt_ctx };
  println!("res is {},time is {}", res, durtime.duration);
}
