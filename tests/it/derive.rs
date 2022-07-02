#![allow(dead_code)]

use debug3::{pprint, Debug, Formatter};

#[derive(Debug)]
struct Unit;

#[derive(Debug)]
struct Tuple(isize, usize);

#[derive(Debug)]
struct Struct {
    x: isize,
    y: usize,
}

#[derive(Debug)]
enum Enum {
    Nullary,
    Variant(isize, usize),
    StructVariant { x: isize, y: usize },
}

#[derive(Debug)]
struct Pointers(*const dyn Send, *mut dyn Sync);

macro_rules! t {
    ($x:expr, $expected:expr) => {
        assert_eq!(debug3::pprint($x), $expected.to_string())
    };
}

#[test]
fn derive1() {
    t!(Unit, "Unit");
    t!(Tuple(1, 2), "Tuple(1, 2)");
    t!(Struct { x: 1, y: 2 }, "Struct { x: 1, y: 2 }");
    t!(Enum::Nullary, "Nullary");
    t!(Enum::Variant(1, 2), "Variant(1, 2)");
    t!(
        Enum::StructVariant { x: 1, y: 2 },
        "StructVariant { x: 1, y: 2 }"
    );
}

// https://github.com/rust-lang/rust/blob/9f8f0a6e9484fe25517c082a5cbe1e9edb17c8a8/src/test/ui/deriving/deriving-show.rs
// https://github.com/rust-lang/rust/blob/9f8f0a6e9484fe25517c082a5cbe1e9edb17c8a8/src/test/ui/deriving/deriving-show-2.rs

#[derive(Debug)]
enum A {}

#[derive(Debug)]
enum B {
    B1,
    B2,
    B3,
}
#[derive(Debug)]
enum C {
    C1(isize),
    C2(B),
    C3(String),
}
#[derive(Debug)]
enum D {
    D1 { a: isize },
}
#[derive(Debug)]
struct E;
#[derive(Debug)]
struct F(isize);
#[derive(Debug)]
struct G(isize, isize);
#[derive(Debug)]
struct H {
    a: isize,
}
#[derive(Debug)]
struct I {
    a: isize,
    b: isize,
}
#[derive(Debug)]
struct J(Custom);

struct Custom;
impl Debug for Custom {
    fn fmt(&self, f: &mut Formatter) {
        // TODO: Do we want something like `f.write_str("Custom")`
        // More generaly, How do we want custom `Debug` impls that dont look like
        // Derives to work?

        // Eg Rustc's Span and DefID
        "yay".fmt(f);
    }
}

trait ToDebug {
    fn to_show(&self) -> String;
}

impl<T: Debug> ToDebug for T {
    fn to_show(&self) -> String {
        pprint(self)
    }
}

#[test]
fn derive2() {
    assert_eq!(B::B1.to_show(), "B1".to_string());
    assert_eq!(B::B2.to_show(), "B2".to_string());
    assert_eq!(C::C1(3).to_show(), "C1(3)".to_string());
    assert_eq!(C::C2(B::B2).to_show(), "C2(B2)".to_string());
    assert_eq!(D::D1 { a: 2 }.to_show(), "D1 { a: 2 }".to_string());
    assert_eq!(E.to_show(), "E".to_string());
    assert_eq!(F(3).to_show(), "F(3)".to_string());
    assert_eq!(G(3, 4).to_show(), "G(3, 4)".to_string());
    assert_eq!(I { a: 2, b: 4 }.to_show(), "I { a: 2, b: 4 }".to_string());
    assert_eq!(J(Custom).to_show(), "J(\"yay\")".to_string());
}
