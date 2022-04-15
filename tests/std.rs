use std::{
    cell::RefCell,
    sync::{Arc, Mutex},
};

use debug3::pprint;

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
