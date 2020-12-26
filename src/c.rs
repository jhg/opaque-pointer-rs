//! # Specific C types like C-like string pointers.

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
    #[cfg(any(feature = "panic-if-null", debug_assertions))]
    panic_if_null(string);
    // CAUTION: this is unsafe
    let string = CStr::from_ptr(string);
    return string.to_str();
}
