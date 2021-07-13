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
use std::mem::size_of;
// use std::mem::zeroed;
use std::ptr::null_mut;
use std::slice::from_raw_parts;

extern "C" {
  fn double_input(input: libc::c_int) -> libc::c_int;
  fn test(input: *const ::std::os::raw::c_char) -> libc::c_void;
}

fn saveframe(frame: *mut sys::AVFrame, index: i32, _with: i32, _height: i32, pix_fmt: i32) {
  unsafe {
    let project_path = "/Users/zhushijie/Desktop/github/rust-ffmepg";
    let filepath = format!("{}/{}.bmp", project_path, index.to_string());
    println!("pic name is {}", filepath);

    let bufsize = sys::av_image_alloc(
      (*frame).data.as_mut_ptr(),
      (*frame).linesize.as_mut_ptr(),
      (*frame).width,
      (*frame).height,
      pix_fmt,
      32,
    );
    println!("当前图片计算大小->{}", bufsize);

    let fp = libc::fopen(
      CString::new(filepath)
        .expect("CString::new failed")
        .into_raw(),
      CString::new("w").expect("CString::new failed").into_raw(),
    );

    let data = (*frame).data[0] as *const libc::c_void;

    libc::fwrite(data, 1, bufsize as usize, fp);

    libc::fclose(fp);
  }
}

fn main() {
  let input = 4;
  let output = unsafe { double_input(input) };
  println!("{} * 2 = {}", input, output);

  let path = "/Users/zhushijie/Desktop/m3u8-demo/test.mp4";
  // let path = "/Users/zhushijie/Desktop/demo/m3u8-demo/a.mp4";
  let c_path = CString::new(path).expect("CString::new failed").into_raw();

  unsafe {
    test(c_path);
  };

  unsafe {
    av_register_all();
    sys::avdevice_register_all();
    let mut video_stream_idx: Vec<usize> = Vec::new();
    let txt_inputformat = CString::new("video4linux2")
      .expect("CString::new failed")
      .into_raw();
    let input_fmt = sys::av_find_input_format(txt_inputformat);
    // let audio_stream_idx = -1;
    let mut ifmt_ctx: *mut AVFormatContext = sys::avformat_alloc_context();
    let code = avformat_open_input(&mut ifmt_ctx, c_path, input_fmt, null_mut());
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
        video_stream_idx.push(index);
        let codec: *mut sys::AVCodec = sys::avcodec_find_decoder((*acc).codec_id);
        if codec == null_mut() {
          println!("没有该类型的解码器!");
          break;
        }
        let codec_ctx: *mut sys::AVCodecContext = sys::avcodec_alloc_context3(codec);
        sys::avcodec_parameters_to_context(codec_ctx, acc);
        let res = sys::avcodec_open2(codec_ctx, codec, null_mut());
        if res != 0 {}
        println!("解码器打开成功");
        let packet: *mut sys::AVPacket = sys::av_packet_alloc();
        let pframe: *mut sys::AVFrame = sys::av_frame_alloc();
        let tr_frame: *mut sys::AVFrame = sys::av_frame_alloc();

        let numbytes = sys::av_image_get_buffer_size(
          sys::AVPixelFormat_AV_PIX_FMT_RGB24,
          (*codec_ctx).width,
          (*codec_ctx).height,
          1,
        ) as usize;

        let buffer = sys::av_malloc(numbytes * size_of::<u8>()) as *mut u8;
        sys::av_image_fill_arrays(
          (*tr_frame).data.as_mut_ptr(),
          (*tr_frame).linesize.as_mut_ptr(),
          buffer,
          sys::AVPixelFormat_AV_PIX_FMT_RGB24,
          (*codec_ctx).width,
          (*codec_ctx).height,
          1,
        );

        println!(
          "width: {} height: {} pix_fmt: {}",
          (*codec_ctx).width,
          (*codec_ctx).height,
          (*codec_ctx).pix_fmt
        );

        let mut pic_index = 1;
        let mut framefinished: i32 = 0;

        while sys::av_read_frame(ifmt_ctx, packet) >= 0 {
          let stream_index = (*packet).stream_index as usize;
          if video_stream_idx.contains(&stream_index) && (*packet).flags == 1 {
            sys::avcodec_decode_video2(codec_ctx, pframe, &mut framefinished, packet);

            pic_index += 1;
            if pic_index < 20 && framefinished > 0 {
              let img_convert_ctx: *mut sys::SwsContext = sys::sws_getContext(
                (*pframe).width,
                (*pframe).height,
                (*pframe).format,
                (*pframe).width,
                (*pframe).height,
                sys::AVPixelFormat_AV_PIX_FMT_RGB24,
                sys::SWS_BILINEAR as i32,
                null_mut(),
                null_mut(),
                null_mut(),
              );

              let h = sys::sws_scale(
                img_convert_ctx,
                (*pframe).data.as_ptr() as *mut *const u8,
                (*pframe).linesize.as_ptr(),
                0,
                (*codec_ctx).height,
                (*tr_frame).data.as_ptr(),
                (*tr_frame).linesize.as_ptr(),
              );

              println!("重新计算的高端:{}", h);

              saveframe(
                tr_frame,
                pic_index,
                (*codec_ctx).width,
                (*codec_ctx).height,
                sys::AVPixelFormat_AV_PIX_FMT_RGB24,
              );
            } else {
              break;
            }
          }
        }

        sys::av_frame_unref(pframe);
        sys::av_frame_unref(tr_frame);
        sys::avcodec_close(codec_ctx);
        sys::avformat_close_input(&mut ifmt_ctx);
      }
    }
  }

  return;
}
