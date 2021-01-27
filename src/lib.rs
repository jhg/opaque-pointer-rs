//! # FFI opaque pointers.
//! 
//! FFI to use Rust objects from C as opaque pointer.

#![allow(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![deny(clippy::complexity)]
#![deny(clippy::cognitive_complexity)]
#![allow(clippy::needless_return)] // To avoid surprise in devs more familiar where return is always explicit

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
        log::error!("Trying to use a NULL pointer as a opaque pointer to Rust data");
        unreachable!("Trying to use a NULL pointer as a opaque pointer to Rust data");
    }
}

/// Get a heap-allocated raw pointer without ownership.
/// 
/// To back to manage the memory with ownership use [`own_back<T>()`].
#[cfg(any(feature = "alloc", feature = "std"))]
#[inline]
pub fn raw<T>(data: T) -> *mut T {
    return Box::into_raw(Box::new(data));
}

#[deprecated(
    since = "0.7.2",
    note = "Please use the own_back function instead"
)]
#[cfg(any(feature = "alloc", feature = "std"))]
#[inline]
pub unsafe fn free<T>(pointer: *mut T) {
    #[cfg(any(feature = "panic-if-null", debug_assertions))]
    panic_if_null(pointer);
    // TODO: decide if remove this try to avoid crash when free a null pointer
    #[cfg(not(any(feature = "panic-if-null", debug_assertions)))]
    if pointer.is_null() {
        log::warn!("Trying to free a NULL pointer was ignored");
        return;
    }
    // CAUTION: this is unsafe
    Box::from_raw(pointer);
    // We let drop the boxed data.
}

/// Opposite of [`raw<T>()`], to use Rust's ownership as usually.
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
    // CAUTION: this is the unsafe part of the function.
    let boxed = Box::from_raw(pointer);
    return *boxed;
}

/// Reference to a object but without back to own it.
/// 
/// That's the difference with [`own_back<T>()`], you must
/// use [`own_back<T>()`] to own it again and it will be dropped.
/// 
/// # Safety
/// 
/// Invalid pointer or call it twice could cause an undefined behavior or heap error and a crash.
#[inline]
pub unsafe fn object<'a, T>(pointer: *const T) -> &'a T {
    #[cfg(any(feature = "panic-if-null", debug_assertions))]
    panic_if_null(pointer);
    // CAUTION: this is unsafe
    return &*pointer;
}

/// Mutable reference to a object but without back to own it.
/// 
/// That's the difference with [`own_back<T>()`], you must
/// use [`own_back<T>()`] to own it again and it will be dropped.
/// 
/// # Safety
/// 
/// Invalid pointer or call it twice could cause an undefined behavior or heap error and a crash.
#[inline]
pub unsafe fn mut_object<'a, T>(pointer: *mut T) -> &'a mut T {
    #[cfg(any(feature = "panic-if-null", debug_assertions))]
    panic_if_null(pointer);
    // CAUTION: this is unsafe
    return &mut *pointer;
}
