//! # C types FFI opaque pointers.
//! 
//! Specific C types like C-like string pointers.

#![cfg(all(feature = "std", feature = "c-types"))]

use std::os::raw::c_char;
use std::ffi::CStr;

use super::panic_if_null;

/// Reference to a C string.
/// 
/// # Safety
/// 
/// The pointer must be a valid reference to that value with that type.
/// 
/// # Panics
/// 
/// This could panic if the C string is not a valid UTF-8 string.
#[must_use]
#[inline]
pub unsafe fn ref_str<'a>(string: *const c_char) -> &'a str {
    panic_if_null(string);
    // CAUTION: this is unsafe
    let string = CStr::from_ptr(string);
    return string.to_str().expect("Invalid UTF-8 string from C or C++ code");
}
