use crate::{Debug, Formatter, INDENT};

use super::DebugInner;

/// A struct to help with [`Debug`](Debug) implementations.
///
/// This is useful when you wish to output a formatted set of items as a part
/// of your [`Debug::fmt`] implementation.
///
/// This can be constructed by the [`Formatter::debug_set`] method.
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
///         fmt.debug_set().entries(self.0.iter()).finish()
///     }
/// }
///
/// assert_eq!(debug3::pprint(Foo(vec![10, 11])), "{10, 11}",);
/// ```
#[must_use = "must eventually call `finish()` on Debug builders"]
pub struct DebugSet<'a> {
    inner: DebugInner<'a>,
}

pub(crate) fn new(fmt: &mut Formatter) -> DebugSet {
    fmt.word("{");
    fmt.cbox(INDENT);
    fmt.zerobreak();

    DebugSet {
        inner: DebugInner {
            fmt,
            has_fields: false,
        },
    }
}

impl<'a> DebugSet<'a> {
    /// Adds a new entry to the set output.
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
    ///         fmt.debug_set()
    ///             .entry(&self.0) // Adds the first "entry".
    ///             .entry(&self.1) // Adds the second "entry".
    ///             .finish()
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     debug3::pprint(Foo(vec![10, 11], vec![12, 13])),
    ///     "{[10, 11], [12, 13]}",
    /// );
    /// ```
    pub fn entry(&mut self, entry: &dyn Debug) -> &mut Self {
        self.inner.entry(entry);
        self
    }

    /// Adds the contents of an iterator of entries to the set output.
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
    ///         fmt.debug_set()
    ///             .entries(self.0.iter()) // Adds the first "entry".
    ///             .entries(self.1.iter()) // Adds the second "entry".
    ///             .finish()
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     debug3::pprint(Foo(vec![10, 11], vec![12, 13])),
    ///     "{10, 11, 12, 13}",
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
    ///         fmt.debug_set().entries(self.0.iter()).finish() // Ends the struct formatting.
    ///     }
    /// }
    ///
    /// assert_eq!(debug3::pprint(Foo(vec![10, 11])), "{10, 11}",);
    /// ```
    pub fn finish(&mut self) {
        // TODO: Move common code to Inner (this and DebugList).

        if self.inner.has_fields {
            self.inner.fmt.trailing_comma(true);
        }

        self.inner.fmt.offset(-INDENT);
        self.inner.fmt.end();
        self.inner.fmt.word("}");
    }
}
