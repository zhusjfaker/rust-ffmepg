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
        let video_stream_idx = -1;
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
                // let pFrame:*mut sys::AVFrame = sys::av_frame_alloc();
                let mut packet: sys::AVPacket = std::mem::zeroed();
                let pframe: *mut sys::AVFrame = sys::av_frame_alloc();
                while sys::av_read_frame(ifmt_ctx, &mut packet) > 0 {
                    if packet.stream_index == video_stream_idx {
                        let sendpacket_res = sys::avcodec_send_packet(codec_ctx, &packet);
                        if sendpacket_res != 0 {
                            break;
                        }
                        let receiveframe_res = sys::avcodec_receive_frame(codec_ctx, pframe);
                        println!("receiveframe_res is {}", receiveframe_res);
                    }
                }
            }
        }
    }

    return;
}
