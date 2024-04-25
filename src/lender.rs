#![cfg(all(feature = "std", feature = "lender"))]

use lazy_static::lazy_static;
use std::collections::HashSet;
use std::sync::{RwLock, RwLockWriteGuard};

lazy_static! {
    static ref LENT_POINTERS: RwLock<HashSet<usize>> = RwLock::new(HashSet::new());
}

/// Check if a pointer was lent.
///
/// # Panics
///
/// If the RwLock used is poisoned, but it only happens if a panic happens
/// while holding it. And it's specially reviewed and in a small module to
/// avoid panics while holding it.
pub(super) fn is_lent<T>(pointer: *const T) -> bool {
    let Ok(lent_pointers) = LENT_POINTERS.read() else {
        log::error!("RwLock poisoned, it is not possible to check pointers");
        unreachable!("RwLock poisoned when there is not panics in code that can hold it");
    };
    return lent_pointers.contains(&(pointer as usize));
}

/// Add a pointer to the list.
///
/// # Panics
///
/// If the RwLock used is poisoned, but it only happens if a panic happens
/// while holding it. And it's specially reviewed and in a small module to
/// avoid panics while holding it.
pub(super) fn lend<T>(pointer: *const T) {
    // Use try_reserve in nightly until it is available in stable
    writable_lent_pointers().insert(pointer as usize);
}

/// Remove a pointer from the list.
///
/// # Panics
///
/// If the RwLock used is poisoned, but it only happens if a panic happens
/// while holding it. And it's specially reviewed and in a small module to
/// avoid panics while holding it.
pub(super) fn retrieve<T>(pointer: *const T) {
    writable_lent_pointers().remove(&(pointer as usize));
}

fn writable_lent_pointers() -> RwLockWriteGuard<'_, HashSet<usize>> {
    let Ok(lent_pointers) = LENT_POINTERS.write() else {
        log::error!("RwLock poisoned, it is not possible to add or remove pointers");
        unreachable!("RwLock poisoned when there is not panics in code that can hold it");
    };
    return lent_pointers;
}
