#![cfg(all(feature = "std", feature = "lender"))]

use lazy_static::lazy_static;
use std::collections::HashSet;
use std::sync::{RwLock, RwLockWriteGuard};

use crate::error::PointerError;

lazy_static! {
    static ref LENT_POINTERS: RwLock<HashSet<usize>> = RwLock::new(HashSet::new());
}

/// Check if a pointer was [`lent`](lend).
///
/// # Panics
///
/// If the [`RwLock`] used is poisoned, but it only happens if a panic happens
/// while holding it. And it's specially reviewed and in a small module to
/// avoid panics while holding it.
pub(super) fn is_lent<T>(pointer: *const T) -> bool {
    let Ok(lent_pointers) = LENT_POINTERS.read() else {
        log::error!("RwLock poisoned, it is not possible to check pointers");
        unreachable!();
    };
    return lent_pointers.contains(&(pointer as usize));
}

/// Use only when lend memory as a [`raw`](crate::raw) pointer.
///
/// # Panics
///
/// If the [`RwLock`] used is poisoned, but it only happens if a panic happens
/// while holding it. And it's specially reviewed and in a small module to
/// avoid panics while holding it.
pub(super) fn lend<T>(pointer: *const T) -> Result<(), PointerError> {
    let mut lent_pointers = writable_lent_pointers();

    if let Err(error) = lent_pointers.try_reserve(1) {
        log::error!("Can not alloc memory to lent a pointer: {error}");
        return Err(PointerError::from(error));
    }
    lent_pointers.insert(pointer as usize);
    return Ok(());
}

/// Use only when [`own_back`](crate::own_back) memory.
///
/// # Panics
///
/// If the [`RwLock`] used is poisoned, but it only happens if a panic happens
/// while holding it. And it's specially reviewed and in a small module to
/// avoid panics while holding it.
pub(super) fn retrieve<T>(pointer: *const T) {
    writable_lent_pointers().remove(&(pointer as usize));
}

fn writable_lent_pointers() -> RwLockWriteGuard<'static, HashSet<usize>> {
    let Ok(lent_pointers) = LENT_POINTERS.write() else {
        log::error!("RwLock poisoned, it is not possible to add or remove pointers");
        unreachable!();
    };

    lent_pointers
}
