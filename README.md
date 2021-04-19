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

## Lender feature

If you activate the feature `lender` the functions like `own_back<T>()` will check if the pointer is valid.
It means, the function result will be a error if the pointer was not returned by `raw<T>()`.

## Examples

Creating FFIs to use a Rust's `struct` methods from C or C++:

```rust
struct Counter { value: u8 }

impl Counter {
    pub fn new() -> Self { Self { value: 0 } }
    pub fn add(&mut self, value: u8) { self.value += value }
    pub fn get(&self) -> u8 { self.value }
}

/// Ownership will NOT control the heap-allocated memory until own it back.
#[no_mangle]
pub extern fn counter_new(value: u8) -> *mut Counter {
    return opaque_pointer::raw(Counter::new());
}

/// Drop (free memory of) Rust's Counter object as usually.
#[no_mangle]
pub extern fn counter_free(counter: *mut Counter) {
    unsafe { opaque_pointer::free(counter) };
}

#[no_mangle]
pub extern fn counter_add(counter: *mut Counter, value: u8) -> bool {
    let counter = unsafe { opaque_pointer::mut_object(counter) };
    if counter.is_err() {
        return false;
    }
    let counter = counter.unwrap();
    counter.add(value);
    // Here will NOT be dropped, the pointer continues been valid.
    return true;
}

#[no_mangle]
pub extern fn counter_get(counter: *const Counter) -> u8  {
    let counter = unsafe { opaque_pointer::object(counter) };
    if counter.is_err() {
        return 0;
    }
    let counter = counter.unwrap();
    return counter.get();
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
