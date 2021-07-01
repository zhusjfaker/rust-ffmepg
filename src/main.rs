extern crate libc;
extern crate ffmpeg_dev;

// use ffmpeg_dev::sys::AVFormatContext;

extern {
  fn double_input(input: libc::c_int) -> libc::c_int;
}

fn main() {
  let input = 4;
  let output = unsafe { double_input(input) };
  println!("{} * 2 = {}", input, output);
 
  // let afc: AVFormatContext;
}
