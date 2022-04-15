use crate::{Debug, Formatter, Write, INDENT};

use super::DebugInner;

/// A struct to help with [`Debug`](Debug) implementations.
///
/// This is useful when you wish to output a formatted list of items as a part
/// of your [`Debug::fmt`] implementation.
///
/// This can be constructed by the [`Formatter::debug_list`] method.
///
/// # Examples
///
/// ```
/// use debug3::{Debug, Formatter};
///
/// struct Foo(Vec<i32>);
///
/// impl Debug for Foo {
///     fn fmt(&self, fmt: &mut Formatter) {
///         fmt.debug_list().entries(self.0.iter()).finish()
///     }
/// }
///
/// assert_eq!(
///     debug3::pprint(Foo(vec![10, 11])),
///     "[10, 11]",
/// );
/// ```
#[must_use = "must eventually call `finish()` on Debug builders"]
#[allow(missing_debug_implementations)]
pub struct DebugList<'a> {
    inner: DebugInner<'a>,
}

pub(crate) fn new<'a>(fmt: &'a mut Formatter) -> DebugList<'a> {
    fmt.p.word("[");
    fmt.p.cbox(INDENT);
    fmt.p.zerobreak();

    DebugList {
        inner: DebugInner {
            fmt,
            has_fields: false,
        },
    }
}

impl<'a> DebugList<'a> {
    /// Adds a new entry to the list output.
    ///
    /// # Examples
    ///
    /// ```
    /// use debug3::{Debug, Formatter};
    ///
    /// struct Foo(Vec<i32>, Vec<u32>);
    ///
    /// impl Debug for Foo {
    ///     fn fmt(&self, fmt: &mut Formatter) {
    ///         fmt.debug_list()
    ///            .entry(&self.0) // We add the first "entry".
    ///            .entry(&self.1) // We add the second "entry".
    ///            .finish()
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     debug3::pprint(Foo(vec![10, 11], vec![12, 13])),
    ///     "[[10, 11], [12, 13]]",
    /// );
    /// ```
    pub fn entry(&mut self, entry: &dyn Debug) -> &mut Self {
        self.inner.entry(entry);
        self
    }

    /// Adds the contents of an iterator of entries to the list output.
    ///
    /// # Examples
    ///
    /// ```
    /// use debug3::{Debug, Formatter};
    ///
    /// struct Foo(Vec<i32>, Vec<u32>);
    ///
    /// impl Debug for Foo {
    ///     fn fmt(&self, fmt: &mut Formatter) {
    ///         fmt.debug_list()
    ///            .entries(self.0.iter())
    ///            .entries(self.1.iter())
    ///            .finish()
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     debug3::pprint(Foo(vec![10, 11], vec![12, 13])),
    ///     "[10, 11, 12, 13]",
    /// );
    /// ```
    pub fn entries<D, I>(&mut self, entries: I) -> &mut Self
    where
        D: Debug,
        I: IntoIterator<Item = D>,
    {
        for entry in entries {
            self.entry(&entry);
        }
        self
    }

    /// Finishes output and returns any error encountered.
    ///
    /// # Examples
    ///
    /// ```
    /// use debug3::{Debug, Formatter};
    ///
    /// struct Foo(Vec<i32>);
    ///
    /// impl Debug for Foo {
    ///     fn fmt(&self, fmt: &mut Formatter) {
    ///         fmt.debug_list()
    ///            .entries(self.0.iter())
    ///            .finish() // Ends the struct formatting.
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     debug3::pprint(Foo(vec![10, 11])),
    ///     "[10, 11]",
    /// );
    /// ```
    pub fn finish(&mut self) {
        if self.inner.has_fields {
            self.inner.fmt.p.trailing_comma(true);
        }

        self.inner.fmt.p.offset(-INDENT);
        self.inner.fmt.p.end();
        self.inner.fmt.p.word("]");
    }
}
