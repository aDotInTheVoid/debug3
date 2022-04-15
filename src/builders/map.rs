use crate::{Debug, Formatter, INDENT};

/// A struct to help with [`Debug`](Debug) implementations.
///
/// This is useful when you wish to output a formatted map as a part of your
/// [`Debug::fmt`] implementation.
///
/// This can be constructed by the [`Formatter::debug_map`] method.
///
/// # Examples
///
/// ```
/// use debug3::{Debug, Formatter};
///
/// struct Foo(Vec<(String, i32)>);
///
/// impl Debug for Foo {
///     fn fmt(&self, fmt: &mut Formatter) {
///         fmt.debug_map().entries(self.0.iter().map(|&(ref k, ref v)| (k, v))).finish()
///     }
/// }
///
/// assert_eq!(
///     debug3::pprint(Foo(vec![("A".to_string(), 10), ("B".to_string(), 11)])),
///     "{\"A\": 10, \"B\": 11}",
/// );
/// ```
#[must_use = "must eventually call `finish()` on Debug builders"]
pub struct DebugMap<'a> {
    fmt: &'a mut Formatter,
    has_fields: bool,
    has_key: bool,
}

pub(crate) fn new(fmt: &mut Formatter) -> DebugMap<'_> {
    fmt.word("{");
    fmt.cbox(INDENT);
    fmt.zerobreak();

    DebugMap {
        fmt,
        has_fields: false,
        has_key: false,
    }
}

impl<'a> DebugMap<'a> {
    /// Adds a new entry to the map output.
    ///
    /// # Examples
    ///
    /// ```
    /// use debug3::{Debug, Formatter};
    ///
    /// struct Foo(Vec<(String, i32)>);
    ///
    /// impl Debug for Foo {
    ///     fn fmt(&self, fmt: &mut Formatter) {
    ///         fmt.debug_map()
    ///            .entry(&"whole", &self.0) // We add the "whole" entry.
    ///            .finish()
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     debug3::pprint(Foo(vec![("A".to_string(), 10), ("B".to_string(), 11)])),
    ///     "{\"whole\": [(\"A\", 10), (\"B\", 11)]}",
    /// );
    /// ```
    pub fn entry(&mut self, key: &dyn Debug, value: &dyn Debug) -> &mut Self {
        self.key(key).value(value)
    }

    /// Adds the key part of a new entry to the map output.
    ///
    /// This method, together with `value`, is an alternative to `entry` that
    /// can be used when the complete entry isn't known upfront. Prefer the `entry`
    /// method when it's possible to use.
    ///
    /// # Panics
    ///
    /// `key` must be called before `value` and each call to `key` must be followed
    /// by a corresponding call to `value`. Otherwise this method will panic.
    ///
    /// # Examples
    ///
    /// ```
    /// use debug3::{Debug, Formatter};
    ///
    /// struct Foo(Vec<(String, i32)>);
    ///
    /// impl Debug for Foo {
    ///     fn fmt(&self, fmt: &mut Formatter) {
    ///         fmt.debug_map()
    ///            .key(&"whole").value(&self.0) // We add the "whole" entry.
    ///            .finish()
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     debug3::pprint(Foo(vec![("A".to_string(), 10), ("B".to_string(), 11)])),
    ///     "{\"whole\": [(\"A\", 10), (\"B\", 11)]}",
    /// );
    /// ```
    pub fn key(&mut self, key: &dyn Debug) -> &mut Self {
        assert!(
            !self.has_key,
            "attempted to begin a new map entry \
                                    without completing the previous one"
        );

        // if self.is_pretty() {
        //     if !self.has_fields {
        //         self.fmt.write_str("\n");
        //     }
        //     let mut slot = None;
        //     self.state = Default::default();
        //     let mut writer = PadAdapter::wrap(self.fmt, &mut slot, &mut self.state);
        //     key.fmt(&mut writer);
        //     writer.write_str(": ");
        // } else {
        //     if self.has_fields {
        //         self.fmt.write_str(", ");
        //     }
        //     key.fmt(self.fmt);
        //     self.fmt.write_str(": ");
        // }

        if self.has_fields {
            self.fmt.trailing_comma_or_space(false);
        }

        self.fmt.ibox(0);
        key.fmt(self.fmt); // TODO: Should this be Boxed?
        self.fmt.end();

        self.fmt.word(": ");

        self.has_key = true;
        self
    }

    /// Adds the value part of a new entry to the map output.
    ///
    /// This method, together with `key`, is an alternative to `entry` that
    /// can be used when the complete entry isn't known upfront. Prefer the `entry`
    /// method when it's possible to use.
    ///
    /// # Panics
    ///
    /// `key` must be called before `value` and each call to `key` must be followed
    /// by a corresponding call to `value`. Otherwise this method will panic.
    ///
    /// # Examples
    ///
    /// ```
    /// use debug3::{Debug, Formatter};
    ///
    /// struct Foo(Vec<(String, i32)>);
    ///
    /// impl Debug for Foo {
    ///     fn fmt(&self, fmt: &mut Formatter) {
    ///         fmt.debug_map()
    ///            .key(&"whole").value(&self.0) // We add the "whole" entry.
    ///            .finish()
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     debug3::pprint(Foo(vec![("A".to_string(), 10), ("B".to_string(), 11)])),
    ///     "{\"whole\": [(\"A\", 10), (\"B\", 11)]}",
    /// );
    /// ```
    pub fn value(&mut self, value: &dyn Debug) -> &mut Self {
        assert!(
            self.has_key,
            "attempted to format a map value before its key"
        );

        // if self.is_pretty() {
        //     let mut slot = None;
        //     let mut writer = PadAdapter::wrap(self.fmt, &mut slot, &mut self.state);
        //     value.fmt(&mut writer);
        //     writer.write_str(",\n");
        // } else {
        //     value.fmt(self.fmt);
        // }

        self.fmt.ibox(0);
        value.fmt(self.fmt);
        self.fmt.end();

        self.has_key = false;
        self.has_fields = true;
        self
    }

    /// Adds the contents of an iterator of entries to the map output.
    ///
    /// # Examples
    ///
    /// ```
    /// use debug3::{Debug, Formatter};
    ///
    /// struct Foo(Vec<(String, i32)>);
    ///
    /// impl Debug for Foo {
    ///     fn fmt(&self, fmt: &mut Formatter) {
    ///         fmt.debug_map()
    ///            // We map our vec so each entries' first field will become
    ///            // the "key".
    ///            .entries(self.0.iter().map(|&(ref k, ref v)| (k, v)))
    ///            .finish()
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     debug3::pprint(Foo(vec![("A".to_string(), 10), ("B".to_string(), 11)])),
    ///     "{\"A\": 10, \"B\": 11}",
    /// );
    /// ```
    pub fn entries<K, V, I>(&mut self, entries: I) -> &mut Self
    where
        K: Debug,
        V: Debug,
        I: IntoIterator<Item = (K, V)>,
    {
        for (k, v) in entries {
            self.entry(&k, &v);
        }
        self
    }

    /// Finishes output and returns any error encountered.
    ///
    /// # Panics
    ///
    /// `key` must be called before `value` and each call to `key` must be followed
    /// by a corresponding call to `value`. Otherwise this method will panic.
    ///
    /// # Examples
    ///
    /// ```
    /// use debug3::{Debug, Formatter};
    ///
    /// struct Foo(Vec<(String, i32)>);
    ///
    /// impl Debug for Foo {
    ///     fn fmt(&self, fmt: &mut Formatter) {
    ///         fmt.debug_map()
    ///            .entries(self.0.iter().map(|&(ref k, ref v)| (k, v)))
    ///            .finish() // Ends the struct formatting.
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     debug3::pprint(Foo(vec![("A".to_string(), 10), ("B".to_string(), 11)])),
    ///     "{\"A\": 10, \"B\": 11}",
    /// );
    /// ```
    pub fn finish(&mut self) {
        assert!(
            !self.has_key,
            "attempted to finish a map with a partial entry"
        );

        if self.has_fields {
            self.fmt.trailing_comma(true);
        }

        self.fmt.offset(-INDENT);
        self.fmt.end();
        self.fmt.word("}");
    }

    // pub(crate) fn is_pretty(&self) -> bool {
    //     self.fmt.alternate()
    // }
}
