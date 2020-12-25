# opaque-pointer-rs

Generic functions to work with opaque pointers when use FFI to expose Rust structs

[![Crates.io](https://img.shields.io/crates/v/opaque-pointer)](https://crates.io/crates/opaque-pointer)
[![Crates.io](https://img.shields.io/crates/l/opaque-pointer)](https://unlicense.org/)
[![Crates.io](https://img.shields.io/crates/d/opaque-pointer)](.)

## Basic usage

With this crate you can manage raw pointers easily to expose structs that will be
 use as opaque pointers from C or C++ calling to Rust functions to use it. This
 can be used with [cbindgen](https://crates.io/crates/cbindgen) crate with option `parse.parse_deps = true`
 for it will generate opaque C/C++ structs to use pointers in the arguments. You can find
 more information in [The Rust FFI Omnibus objects section](http://jakegoulding.com/rust-ffi-omnibus/objects/) of Jake Goulding.

## Examples

```rust
struct TestIt {
    value: u8,
}

impl TestIt {
    pub fn new(value: u8) -> Self {
        Self {
            value,
        }
    }
    pub fn add(&mut self, value: u8) {
        self.value += value;
    }
    pub fn get(&self) -> u8 {
        self.value
    }
}

/// TestIt new method.
#[no_mangle]
pub extern fn testit_new(value: u8) -> *mut TestIt {
    opaque_pointer::raw(TestIt::new(value))
}

/// TestIt add method.
#[no_mangle]
pub extern fn testit_add(testit: *mut TestIt, value: u8) {
    let testit = unsafe { opaque_pointer::mut_object(testit) };
    testit.add(value);
}

/// TestIt get method.
#[no_mangle]
pub extern fn testit_get(testit: *const TestIt) -> u8 {
    let testit = unsafe { opaque_pointer::object(testit) };
    testit.get()
}

/// TestIt free.
#[no_mangle]
pub extern fn testit_free(testit: *mut TestIt) {
    unsafe { opaque_pointer::free(testit) }
}
```

## Features

- `std`: activated by default, it is required for functions using std
- `alloc`: alternative to compile without std but some functions will not be available
- `c-types`: it allow to use C types (like pointers to C strings) and requires std feature
- `panic-if-null`: it will check if a pointer is null to panic before to use a null pointer
