extern crate cc;

fn main() {
    cc::Build::new()
        .file("c-lib/double.c")
        .compile("lib-display.a");
}