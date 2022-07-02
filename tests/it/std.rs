use std::{
    cell::RefCell,
    sync::{Arc, Mutex},
};

use debug3::{pprint, Debug};

#[test]
fn refcell() {
    let refcell = RefCell::new(5);
    assert_eq!(pprint(&refcell), "RefCell { value: 5 }");
    let borrow = refcell.borrow_mut();
    assert_eq!(pprint(&refcell), "RefCell { value: <borrowed> }");
    drop(borrow);
    assert_eq!(pprint(&refcell), "RefCell { value: 5 }");
}

#[test]
fn mutex() {
    let x = Arc::new(Mutex::new(5));
    assert_eq!(
        pprint(&x),
        "Mutex {\n    data: 5,\n    poisoned: false,\n    ..\n}"
    );
    let _g = x.lock().unwrap();
    assert_eq!(
        pprint(&x),
        "Mutex {\n    data: <locked>,\n    poisoned: false,\n    ..\n}"
    );
    drop(_g);
    assert_eq!(
        pprint(&x),
        "Mutex {\n    data: 5,\n    poisoned: false,\n    ..\n}"
    );

    let tx = Arc::clone(&x);
    std::thread::spawn(move || {
        let _lock = tx.lock().unwrap();
        panic!(); // the mutex gets poisoned
    })
    .join()
    .unwrap_err();

    assert_eq!(
        pprint(&x),
        "Mutex {\n    data: 5,\n    poisoned: true,\n    ..\n}"
    );
}

#[derive(Debug)]
struct Ref<'a, T: ?Sized> {
    ptr: &'a T,
}

#[test]
fn unsized_ref() {
    let x: Ref<'static, str> = Ref { ptr: "Heya" };
    assert_eq!(pprint(x), "Ref { ptr: \"Heya\" }");

    let x: Ref<'static, [u8]> = Ref { ptr: &[1, 2, 3] };
    assert_eq!(pprint(x), "Ref { ptr: [1, 2, 3] }");

    let x: Ref<'static, dyn Debug> = Ref {
        ptr: &5 as &dyn Debug,
    };
    assert_eq!(pprint(x), "Ref { ptr: 5 }");
}
