#![doc = include_str!("../README.md")]

// From rust-lang/rust/library
pub mod builders;
mod formatter;
mod std_impls;

// From dtolnay/prettyplease/
mod algorithm;
mod convenience;
mod ring;

// Target line width.
const MARGIN: isize = 89;

// Number of spaces increment at each level of block indentation.
const INDENT: isize = 4;

// Every line is allowed at least this much space, even if highly indented.
const MIN_SPACE: isize = 60;

pub use algorithm::Formatter;
pub use debug3_derive::Debug;

pub trait Debug {
    fn fmt(&self, f: &mut Formatter);
}

pub fn pprint<T: Debug>(x: T) -> String {
    let mut f = Formatter::new();
    x.fmt(&mut f);
    f.eof()
}
