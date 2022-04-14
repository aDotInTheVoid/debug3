#![doc = include_str!("../README.md")]

// From rust-lang/rust/library
pub mod builders;
mod formatter;

mod std_impls;
mod write;

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

use algorithm::Printer;
pub use debug3_derive::Debug;

pub struct Formatter<'a> {
    /*
    Transition Notes
    ----------------

    The Old way (std) directly wrote to `buf`

    The new Way (prettyplease) will use `p`

    When the transition is complete, `buf` will be removed, as
    `Printer's` fields will be inlined into `Formatter`
    */
    buf: &'a mut dyn Write,
    p: Printer,
}

pub trait Debug {
    fn fmt(&self, f: &mut Formatter<'_>);
}

pub trait Write {
    /// Writes a string slice into this writer, returning whether the write
    /// succeeded.
    ///
    /// This method can only succeed if the entire string slice was successfully
    /// written, and this method will not return until all data has been
    /// written or an error occurs.
    ///
    /// # Errors
    ///
    /// This function will return an instance of [`Error`] on error.
    ///
    /// # Examples
    ///
    /// ```
    /// use debug3::Write;
    ///
    /// fn writer<W: Write>(f: &mut W, s: &str) {
    ///     f.write_str(s);
    /// }
    ///
    /// let mut buf = String::new();
    /// writer(&mut buf, "hola");
    /// assert_eq!(&buf, "hola");
    /// ```
    fn write_str(&mut self, s: &str);

    /// Writes a [`char`] into this writer, returning whether the write succeeded.
    ///
    /// A single [`char`] may be encoded as more than one byte.
    /// This method can only succeed if the entire byte sequence was successfully
    /// written, and this method will not return until all data has been
    /// written or an error occurs.
    ///
    /// # Examples
    ///
    /// ```
    /// use debug3::Write;
    ///
    /// fn writer<W: Write>(f: &mut W, c: char) {
    ///     f.write_char(c)
    /// }
    ///
    /// let mut buf = String::new();
    /// writer(&mut buf, 'a');
    /// writer(&mut buf, 'b');
    /// assert_eq!(&buf, "ab");
    /// ```
    fn write_char(&mut self, c: char) {
        self.write_str(c.encode_utf8(&mut [0; 4]));
    }
}

pub fn pprint<T: Debug>(x: T) -> String {
    let mut buf = String::new();
    let mut f = Formatter::new(&mut buf);
    x.fmt(&mut f);
    buf
}

pub fn pprint_new<T: Debug>(x: T) -> String {
    struct Canary {}
    impl Write for Canary {
        fn write_str(&mut self, _s: &str) {
            panic!("CANARY WRITE");
        }
    }
    let mut canary = Canary {};
    let mut f = Formatter::new(&mut canary);
    x.fmt(&mut f);
    let out = f.p.eof();
    return out;
}
