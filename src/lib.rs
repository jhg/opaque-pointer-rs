//! FFI opaque pointers.
//!
//! FFI to use Rust objects from C as opaque pointer.

#![allow(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![deny(clippy::complexity)]
#![deny(clippy::cognitive_complexity)]
#![allow(clippy::needless_return)] // To avoid surprise in devs more familiar with languages where return is always explicit
#![doc(html_no_source)]
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

pub mod error;

#[inline]
fn null_error_check<T>(pointer: *const T) -> Result<(), crate::error::PointerError> {
    if pointer.is_null() {
        log::error!("Using a NULL pointer as a opaque pointer to Rust data");
        return Err(crate::error::PointerError::NulPointer);
    }
    return Ok(());
}

/// Get a heap-allocated raw pointer without ownership.
///
/// To back to manage the memory with ownership use [`own_back<T>()`].
#[cfg(any(feature = "alloc", feature = "std"))]
#[inline]
pub fn raw<T>(data: T) -> *mut T {
    return Box::into_raw(Box::new(data));
}

#[deprecated(since = "0.7.2", note = "Use `own_back<T>()` instead")]
#[allow(missing_docs)]
#[cfg(any(feature = "alloc", feature = "std"))]
#[inline]
pub unsafe fn free<T>(pointer: *mut T) {
    let _ = own_back(pointer);  // Ignore the must use lint as previous behavior was ignore null pointers
}

/// Opposite of [`raw<T>()`], to use Rust's ownership as usually.
///
/// # Safety
///
/// The pointer must be a valid reference and never call it twice or behavior is undefined.
///
/// That could produce a HEAP error that produce a crash.
#[doc(alias = "free")]
#[cfg(any(feature = "alloc", feature = "std"))]
#[inline]
pub unsafe fn own_back<T>(pointer: *mut T) -> Result<T, crate::error::PointerError> {
    null_error_check(pointer)?;
    // CAUTION: this is the unsafe part of the function.
    let boxed = Box::from_raw(pointer);
    return Ok(*boxed);
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
pub unsafe fn object<'a, T>(pointer: *const T) -> Result<&'a T, crate::error::PointerError> {
    null_error_check(pointer)?;
    // CAUTION: this is unsafe
    return Ok(&*pointer);
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
pub unsafe fn mut_object<'a, T>(pointer: *mut T) -> Result<&'a mut T, crate::error::PointerError> {
    null_error_check(pointer)?;
    // CAUTION: this is unsafe
    return Ok(&mut *pointer);
}
