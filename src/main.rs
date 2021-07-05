extern crate ffmpeg;
extern crate libc;

use ffmpeg::sys::avformat_open_input;
use ffmpeg::sys::AVFormatContext;
use std::ffi::CString;
use std::ptr::null_mut;

extern "C" {
  fn double_input(input: libc::c_int) -> libc::c_int;
  fn test(input: *const ::std::os::raw::c_char) -> libc::c_void;
}

fn main() {
  let input = 4;
  let output = unsafe { double_input(input) };
  println!("{} * 2 = {}", input, output);

  let path = "/Users/zhushijie/Desktop/github/rust-ffmepg/assets/b.mp4";
  let c_path = CString::new(path).expect("CString::new failed").into_raw();

  unsafe {
    test(c_path);
  };

  unsafe {
    ffmpeg::sys::av_register_all();
    let mut ifmt_ctx: *mut AVFormatContext = null_mut();
    let code = avformat_open_input(&mut ifmt_ctx, c_path, null_mut(), null_mut());
    if code < 0 {
      println!("找不到视频文件");
    } else {
      println!("视频打开成功");
    }
    let durtime = ((*ifmt_ctx).duration) as i32 / (ffmpeg::sys::AV_TIME_BASE);
    println!("res is {},time is {}", code, durtime);
  }
}
