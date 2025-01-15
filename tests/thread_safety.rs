use std::thread;
use std::time::Duration;
use opaque_pointer;

#[cfg(all(feature = "std", feature = "lender"))]
#[test]
fn own_back() {
    let for_test = 0;
    let pointer = opaque_pointer::raw(for_test).unwrap();

    let mut threads = Vec::new();
    for _ in 0..1000 {
        let pointer = pointer as usize;
        threads.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(5));
            unsafe { opaque_pointer::own_back(pointer as *mut i32).is_ok() }
        }));
    }

    // If all works well, only one thread will be able to own_back the pointer.
    let mut counter = 0;
    for thread in threads {
        if let Ok(true) = thread.join() {
            counter += 1;
        }
    }

    assert_eq!(1, counter);
}
