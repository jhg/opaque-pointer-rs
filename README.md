# opaque-pointer-rs

Generic functions to work with opaque pointers when use FFI to expose Rust structs

[![Crates.io](https://img.shields.io/crates/v/opaque-pointer)](https://crates.io/crates/opaque-pointer)
[![Crates.io](https://img.shields.io/crates/l/opaque-pointer)](https://unlicense.org/)
[![Crates.io](https://img.shields.io/crates/d/opaque-pointer)](https://crates.io/crates/opaque-pointer)

## Basic usage

With this crate you can manage raw pointers easily to expose `structs` that will be
 use as opaque pointers from C or C++ calling to Rust functions to use it. This
 can be used with [cbindgen](https://crates.io/crates/cbindgen) crate with option `parse.parse_deps = true`
 for it will generate opaque C/C++ `structs` to use pointers in the arguments.

You can find more information about using Rust from other languages in
 [The Rust FFI Omnibus objects section](http://jakegoulding.com/rust-ffi-omnibus/objects/)
 of [Jake Goulding](https://github.com/shepmaster).

## Examples

Creating FFIs to use a Rust's `struct` methods from C or C++:

```rust
struct TestIt { value: u8 }

impl TestIt {
    pub fn add(&mut self, value: u8) { self.value += value }
    pub fn get(&self) -> u8 { self.value }
}

/// Ownership will NOT control the heap-allocated memory until own it back.
#[no_mangle]
pub extern fn test_it_new(value: u8) -> *mut TestIt {
    return opaque_pointer::raw(TestIt { value });
}

/// Drop (free memory of) Rust's TestIt object as usually.
#[no_mangle]
pub extern fn test_it_free(test_it: *mut TestIt) {
    let test_it = unsafe { opaque_pointer::free(test_it) };
}

#[no_mangle]
pub extern fn test_it_add(test_it: *mut TestIt, value: u8) -> Result<(), opaque_pointer::error::PointerError> {
    let test_it = unsafe { opaque_pointer::mut_object(test_it)? };
    test_it.add(value);
    // Here will NOT be dropped, the pointer continues been valid.
    return Ok(());
}

#[no_mangle]
pub extern fn test_it_get(test_it: *const TestIt) -> Result<u8, opaque_pointer::error::PointerError> {
    let test_it = unsafe { opaque_pointer::object(test_it)? };
    return Ok(test_it.get());
    // Here will NOT be dropped, the pointer continues been valid.
}
```

The previous example is compiled when tests are run. If you have an error
with that code, please, [open a issue](https://github.com/jhg/opaque-pointer-rs/issues?q=is%3Aissue+is%3Aopen).

## Features

- `std`: activated by default, it is required for c-types FFI.
- `alloc`: required if compile without std.
- `c-types`: FFI for C types, requires std feature.

## Panic & unwind in FFI functions

This create returns a result to avoid panic if the pointer is null, if yours FFI need to panic check out the next links.

As a good resume see [comment in gtk-rs issue #78](https://github.com/gtk-rs/gtk-rs/issues/78#issuecomment-753841968):
> Currently any unwinding across extern "C" functions is UB, even if all those functions happens
> to be implemented in Rust. That's part of what that WG is working on solving.
> For example this adds support for an extern "C-unwind" ABI that explicitly allows unwinding (and AFAIU causes
> unwinding through extern "C" to abort as it should).

And the [mentioned pull request #76570 of Rust](https://github.com/rust-lang/rust/pull/76570).

Also see [comment in Rust issue #58794](https://github.com/rust-lang/rust/issues/58794#issuecomment-468109183)
 and [Rust issue #58760](https://github.com/rust-lang/rust/issues/58760):
> The default was changed to abort-by-default in extern functions in this PR.
> This is tracking the stabilization of the #[unwind(allowed)] (and #[unwind(abort)]) attributes.

Also [Rust pull request #55982](https://github.com/rust-lang/rust/pull/55982):
> This PR changes the behavior of generated code to be sound-by-default. If an extern fn is unwound (panicked through) then it immediately aborts the program. Put another way, no extern fn can unwind.

And [Rust issue #52652](https://github.com/rust-lang/rust/issues/52652):
> This UB is not mentioned in any later release notes.
