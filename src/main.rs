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
// use std::fs::File;
// use std::io::Write;
use std::mem::size_of;
use std::ptr::null_mut;
use std::slice::from_raw_parts;

extern "C" {
  fn double_input(input: libc::c_int) -> libc::c_int;
  fn test(input: *const ::std::os::raw::c_char) -> libc::c_void;
}

fn saveframe(frame: *mut sys::AVFrame, index: i32, with: i32, height: i32, pix_fmt: i32) {
  unsafe {
    let project_path = "/Users/zhushijie/Desktop/github/rust-ffmepg";
    let filepath = format!("{}/{}.jpg", project_path, index.to_string());
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

    const AV_NUM_DATA_POINTERS: usize = sys::AV_NUM_DATA_POINTERS as usize;
    let mut cp_data = [null_mut(); AV_NUM_DATA_POINTERS];
    let mut linesize = [0, AV_NUM_DATA_POINTERS as i32];

    sys::av_image_copy(
      cp_data.as_mut_ptr(),
      linesize.as_mut_ptr(),
      (*frame).data.as_mut_ptr() as *mut *const u8,
      (*frame).linesize.as_mut_ptr(),
      pix_fmt,
      with,
      height,
    );

    println!("数据填充完毕");

    // let mut pic_file = File::create(filepath).expect("create failed");
    // let fs_data = &mut (*frame).data;
    // pic_file.write_all(fs_data);
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
    let mut video_stream_idx: Vec<usize> = Vec::new();
    // let audio_stream_idx = -1;
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
        let img_convert_ctx: *mut sys::SwsContext = sys::sws_getContext(
          (*codec_ctx).width,
          (*codec_ctx).height,
          (*codec_ctx).pix_fmt,
          (*codec_ctx).width,
          (*codec_ctx).height,
          sys::AVPixelFormat_AV_PIX_FMT_RGB24,
          sys::SWS_BILINEAR as i32,
          null_mut(),
          null_mut(),
          null_mut(),
        );

        let mut pic_index = 1;
        let mut framefinished: i32 = std::mem::zeroed();

        while sys::av_read_frame(ifmt_ctx, packet) >= 0 {
          // let stream_index = (*packet).stream_index as usize;
          // if video_stream_idx.contains(&stream_index) && (*packet).flags == 1 {
          if (*packet).flags == 1 {
            // let sendpacket_res = sys::avcodec_send_packet(codec_ctx, packet);
            // // println!("sendpacket_res is {}", sendpacket_res);
            // if sendpacket_res != 0 {
            //   break;
            // }
            // let receiveframe_res = sys::avcodec_receive_frame(codec_ctx, pframe);
            // if receiveframe_res != 0 {
            //   break;
            // }
            sys::avcodec_decode_video2(codec_ctx, pframe, &mut framefinished, packet);

            pic_index += 1;
            if pic_index < 50 && framefinished != std::mem::zeroed() {
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
                pframe,
                pic_index,
                (*codec_ctx).width,
                (*codec_ctx).height,
                (*codec_ctx).pix_fmt,
              );

              sys::av_frame_unref(pframe);
            } else {
              break;
            }
            // println!("receiveframe_res is {}", receiveframe_res);
            // sys::sws_scale(img_convert_ctx,(*pframe).data,(*pframe).linesize,0,(*codec_ctx).height)
          }
        }
      }
    }
  }

  return;
}
