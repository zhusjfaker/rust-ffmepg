extern crate ffmpeg_dev;
extern crate libc;

use ffmpeg_dev::sys;
use ffmpeg_dev::sys::av_register_all;
use ffmpeg_dev::sys::avformat_open_input;
use ffmpeg_dev::sys::AVCodecParameters;
use ffmpeg_dev::sys::AVFormatContext;
use ffmpeg_dev::sys::AVMediaType_AVMEDIA_TYPE_VIDEO;
use ffmpeg_dev::sys::AV_TIME_BASE;
use std::ffi::CString;
use std::ptr::null_mut;
use std::slice::from_raw_parts;

extern "C" {
  fn double_input(input: libc::c_int) -> libc::c_int;
  fn test(input: *const ::std::os::raw::c_char) -> libc::c_void;
}

fn main() {
  let input = 4;
  let output = unsafe { double_input(input) };
  println!("{} * 2 = {}", input, output);

  let path = "/Users/zhushijie/Desktop/m3u8-demo/b.mp4";
  let c_path = CString::new(path).expect("CString::new failed").into_raw();

  unsafe {
    test(c_path);
  };

  unsafe {
    av_register_all();
    let mut ifmt_ctx: *mut AVFormatContext = null_mut();
    let code = avformat_open_input(&mut ifmt_ctx, c_path, null_mut(), null_mut());
    if code < 0 {
      println!("找不到视频文件");
    } else {
      println!("视频打开成功");
    }
    let value = (*ifmt_ctx).duration;
    let trade = AV_TIME_BASE as i64;
    let durtime = value / trade;
    let ctx = *ifmt_ctx;
    println!("res is {},time is {}", code, durtime);
    println!("stream is {}", ctx.nb_streams);

    let stream_count = ctx.nb_streams;
    let streams = from_raw_parts((*ifmt_ctx).streams, stream_count as usize)
      .iter()
      .map(|x| (*x).as_ref().expect("not null"))
      .collect::<Vec<&sys::AVStream>>();

    for (index, stream_ptr) in streams.iter().enumerate() {
      let acc: *mut AVCodecParameters = stream_ptr.codecpar;
      println!("codec_type is {},index is {}", (*acc).codec_type, index);
      if (*acc).codec_type == AVMediaType_AVMEDIA_TYPE_VIDEO {
        let codec: *mut sys::AVCodec = sys::avcodec_find_decoder((*acc).codec_id);
        if codec == null_mut() {
          println!("没有该类型的解码器!")
        }
      }
    }
  }
}
