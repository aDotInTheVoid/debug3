#![deny(dead_code)] // Becuase ui-test requires an error, not warning

// TODO: Add #[derive(debug3::Debug)], and have the same error.
// https://twitter.com/adotinthevoid/status/1647682561366188034
pub struct Foo {
    x: i32,
    y: i32,
}

pub fn foo_x(foo: &Foo) -> i32 {
    foo.x
}

fn main() {}
