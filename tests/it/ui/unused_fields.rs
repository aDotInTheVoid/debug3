#![deny(dead_code)] // Becuase ui-test requires an error, not warning

// Don't change line nos
pub struct Foo {
    x: i32,
    y: i32,
}

pub fn foo_x(foo: &Foo) -> i32 {
    foo.x
}

fn main() {}
