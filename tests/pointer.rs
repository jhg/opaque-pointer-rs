use opaque_pointer;

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
fn opaque_pointer_with_free() {
    let pointer = opaque_pointer::raw(TestIt::new(2));
    // Context to drop object variable
    {
        let object = unsafe { opaque_pointer::mut_object(pointer) };
        object.add(3);
    }
    // Context to drop object variable
    {
        let object = unsafe { opaque_pointer::object(pointer) };
        assert_eq!(object.get(), 5);
    }
    unsafe { opaque_pointer::free(pointer) };
}

#[test]
fn opaque_pointer_with_own_back() {
    let pointer = opaque_pointer::raw(TestIt::new(2));
    // Context to drop object variable
    {
        let object = unsafe { opaque_pointer::mut_object(pointer) };
        object.add(3);
    }
    // Context to drop object variable
    {
        let object = unsafe { opaque_pointer::object(pointer) };
        assert_eq!(object.get(), 5);
    }
    let test_it = unsafe { opaque_pointer::own_back(pointer) };
    assert_eq!(test_it.get(), 5);
}
