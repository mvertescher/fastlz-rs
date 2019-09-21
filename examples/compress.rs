//! FastLZ compression example

fn main() {
    let input = "something to compress...";
    let mut output: [u8; 100] = [0; 100];
    let _ = fastlz::compress(&input.as_bytes(), &mut output).unwrap();
}
