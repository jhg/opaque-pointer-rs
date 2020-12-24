//! # FFI opaque pointers.
//! 
//! FFI to use Rust objects from C as opaque pointer.

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

/// Panic if a pointer is null.
#[inline]
fn panic_if_null<T>(pointer: *const T) {
    if pointer.is_null() {
        unreachable!("A null pointer was passed to the library, something is wrong in the C or C++ code");
    }
}

/// Convert type to raw pointer.
#[cfg(any(feature = "alloc", feature = "std"))]
#[inline]
pub fn raw<T>(data: T) -> *mut T {
    return Box::into_raw(Box::new(data));
}

/// Free pointer to type.
/// 
/// # Safety
/// 
/// Never call it twice. That could produce a HEAP error that produce a crash.
#[cfg(any(feature = "alloc", feature = "std"))]
#[inline]
pub unsafe fn free<T>(pointer: *mut T) {
    if pointer.is_null() {
        #[cfg(debug_assertions)]
        unreachable!("A null pointer was passed to the library, something is wrong in the C or C++ code");
        #[cfg(not(debug_assertions))]
        return;
    }
    // CAUTION: this is unsafe
    Box::from_raw(pointer);
    // We let drop the boxed data.
}

/// Own back from a raw pointer.
/// 
/// # Safety
/// 
/// Never call it twice. That could produce a HEAP error that produce a crash.
#[cfg(any(feature = "alloc", feature = "std"))]
#[inline]
pub unsafe fn own_back<T>(pointer: *mut T) -> T {
    panic_if_null(pointer);
    // CAUTION: this is unsafe
    let boxed = Box::from_raw(pointer);
    return *boxed;
}

/// Convert raw pointer to type to type reference.
/// 
/// # Safety
/// 
/// The pointer must be a valid reference to that value with that type.
#[inline]
pub unsafe fn object<'a, T>(pointer: *const T) -> &'a T {
    panic_if_null(pointer);
    // CAUTION: this is unsafe
    return &*pointer;
}

/// Convert raw pointer to type into type mutable reference.
/// 
/// # Safety
/// 
/// The pointer must be a valid reference to that value with that type.
#[inline]
pub unsafe fn mut_object<'a, T>(pointer: *mut T) -> &'a mut T {
    panic_if_null(pointer);
    // CAUTION: this is unsafe
    return &mut *pointer;
}
