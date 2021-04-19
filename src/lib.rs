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

#[cfg(all(feature = "std", feature = "lender"))]
#[macro_use]
extern crate lazy_static;
#[cfg(all(feature = "std", feature = "lender"))]
use std::collections::HashSet;
#[cfg(all(feature = "std", feature = "lender"))]
use std::sync::RwLock;

#[cfg(all(feature = "std", feature = "c-types"))]
pub mod c;

pub mod error;

#[cfg(all(feature = "std", feature = "lender"))]
lazy_static! {
    static ref LENT_POINTERS: RwLock<HashSet<usize>> = RwLock::new(HashSet::new());
}

#[cfg(all(feature = "std", feature = "lender"))]
#[inline]
fn invalid_error_check<T>(pointer: *const T) -> Result<(), crate::error::PointerError> {
    if !LENT_POINTERS.read().unwrap().contains(&(pointer as usize)) {
        return Err(crate::error::PointerError::Invalid);
    }
    return Ok(());
}

#[inline]
fn null_error_check<T>(pointer: *const T) -> Result<(), crate::error::PointerError> {
    if pointer.is_null() {
        log::error!("Using a NULL pointer as an opaque pointer to Rust's data");
        return Err(crate::error::PointerError::Null);
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
    LENT_POINTERS.write().unwrap().insert(pointer as usize);
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
#[deprecated(since = "0.7.2", note = "Use own_back<T>() instead")]
#[cfg(any(feature = "alloc", feature = "std"))]
#[inline]
pub unsafe fn free<T>(pointer: *mut T) {
    std::mem::drop(own_back(pointer))
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
pub unsafe fn own_back<T>(pointer: *mut T) -> Result<T, crate::error::PointerError> {
    null_error_check(pointer)?;
    #[cfg(all(feature = "std", feature = "lender"))]
    invalid_error_check(pointer)?;
    let boxed = { Box::from_raw(pointer) };
    #[cfg(all(feature = "std", feature = "lender"))]
    LENT_POINTERS.write().unwrap().remove(&(pointer as usize));
    return Ok(*boxed);
}

/// Reference to a object but without back to own it.
///
/// That's the difference with [`own_back<T>()`], you must
/// use [`own_back<T>()`] to own it again and it will be dropped.
///
/// # Errors
///
/// The pointer must be not null as it is an obvious invalid pointer.
///
/// # Safety
///
/// Invalid pointer or call it twice could cause an undefined behavior or heap error and a crash.
#[inline]
pub unsafe fn object<'a, T>(pointer: *const T) -> Result<&'a T, crate::error::PointerError> {
    null_error_check(pointer)?;
    #[cfg(all(feature = "std", feature = "lender"))]
    invalid_error_check(pointer)?;
    return Ok(&*pointer);
}

/// Mutable reference to a object but without back to own it.
///
/// That's the difference with [`own_back<T>()`], you must
/// use [`own_back<T>()`] to own it again and it will be dropped.
///
/// # Errors
///
/// The pointer must be not null as it is an obvious invalid pointer.
///
/// # Safety
///
/// Invalid pointer or call it twice could cause an undefined behavior or heap error and a crash.
#[inline]
pub unsafe fn mut_object<'a, T>(pointer: *mut T) -> Result<&'a mut T, crate::error::PointerError> {
    null_error_check(pointer)?;
    #[cfg(all(feature = "std", feature = "lender"))]
    invalid_error_check(pointer)?;
    return Ok(&mut *pointer);
}
