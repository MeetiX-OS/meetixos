/*! `&str` and `&[u8]` utilities */

use core::{
    cmp::min,
    slice,
    str
};

/**
 * Copies an `&str` into a `&mut [u8]` buffer using the min of the two
 * lengths
 */
pub fn copy_str_to_u8_buf(dst: &mut [u8], src: &str) {
    let min = min(dst.len(), src.len());
    dst[..min].copy_from_slice(&src.as_bytes()[..min]);
}

/**
 * Constructs an `&str` from the `&[u8]` given, performing then an unchecked
 * UTF-8 conversion
 */
pub fn u8_slice_to_str_slice(slice: &[u8]) -> &str {
    unsafe { str::from_utf8_unchecked(slice) }
}

/**
 * Constructs an `&str` from the raw parts given, performing then an
 * unchecked UTF-8 conversion
 */
pub fn u8_ptr_to_str_slice<'a>(ptr: *const u8, len: usize) -> &'a str {
    unsafe {
        let slice = slice::from_raw_parts(ptr, len);
        str::from_utf8_unchecked(slice)
    }
}

/**
 * Returns the length of the given slice interpreting it as ascii
 * string-slice
 */
pub fn str_len(slice: &[u8]) -> usize {
    for (i, c) in slice.iter().enumerate() {
        if *c == b'\0' {
            return i;
        }
    }
    0
}
