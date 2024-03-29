//! Opaque pointers errors.

use core::str::Utf8Error;

/// Errors that can be detected by the functions of this crate.
///
/// Of course, invalid address can not be detected, then it's unsafe yet.
#[allow(clippy::module_name_repetitions)] // Like std::error::Error, it is for pointer errors
#[derive(Debug, PartialEq, Eq)]
pub enum PointerError {
    #[allow(missing_docs)] // Obviously, the name is the ref doc.
    Null,
    /// A pointer that was not previously lent to the FFI user.
    Invalid,
    /// Trying to convert to `&str` a C string which content is not valid UTF-8.
    Utf8Error(Utf8Error),
}

impl core::fmt::Display for PointerError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match *self {
            Self::Null => {
                write!(f, "dereference a null pointer will produce a crash")
            }
            Self::Invalid => {
                write!(f, "dereference a unknown pointer could produce a crash")
            }
            Self::Utf8Error(..) => {
                write!(f, "the provided C string is not a valid UTF-8 string")
            }
        }
    }
}

#[cfg(feature = "std")] // Waiting for https://github.com/rust-lang/rust/issues/103765
impl std::error::Error for PointerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Null | Self::Invalid => None,
            Self::Utf8Error(ref e) => Some(e),
        }
    }
}

impl From<Utf8Error> for PointerError {
    fn from(err: Utf8Error) -> Self {
        Self::Utf8Error(err)
    }
}
