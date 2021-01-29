//! # Specific C types like C-like string pointers.
//!
//! You need to use the `c-types` feature to use this module.

#![cfg(all(feature = "std", feature = "c-types"))]

use std::os::raw::c_char;
use std::ffi::CStr;
use std::str::Utf8Error;

#[cfg(any(feature = "panic-if-null", debug_assertions))]
use super::panic_if_null;

/// Convert a reference to a C string into a static reference to Rust `str`.
/// 
/// # Safety
/// 
/// The pointer must be a valid reference or behavior is undefined.
/// 
/// # Errors
/// 
/// If the C string is not a valid UTF-8 string.
#[must_use]
#[inline]
pub unsafe fn ref_str<'a>(string: *const c_char) -> Result<&'a str, Utf8Error> {
    // ATTENTION! 'a lifetime is required, does NOT REMOVE it
    // see commit 5a03be91d2da8909986db7c54650f3a7863a91ff fixing 3a1d15f33e8e418ef6bee2b7b9e096780bd2c8ac
    #[cfg(any(feature = "panic-if-null", debug_assertions))]
    panic_if_null(string);
    // CAUTION: this is unsafe
    let string = CStr::from_ptr(string);
    return string.to_str();
}
