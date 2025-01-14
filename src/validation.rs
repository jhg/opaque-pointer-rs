use crate::error::PointerError;
#[cfg(all(feature = "std", feature = "lender"))]
use crate::lender;

#[inline]
pub fn not_null_pointer<T>(pointer: *const T) -> Result<(), PointerError> {
    if pointer.is_null() {
        log::error!("Using a NULL pointer as an opaque pointer to Rust's data");
        return Err(PointerError::Null);
    }

    Ok(())
}

#[inline]
pub fn lent_pointer<T: 'static>(pointer: *const T) -> Result<(), PointerError> {
    #[cfg(all(feature = "std", feature = "lender"))]
    match lender::lent_type_of(pointer) {
        Some(type_id) if type_id != std::any::TypeId::of::<T>() => {
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
