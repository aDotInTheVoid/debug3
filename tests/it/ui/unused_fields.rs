#![deny(dead_code)] // Becuase ui-test requires an error, not warning

#[derive(debug3::Debug)]
pub struct Foo {
    x: i32,
    y: i32,
}

pub fn foo_x(foo: &Foo) -> i32 {
    foo.x
}

fn main() {}
