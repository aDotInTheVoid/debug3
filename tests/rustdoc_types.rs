#![cfg(rustdoc_types)]

use debug3::pprint;
use rustdoc_types::Crate;

#[test]
#[ignore = "HashMaps change order, we shoud (???) fix this"]
fn rustdoc_types() {
    let c: Crate = serde_json::from_str(include_str!("./foo.json")).unwrap();
    let x = pprint(c);
    // std::fs::write("foo.txt", &x).unwrap();
    assert_eq!(x, include_str!("./foo.txt"));
}
