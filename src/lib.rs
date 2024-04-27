#![doc = include_str!("../README.md")] // This also allow to run examples in that file.
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
use error::PointerError;

#[cfg(all(feature = "std", feature = "lender"))]
mod lender;

#[inline]
fn validate_pointer_is_not_null<T>(pointer: *const T) -> Result<(), PointerError> {
    if pointer.is_null() {
        log::error!("Using a NULL pointer as an opaque pointer to Rust's data");
        return Err(PointerError::Null);
    }
    return Ok(());
}

#[inline]
fn validate_pointer<T>(pointer: *const T) -> Result<(), PointerError> {
    validate_pointer_is_not_null(pointer)?;
    #[cfg(all(feature = "std", feature = "lender"))]
    if !lender::is_lent(pointer) {
        log::error!("Using an invalid pointer as an opaque pointer to Rust's data");
        return Err(PointerError::Invalid);
    }
    return Ok(());
}

/// Get a heap-allocated raw pointer without ownership.
///
/// To back to manage the memory with ownership use [`own_back<T>()`].
#[cfg(any(feature = "alloc", feature = "std"))]
#[inline]
pub fn raw<T>(data: T) -> *mut T {
    let pointer = Box::into_raw(Box::new(data));
    #[cfg(all(feature = "std", feature = "lender"))]
    lender::lend(pointer);
    return pointer;
}

/// Call to [`own_back<T>()`] ignoring the result.
///
/// This is deprecated and will be removed in the version 0.9.0 then you can do this:
///
/// ```no_run
/// # let value = 0;
/// # let pointer = opaque_pointer::raw(value);
/// std::mem::drop(unsafe { opaque_pointer::own_back(pointer) });
/// ```
///
/// # Safety
///
/// See [`own_back<T>()`] reference doc.
#[deprecated(
    since = "0.7.2",
    note = "Use own_back<T>() instead, it'll be removed at version 0.9.0"
)]
#[cfg(any(feature = "alloc", feature = "std"))]
#[inline]
pub unsafe fn free<T>(pointer: *mut T) {
    core::mem::drop(own_back(pointer));
}

/// Opposite of [`raw<T>()`], to use Rust's ownership as usually.
///
/// # Errors
///
/// The pointer must be not null as it is an obvious invalid pointer.
///
/// # Safety
///
/// Invalid pointer or call it twice could cause an undefined behavior or heap error and a crash.
#[doc(alias = "free")]
#[cfg(any(feature = "alloc", feature = "std"))]
#[inline]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub unsafe fn own_back<T>(pointer: *mut T) -> Result<T, PointerError> {
    validate_pointer(pointer)?;
    let boxed = { Box::from_raw(pointer) };
    #[cfg(all(feature = "std", feature = "lender"))]
    lender::retrieve(pointer);
    return Ok(*boxed);
}

/// Reference to a object but without to own it.
///
/// # Errors
///
/// The pointer must be not null as it is an obvious invalid pointer.
///
/// # Safety
///
/// Invalid pointer could cause an undefined behavior or heap error and a crash.
#[inline]
pub unsafe fn object<'a, T>(pointer: *const T) -> Result<&'a T, PointerError> {
    validate_pointer_is_not_null(pointer)?;
    return Ok(&*pointer);
}

/// Mutable reference to a object but without back to own it.
///
/// # Errors
///
/// The pointer must be not null as it is an obvious invalid pointer.
///
/// # Safety
///
/// Invalid pointer could cause an undefined behavior or heap error and a crash.
#[inline]
pub unsafe fn mut_object<'a, T>(pointer: *mut T) -> Result<&'a mut T, PointerError> {
    validate_pointer_is_not_null(pointer)?;
    return Ok(&mut *pointer);
}
