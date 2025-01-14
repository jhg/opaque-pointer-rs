#![doc = include_str!("../README.md")] // This also allow to run examples in that file.
#![allow(unsafe_code)]
#![deny(clippy::complexity)]
#![deny(clippy::cognitive_complexity)]
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

pub mod error;
use error::PointerError;

#[cfg(all(feature = "std", feature = "lender"))]
mod lender;

mod validation;

/// Get a heap-allocated raw pointer without ownership.
///
/// To back to manage the memory with ownership use [`own_back<T>()`].
///
/// # Errors
///
/// If the allocator reports a failure, then an error is returned.
#[cfg(any(feature = "alloc", feature = "std"))]
#[inline]
pub fn raw<T: 'static>(data: T) -> Result<*mut T, PointerError> {
    let pointer = Box::into_raw(Box::new(data));

    #[cfg(all(feature = "std", feature = "lender"))]
    lender::lend(pointer)?;

    Ok(pointer)
}

/// Opposite of [`raw<T>()`], to use Rust's ownership as usually.
///
/// ```
/// # let value = 0;
/// # let pointer = opaque_pointer::raw(value).unwrap();
/// drop(unsafe { opaque_pointer::own_back(pointer) });
/// ```
///
/// # Errors
///
/// The pointer must be not null as it is an obvious invalid pointer.
///
/// Also, the type must be the same as the original.
///
/// # Safety
///
/// Invalid pointer or call it twice could cause an undefined behavior or heap error and a crash.
#[cfg(any(feature = "alloc", feature = "std"))]
#[inline]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub unsafe fn own_back<T: 'static>(pointer: *mut T) -> Result<T, PointerError> {
    validation::not_null_pointer(pointer)?;
    #[cfg(all(feature = "std", feature = "lender"))]
    validation::lent_pointer(pointer)?;
    let boxed = { Box::from_raw(pointer) };

    #[cfg(all(feature = "std", feature = "lender"))]
    lender::retrieve(pointer);

    Ok(*boxed)
}

/// Reference to an object but without to own it.
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
    validation::not_null_pointer(pointer)?;
    Ok(&*pointer)
}

/// Mutable reference to an object but without back to own it.
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
    validation::not_null_pointer(pointer)?;
    Ok(&mut *pointer)
}
