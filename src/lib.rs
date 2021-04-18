#![feature(extended_key_value_attributes)]
#![doc = include_str!("../README.md")] // This also allow to run examples in that file.
#![allow(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![deny(clippy::complexity)]
#![deny(clippy::cognitive_complexity)]
#![allow(clippy::needless_return)]
#![no_std]

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;
#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::boxed::Box;

#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "std")]
use std::boxed::Box;

#[cfg(all(feature = "std", feature = "c-types"))]
pub mod c;

#[cfg(any(feature = "panic-if-null", debug_assertions))]
#[inline]
fn panic_if_null<T>(pointer: *const T) {
    if pointer.is_null() {
        unreachable!(
            "A null pointer was passed to the library, something is wrong in the C or C++ code"
        );
    }
}

/// Convert type to raw pointer ready to be used as opaque pointer.
#[cfg(any(feature = "alloc", feature = "std"))]
#[inline]
pub fn raw<T>(data: T) -> *mut T {
    return Box::into_raw(Box::new(data));
}

/// Free memory of a previous type converted to raw pointer.
///
/// # Safety
///
/// The pointer must be a valid reference and never call it twice or behavior is undefined.
///
/// That could produce a HEAP error that produce a crash.
#[cfg(any(feature = "alloc", feature = "std"))]
#[inline]
pub unsafe fn free<T>(pointer: *mut T) {
    if pointer.is_null() {
        #[cfg(debug_assertions)]
        unreachable!(
            "A null pointer was passed to the library, something is wrong in the C or C++ code"
        );
        #[cfg(not(debug_assertions))]
        return;
    }
    // CAUTION: this is unsafe
    Box::from_raw(pointer);
    // We let drop the boxed data.
}

/// Own back from a raw pointer to use Rust ownership as usually.
///
/// # Safety
///
/// The pointer must be a valid reference and never call it twice or behavior is undefined.
///
/// That could produce a HEAP error that produce a crash.
#[cfg(any(feature = "alloc", feature = "std"))]
#[inline]
pub unsafe fn own_back<T>(pointer: *mut T) -> T {
    #[cfg(any(feature = "panic-if-null", debug_assertions))]
    panic_if_null(pointer);
    // CAUTION: this is unsafe
    let boxed = Box::from_raw(pointer);
    return *boxed;
}

/// Convert raw pointer to type to type reference but without back to own it.
///
/// That's the main difference with `own_back<T>`, it does not back to use ownership
/// and values will not be dropped.
///
/// # Safety
///
/// The pointer must be a valid reference and never call it twice or behavior is undefined.
///
/// That could produce a HEAP error that produce a crash.
#[inline]
pub unsafe fn object<'a, T>(pointer: *const T) -> &'a T {
    #[cfg(any(feature = "panic-if-null", debug_assertions))]
    panic_if_null(pointer);
    // CAUTION: this is unsafe
    return &*pointer;
}

/// Convert raw pointer to type into type mutable reference but without back to own it.
///
/// That's the main difference with `own_back<T>`, it does not back to use ownership
/// and values will not be dropped.
///
/// # Safety
///
/// The pointer must be a valid reference and never call it twice or behavior is undefined.
///
/// That could produce a HEAP error that produce a crash.
#[inline]
pub unsafe fn mut_object<'a, T>(pointer: *mut T) -> &'a mut T {
    #[cfg(any(feature = "panic-if-null", debug_assertions))]
    panic_if_null(pointer);
    // CAUTION: this is unsafe
    return &mut *pointer;
}
