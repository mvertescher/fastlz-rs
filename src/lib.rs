//! FastLZ compression for Rust
//!
//! Rust bindings for the FastLZ compression and decompression library.
//!

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

use core::ffi::c_void;

use fastlz_sys::fastlz_compress;
use fastlz_sys::fastlz_decompress;

/// Compress a block of data in the input buffer and returns the size of
/// compressed block. The size of input buffer is specified by length. The
/// minimum input buffer size is 16.
///
/// The output buffer must be at least 5% larger than the input buffer
/// and can not be smaller than 66 bytes.
///
/// If the input is not compressible, the return value might be larger than
/// length (input buffer size).
///
/// The input buffer and the output buffer can not overlap.
pub fn compress<'a>(input: &[u8], output: &'a mut [u8]) -> Result<&'a mut [u8], ()> {
    let in_ptr: *const c_void = input as *const _ as *const c_void;
    let out_ptr: *mut c_void = output as *mut _ as *mut c_void;
    let size = unsafe { fastlz_compress(in_ptr, input.len() as i32, out_ptr) };
    if size as usize > output.len() {
        panic!("Output buffer overflow!");
    }

    let ret: &mut [u8] = unsafe {
        core::slice::from_raw_parts_mut(out_ptr as *mut _, size as usize)
    };
    Ok(ret)
}

/// Decompress a block of compressed data and returns the size of the
/// decompressed block. If error occurs, e.g. the compressed data is
/// corrupted or the output buffer is not large enough, then 0 (zero)
/// will be returned instead.
///
/// The input buffer and the output buffer can not overlap.
///
/// Decompression is memory safe and guaranteed not to write the output buffer
/// more than what is specified in maxout.
pub fn decompress<'a>(input: &[u8], output: &'a mut [u8]) -> Result<&'a mut [u8], ()> {
    let in_ptr: *const c_void = input as *const _ as *const c_void;
    let out_ptr: *mut c_void = output as *mut _ as *mut c_void;
    let size = unsafe {
        fastlz_decompress(in_ptr, input.len() as i32, out_ptr, output.len() as i32)
    };
    let ret: &mut [u8] = unsafe {
        core::slice::from_raw_parts_mut(out_ptr as *mut _, size as usize)
    };
    Ok(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compress_and_decompress() {
        let data = "This is a sample string to compress then decompress".as_bytes();

        let mut compressed_buffer: [u8; 100] = [0; 100];
        let mut uncompressed_buffer: [u8; 100] = [0; 100];

        let compressed = compress(&data, &mut compressed_buffer).unwrap();
        let decompressed = decompress(compressed, &mut uncompressed_buffer).unwrap();

        assert_eq!(&data, &decompressed);
    }
}
