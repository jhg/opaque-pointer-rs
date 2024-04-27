use crate::error::PointerError;
use crate::lender;

#[inline]
pub fn not_null_pointer<T>(pointer: *const T) -> Result<(), PointerError> {
    if pointer.is_null() {
        log::error!("Using a NULL pointer as an opaque pointer to Rust's data");
        return Err(PointerError::Null);
    }
    return Ok(());
}

#[inline]
pub fn lent_pointer<T>(pointer: *const T) -> Result<(), PointerError> {
    not_null_pointer(pointer)?;
    #[cfg(all(feature = "std", feature = "lender"))]
    if !lender::is_lent(pointer) {
        log::error!("Using an invalid pointer as an opaque pointer to Rust's data");
        return Err(PointerError::Invalid);
    }
    return Ok(());
}
