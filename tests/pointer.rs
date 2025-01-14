use opaque_pointer;
use opaque_pointer::error::PointerError;


#[derive(Debug)]
struct TestIt {
    value: u8,
}

impl TestIt {
    pub fn new(value: u8) -> Self {
        Self { value }
    }
    pub fn add(&mut self, value: u8) {
        self.value += value;
    }
    pub fn get(&self) -> u8 {
        self.value
    }
}


#[test]
fn own_back() {
    let pointer = opaque_pointer::raw(TestIt::new(2)).unwrap();
    let test_it = unsafe { opaque_pointer::own_back(pointer).unwrap() };
    assert_eq!(test_it.get(), 2);
}

#[cfg(all(feature = "std", feature = "lender"))]
#[test]
fn own_back_invalid_pointer() {
    let pointer = Box::into_raw(Box::new(TestIt::new(2)));
    let error = unsafe { opaque_pointer::own_back(pointer).unwrap_err() };
    assert_eq!(
        error,
        PointerError::Invalid
    );
}

#[cfg(all(feature = "std", feature = "lender"))]
#[test]
fn own_back_invalid_type() {
    let pointer = opaque_pointer::raw(TestIt::new(2)).unwrap() as *mut u8;
    let error = unsafe { opaque_pointer::own_back(pointer).unwrap_err() };
    assert_eq!(
        error,
        PointerError::InvalidType
    );

    unsafe { opaque_pointer::own_back(pointer as *mut TestIt).unwrap() };
}


#[test]
fn immutable_reference() {
    let pointer = opaque_pointer::raw(TestIt::new(2)).unwrap();
    let object = unsafe { opaque_pointer::object(pointer).unwrap() };
    assert_eq!(object.get(), 2);

    unsafe { opaque_pointer::own_back(pointer).unwrap() };
}


#[test]
fn mutable_reference() {
    let pointer = opaque_pointer::raw(TestIt::new(2)).unwrap();
    let object = unsafe { opaque_pointer::mut_object(pointer).unwrap() };
    object.add(3);
    assert_eq!(object.get(), 5);

    unsafe { opaque_pointer::own_back(pointer).unwrap() };
}
