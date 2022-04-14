use crate::{Debug, Formatter, Write};

use super::pad::PadAdapter;

/// A struct to help with [`Debug`](Debug) implementations.
///
/// This is useful when you wish to output a formatted tuple as a part of your
/// [`Debug::fmt`] implementation.
///
/// This can be constructed by the [`Formatter::debug_tuple`] method.
///
/// # Examples
///
/// ```
/// use debug3::{Debug, Formatter};
///
/// struct Foo(i32, String);
///
/// impl Debug for Foo {
///     fn fmt(&self, fmt: &mut Formatter) {
///         fmt.debug_tuple("Foo")
///            .field(&self.0)
///            .field(&self.1)
///            .finish()
///     }
/// }
///
/// assert_eq!(
///     debug3::pprint(Foo(10, "Hello World".to_string())),
///     "Foo(10, \"Hello World\")",
/// );
/// ```
#[must_use = "must eventually call `finish()` on Debug builders"]
#[allow(missing_debug_implementations)]
pub struct DebugTuple<'a, 'b: 'a> {
    pub(crate) fmt: &'a mut Formatter<'b>,
    pub(crate) fields: usize,
    pub(crate) empty_name: bool,
}

pub(crate) fn new<'a, 'b>(fmt: &'a mut Formatter<'b>, name: &str) -> DebugTuple<'a, 'b> {
    fmt.write_str(name);
    DebugTuple {
        fmt,
        fields: 0,
        empty_name: name.is_empty(),
    }
}

impl<'a, 'b: 'a> DebugTuple<'a, 'b> {
    /// Adds a new field to the generated tuple struct output.
    ///
    /// # Examples
    ///
    /// ```
    /// use debug3::{Debug, Formatter};
    ///
    /// struct Foo(i32, String);
    ///
    /// impl Debug for Foo {
    ///     fn fmt(&self, fmt: &mut Formatter<'_>) {
    ///         fmt.debug_tuple("Foo")
    ///            .field(&self.0) // We add the first field.
    ///            .field(&self.1) // We add the second field.
    ///            .finish() // We're good to go!
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     debug3::pprint(Foo(10, "Hello World".to_string())),
    ///     "Foo(10, \"Hello World\")",
    /// );
    /// ```
    pub fn field(&mut self, value: &dyn Debug) -> &mut Self {
        if self.is_pretty() {
            if self.fields == 0 {
                self.fmt.write_str("(\n");
            }
            let mut slot = None;
            let mut state = Default::default();
            let mut writer = PadAdapter::wrap(self.fmt, &mut slot, &mut state);
            value.fmt(&mut writer);
            writer.write_str(",\n")
        } else {
            let prefix = if self.fields == 0 { "(" } else { ", " };
            self.fmt.write_str(prefix);
            value.fmt(self.fmt)
        }

        self.fields += 1;
        self
    }

    /// Finishes output and returns any error encountered.
    ///
    /// # Examples
    ///
    /// ```
    /// use debug3::{Debug, Formatter};
    ///
    /// struct Foo(i32, String);
    ///
    /// impl Debug for Foo {
    ///     fn fmt(&self, fmt: &mut Formatter<'_>) {
    ///         fmt.debug_tuple("Foo")
    ///            .field(&self.0)
    ///            .field(&self.1)
    ///            .finish() // You need to call it to "finish" the
    ///                      // tuple formatting.
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     debug3::pprint(Foo(10, "Hello World".to_string())),
    ///     "Foo(10, \"Hello World\")",
    /// );
    /// ```
    pub fn finish(&mut self) {
        if self.fields > 0 {
            if self.fields == 1 && self.empty_name && !self.is_pretty() {
                self.fmt.write_str(",");
            }
            self.fmt.write_str(")")
        }
    }

    pub(crate) fn is_pretty(&self) -> bool {
        self.fmt.alternate()
    }
}
