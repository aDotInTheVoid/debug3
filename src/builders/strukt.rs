use crate::{Debug, Formatter, Write, INDENT};

/// A struct to help with [`Debug`](Debug) implementations.
///
/// This is useful when you wish to output a formatted struct as a part of your
/// [`Debug::fmt`] implementation.
///
/// This can be constructed by the [`Formatter::debug_struct`] method.
///
/// # Examples
///
/// ```
/// use debug3::{Debug, Formatter};
///
/// struct Foo {
///     bar: i32,
///     baz: String,
/// }
///
/// impl Debug for Foo {
///     fn fmt(&self, fmt: &mut Formatter) {
///         fmt.debug_struct("Foo")
///            .field("bar", &self.bar)
///            .field("baz", &self.baz)
///            .finish()
///     }
/// }
///
/// assert_eq!(
///     debug3::pprint(Foo { bar: 10, baz: "Hello World".to_string() }),
///     "Foo { bar: 10, baz: \"Hello World\" }",
/// );
/// ```
#[must_use = "must eventually call `finish()` on Debug builders"]
#[allow(missing_debug_implementations)]
pub struct DebugStruct<'a> {
    pub(crate) fmt: &'a mut Formatter,
    pub(crate) has_fields: bool,
}

pub(crate) fn new<'a>(fmt: &'a mut Formatter, name: &str) -> DebugStruct<'a> {
    fmt.p.cbox(INDENT);
    fmt.p.ibox(-INDENT);
    fmt.p.word_s(name);
    fmt.p.end();

    DebugStruct {
        fmt,
        has_fields: false,
    }
}

impl<'a> DebugStruct<'a> {
    /// Adds a new field to the generated struct output.
    ///
    /// # Examples
    ///
    /// ```
    /// use debug3::{Debug, Formatter};
    ///
    /// struct Bar {
    ///     bar: i32,
    ///     another: String,
    /// }
    ///
    /// impl Debug for Bar {
    ///     fn fmt(&self, fmt: &mut Formatter) {
    ///         fmt.debug_struct("Bar")
    ///            .field("bar", &self.bar) // We add `bar` field.
    ///            .field("another", &self.another) // We add `another` field.
    ///            // We even add a field which doesn't exist (because why not?).
    ///            .field("not_existing_field", &1)
    ///            .finish() // We're good to go!
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     debug3::pprint(Bar { bar: 10, another: "Hello World".to_string() }),
    ///     "Bar { bar: 10, another: \"Hello World\", not_existing_field: 1 }",
    /// );
    /// ```
    pub fn field(&mut self, name: &str, value: &dyn Debug) -> &mut Self {
        // if self.is_pretty() {
        //     if !self.has_fields {
        //         self.fmt.write_str(" {\n");
        //     }
        //     let mut slot = None;
        //     let mut state = Default::default();
        //     let mut writer = PadAdapter::wrap(self.fmt, &mut slot, &mut state);
        //     writer.write_str(name);
        //     writer.write_str(": ");
        //     value.fmt(&mut writer);
        //     writer.write_str(",\n")
        // } else {
        //     let prefix = if self.has_fields { ", " } else { " { " };
        //     self.fmt.write_str(prefix);
        //     self.fmt.write_str(name);
        //     self.fmt.write_str(": ");
        //     value.fmt(self.fmt)
        // }

        if self.has_fields {
            self.fmt.p.trailing_comma_or_space(false);
        } else {
            self.fmt.p.word(" {");
            self.fmt.p.space_if_nonempty();
        }

        self.fmt.p.word_s(name);
        self.fmt.p.word(": ");
        self.fmt.p.ibox(0);
        value.fmt(self.fmt);
        self.fmt.p.end();

        self.has_fields = true;
        self
    }

    /// Marks the struct as non-exhaustive, indicating to the reader that there are some other
    /// fields that are not shown in the debug representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use debug3::{Debug, Formatter};
    ///
    /// struct Bar {
    ///     bar: i32,
    ///     hidden: f32,
    /// }
    ///
    /// impl Debug for Bar {
    ///     fn fmt(&self, fmt: &mut Formatter) {
    ///         fmt.debug_struct("Bar")
    ///            .field("bar", &self.bar)
    ///            .finish_non_exhaustive() // Show that some other field(s) exist.
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     debug3::pprint(Bar { bar: 10, hidden: 1.0 }),
    ///     "Bar { bar: 10, .. }",
    /// );
    /// ```
    pub fn finish_non_exhaustive(&mut self) {
        // if self.has_fields {
        //     if self.is_pretty() {
        //         let mut slot = None;
        //         let mut state = Default::default();
        //         let mut writer = PadAdapter::wrap(self.fmt, &mut slot, &mut state);
        //         writer.write_str("..\n");
        //         self.fmt.write_str("}")
        //     } else {
        //         self.fmt.write_str(", .. }")
        //     }
        // } else {
        //     self.fmt.write_str(" { .. }")
        // }

        if self.has_fields {
            self.fmt.p.trailing_comma_or_space(false);
        } else {
            self.fmt.p.word(" {");
            self.fmt.p.space_if_nonempty();
        }

        self.fmt.p.word("..");
        self.fmt.p.space();
        self.fmt.p.offset(-INDENT);
        self.fmt.p.end_with_max_width(34); // TODO: Why 34
        self.fmt.p.word("}");
    }

    /// Finishes output and returns any error encountered.
    ///
    /// # Examples
    ///
    /// ```
    /// use debug3::{Debug, Formatter};
    ///
    /// struct Bar {
    ///     bar: i32,
    ///     baz: String,
    /// }
    ///
    /// impl Debug for Bar {
    ///     fn fmt(&self, fmt: &mut Formatter) {
    ///         fmt.debug_struct("Bar")
    ///            .field("bar", &self.bar)
    ///            .field("baz", &self.baz)
    ///            .finish() // You need to call it to "finish" the
    ///                      // struct formatting.
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     debug3::pprint(Bar { bar: 10, baz: "Hello World".to_string() }),
    ///     "Bar { bar: 10, baz: \"Hello World\" }",
    /// );
    /// ```
    pub fn finish(&mut self) {
        // if self.has_fields {
        //     if self.is_pretty() {
        //         self.fmt.write_str("}")
        //     } else {
        //         self.fmt.write_str(" }")
        //     }
        // }

        if self.has_fields {
            self.fmt.p.trailing_comma_or_space(true);
        } else {
            self.fmt.p.zerobreak();
        }
        self.fmt.p.offset(-INDENT);
        // TODO: Why 34
        // https://github.com/dtolnay/prettyplease/commit/a98f613b661bba3eb4f54cf4bba5c74c23d395e8
        self.fmt.p.end_with_max_width(34);

        if self.has_fields {
            self.fmt.p.word("}");
        }
    }
}
