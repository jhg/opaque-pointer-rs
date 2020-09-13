# opaque-pointer-rs
Generic functions to work with opaque pointers when use FFI to expose Rust structs

## Basic usage
With this crate you can manage raw pointers easily to expose structs that will be
 use as opaque pointers from C or C++ calling to Rust functions to use it. This
 can be used with [cbindgen](https://crates.io/crates/cbindgen) crate with option `parse.parse_deps = true` for it will
 generate opaque C/C++ structs to use pointers in the arguments. You can find
 more information in [The Rust FFI Omnibus objects section](http://jakegoulding.com/rust-ffi-omnibus/objects/) of Jake Goulding.

Example:
```rust
use opaque_pointer;

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
    opaque_pointer::raw(TestIt::new(testit))
}

/// TestIt add method.
#[no_mangle]
pub extern fn testit_add(testit: *mut TestIt, value: u8) {
    let testit = opaque_pointer::mut_object(testit);
    testit.add(value);
}

/// TestIt get method.
#[no_mangle]
pub extern fn testit_get(testit: *const TestIt) -> u8 {
    let testit = opaque_pointer::object(testit);
    testit.get()
}

/// TestIt free.
#[no_mangle]
pub extern fn testit_free(testit: *const TestIt) {
    opaque_pointer::free(testit)
}
```
