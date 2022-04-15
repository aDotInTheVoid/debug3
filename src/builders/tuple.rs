use crate::{Debug, Formatter, INDENT};

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
pub struct DebugTuple<'a> {
    pub(crate) fmt: &'a mut Formatter,
    pub(crate) fields: usize,
    pub(crate) empty_name: bool,
}

pub(crate) fn new<'a>(fmt: &'a mut Formatter, name: &str) -> DebugTuple<'a> {
    // fmt.write_str(name);
    fmt.p.word_s(name);

    DebugTuple {
        fmt,
        fields: 0,
        empty_name: name.is_empty(),
    }
}

impl<'a> DebugTuple<'a> {
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
    ///     fn fmt(&self, fmt: &mut Formatter) {
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
        // if self.is_pretty() {
        //     if self.fields == 0 {
        //         self.fmt.write_str("(\n");
        //     }
        //     let mut slot = None;
        //     let mut state = Default::default();
        //     let mut writer = PadAdapter::wrap(self.fmt, &mut slot, &mut state);
        //     value.fmt(&mut writer);
        //     writer.write_str(",\n")
        // } else {
        //     let prefix = if self.fields == 0 { "(" } else { ", " };
        //     self.fmt.write_str(prefix);
        //     value.fmt(self.fmt)
        // }

        if self.fields == 0 {
            self.fmt.p.word("(");
            self.fmt.p.cbox(INDENT);
            self.fmt.p.zerobreak();
        } else {
            self.fmt.p.trailing_comma(false);
        }

        value.fmt(self.fmt);

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
    ///     fn fmt(&self, fmt: &mut Formatter) {
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
        // if self.fields > 0 {
        //     if self.fields == 1 && self.empty_name && !self.is_pretty() {
        //         self.fmt.write_str(",");
        //     }
        //     self.fmt.write_str(")")
        // }

        if self.fields > 0 {
            // Handle Closing Comma for tuple of 1,
            if self.fields == 1 && self.empty_name {
                self.fmt.p.word(",");
                self.fmt.p.zerobreak();
            } else {
                self.fmt.p.trailing_comma(true);
            }
            self.fmt.p.offset(-INDENT);
            self.fmt.p.end();
            self.fmt.p.word(")");
        } else if self.empty_name {
            self.fmt.p.word("()");
        }
    }
}
