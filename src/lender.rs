#![cfg(all(feature = "std", feature = "lender"))]

use lazy_static::lazy_static;
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{RwLock, RwLockWriteGuard};

use crate::error::PointerError;

lazy_static! {
    static ref LENT_POINTERS: RwLock<HashMap<usize, TypeId>> = RwLock::new(HashMap::new());
}

/// Check if a pointer was [`lent`](lend).
///
/// # Panics
///
/// If the [`RwLock`] used is poisoned, but it only happens if a panic happens
/// while holding it. And it's specially reviewed and in a small module to
/// avoid panics while holding it.
pub(super) fn lent_type_of<T>(pointer: *const T) -> Option<TypeId> {
    let Ok(lent_pointers) = LENT_POINTERS.read() else {
        log::error!("RwLock poisoned, it is not possible to check pointers");
        unreachable!();
    };

    lent_pointers.get(&(pointer as usize)).copied()
}

fn writable_lent_pointers() -> RwLockWriteGuard<'static, HashMap<usize, TypeId>> {
    let Ok(lent_pointers) = LENT_POINTERS.write() else {
        log::error!("RwLock poisoned, it is not possible to add or remove pointers");
        unreachable!("RwLock poisoned, it is not possible to add or remove pointers");
    };

    lent_pointers
}

/// Use only when lend memory as a [`raw`](crate::raw) pointer.
///
/// # Panics
///
/// If the [`RwLock`] used is poisoned, but it only happens if a panic happens
/// while holding it. And it's specially reviewed and in a small module to
/// avoid panics while holding it.
pub(super) fn lend<T: 'static>(pointer: *const T) -> Result<(), PointerError> {
    let mut lent_pointers = writable_lent_pointers();

    if let Err(error) = lent_pointers.try_reserve(1) {
        log::error!("Can not alloc memory to lent a pointer: {error}");
        return Err(PointerError::from(error));
    }

    lent_pointers.insert(pointer as usize, TypeId::of::<T>());
    Ok(())
}

/// Use only when [`own_back`](crate::own_back) memory.
///
/// # Panics
///
/// If the [`RwLock`] used is poisoned, but it only happens if a panic happens
/// while holding it. And it's specially reviewed and in a small module to
/// avoid panics while holding it.
pub(super) fn retrieve<T: 'static>(pointer: *const T) -> Result<(), PointerError> {
    match writable_lent_pointers().remove(&(pointer as usize)) {
        Some(type_id) if type_id != TypeId::of::<T>() => {
            log::error!(
                "Using a pointer with a different type as an opaque pointer to Rust's data"
            );
            Err(PointerError::InvalidType)
        }
        None => {
            log::error!("Using an invalid pointer as an opaque pointer to Rust's data");
            Err(PointerError::Invalid)
        }
        _ => Ok(()),
    }
}
