# Opaque Pointer for seamless Rust FFI

Harnessing generics with opaque pointers for seamless Rust FFI interoperability with C and C++.

[![Crates.io](https://img.shields.io/crates/v/opaque-pointer)](https://crates.io/crates/opaque-pointer)
[![Crates.io](https://img.shields.io/crates/l/opaque-pointer)](https://unlicense.org/)
[![Crates.io](https://img.shields.io/crates/d/opaque-pointer)](https://crates.io/crates/opaque-pointer)

## Overview

Simplify raw pointer management to expose Rust structs as opaque pointers, seamlessly enabling interaction between Rust and C/C++ functions.
This crate facilitates creating opaque C/C++ structs for use with arguments, generated by [cbindgen](https://crates.io/crates/cbindgen) with `parse.parse_deps = true`.

For comprehensive insights into Rust's interoperability with other languages, explore [The Rust FFI Omnibus objects section](http://jakegoulding.com/rust-ffi-omnibus/objects/) by [Jake Goulding](https://github.com/shepmaster).

## Lender feature

By activating the `lender` feature, functions like `own_back<T>()` validate the pointer, ensuring its validity.
It means, be returned by `raw<T>()` and `own_back<T>()` with same type.

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
    return opaque_pointer::raw(Counter::new())
            .expect("Error trying to lend a pointer");
}

/// Drop (free memory of) Rust's Counter object as usually.
#[no_mangle]
pub extern fn counter_free(counter: *mut Counter) {
    unsafe { opaque_pointer::own_back(counter) };
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
