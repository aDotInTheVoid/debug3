pub mod builders;
mod formatter;
mod std_impls;
mod write;

pub use debug3_derive::Debug;

pub struct Formatter<'a> {
    buf: &'a mut dyn Write,
}

pub trait Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}

pub type Result = std::result::Result<(), Error>;

#[derive(Clone, Copy, std::fmt::Debug)]
pub struct Error {}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
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
    /// use std::fmt::{Error, Write};
    ///
    /// fn writer<W: Write>(f: &mut W, s: &str) -> Result<(), Error> {
    ///     f.write_str(s)
    /// }
    ///
    /// let mut buf = String::new();
    /// writer(&mut buf, "hola").unwrap();
    /// assert_eq!(&buf, "hola");
    /// ```
    fn write_str(&mut self, s: &str) -> Result;

    /// Writes a [`char`] into this writer, returning whether the write succeeded.
    ///
    /// A single [`char`] may be encoded as more than one byte.
    /// This method can only succeed if the entire byte sequence was successfully
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
    /// use std::fmt::{Error, Write};
    ///
    /// fn writer<W: Write>(f: &mut W, c: char) -> Result<(), Error> {
    ///     f.write_char(c)
    /// }
    ///
    /// let mut buf = String::new();
    /// writer(&mut buf, 'a').unwrap();
    /// writer(&mut buf, 'b').unwrap();
    /// assert_eq!(&buf, "ab");
    /// ```
    fn write_char(&mut self, c: char) -> Result {
        self.write_str(c.encode_utf8(&mut [0; 4]))
    }
}

pub fn pprint<T: Debug>(x: T) -> String {
    let mut buf = String::new();
    let mut f = Formatter::new(&mut buf);
    x.fmt(&mut f).unwrap();
    buf
}
