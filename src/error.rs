//! Opaque pointers errors.

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::collections::TryReserveError;
use core::str::Utf8Error;
#[cfg(feature = "std")]
use std::collections::TryReserveError;

/// Errors that can be detected by the functions of this crate.
///
/// Of course, invalid address can not be detected, then it's unsafe yet.
#[derive(Debug, PartialEq, Eq)]
pub enum PointerError {
    // Obviously, the name is the ref doc.
    Null,
    /// A pointer that was not previously lent to the FFI user.
    #[cfg(all(feature = "std", feature = "lender"))]
    Invalid,
    /// A pointer previously lent but the type is not the same.
    #[cfg(all(feature = "std", feature = "lender"))]
    InvalidType,
    /// Trying to convert to `&str` a C string which content is not valid UTF-8.
    Utf8Error(Utf8Error),
    /// Trying to alloc memory, see [`alloc::collections::TryReserveError`].
    #[cfg(any(feature = "alloc", feature = "std"))]
    TryReserveError(TryReserveError),
}

impl core::fmt::Display for PointerError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Null => write!(f, "dereference a null pointer will produce a crash"),
            Self::Invalid => write!(f, "dereference a unknown pointer could produce a crash"),
            #[cfg(all(feature = "std", feature = "lender"))]
            Self::InvalidType => write!(
                f,
                "dereference a pointer with a different type could produce errors"
            ),
            Self::Utf8Error(error) => write!(
                f,
                "the provided C string is not a valid UTF-8 string: {error}"
            ),
            #[cfg(any(feature = "alloc", feature = "std"))]
            Self::TryReserveError(error) => {
                write!(f, "can not alloc memory of internal usage: {error}")
            }
        }
    }
}

#[cfg(feature = "std")] // Waiting for https://github.com/rust-lang/rust/issues/103765
impl std::error::Error for PointerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Null | Self::Invalid | Self::InvalidType => None,
            Self::Utf8Error(error) => Some(error),
            Self::TryReserveError(error) => Some(error),
        }
    }
}

impl From<Utf8Error> for PointerError {
    fn from(error: Utf8Error) -> Self {
        Self::Utf8Error(error)
    }
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl From<TryReserveError> for PointerError {
    fn from(error: TryReserveError) -> Self {
        Self::TryReserveError(error)
    }
}
