//! Opaque pointers errors.

#[cfg(all(feature = "std", feature = "c-types"))]
use std::str::Utf8Error;

/// Errors that can be detected by the functions of this crate.
///
/// Of course, invalid address can not be detected, then it's unsafe yet.
#[allow(clippy::module_name_repetitions)] // Like std::error::Error, it is for pointer errors
#[derive(Debug)]
pub enum PointerError {
    #[allow(missing_docs)] // Obviously, the name is the ref doc.
    NulPointer,
    /// Trying to convert to `&str` a C string which content is not valid UTF-8.
    Utf8Error(Utf8Error),
}

impl std::fmt::Display for PointerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            PointerError::NulPointer => {
                write!(f, "dereference a null pointer will produce a crash")
            }
            PointerError::Utf8Error(..) => {
                write!(f, "the provided C string is not a valid UTF-8 string")
            }
        }
    }
}

impl std::error::Error for PointerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            PointerError::NulPointer => None,
            // The cause is the underlying implementation error type. Is implicitly
            // cast to the trait object `&error::Error`. This works because the
            // underlying type already implements the `Error` trait.
            PointerError::Utf8Error(ref e) => Some(e),
        }
    }
}

impl From<Utf8Error> for PointerError {
    fn from(err: Utf8Error) -> Self {
        Self::Utf8Error(err)
    }
}