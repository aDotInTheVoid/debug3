use crate::{Debug, Formatter, INDENT};

use super::DebugInner;

/// A struct to help with [`Debug`](Debug) implementations.
///
/// This is useful when you wish to output a formatted list of items as a part
/// of your [`Debug::fmt`] implementation, but to also name the list.
///
/// It's also usefull if you want to make your output look like Mathmatica.
/// 
/// This can be constructed by the [`Formatter::debug_named_list`] method.
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
///         fmt.debug_named_list("Foo").entries(self.0.iter()).finish()
///     }
/// }
///
/// assert_eq!(debug3::pprint(Foo(vec![10, 11])), "Foo [10, 11]",);
/// ```
#[must_use = "must eventually call `finish()` on Debug builders"]
pub struct DebugNamedList<'a> {
    inner: DebugInner<'a>,
}

pub(crate) fn new<'a>(fmt: &'a mut Formatter, name: &str) -> DebugNamedList<'a> {
    fmt.word_s(name);
    fmt.word(" [");
    fmt.cbox(INDENT);
    fmt.zerobreak();

    DebugNamedList {
        inner: DebugInner { fmt: fmt, has_fields: false }
    }
}

impl<'a> DebugNamedList<'a> {
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
    ///         fmt.debug_named_list("Foo")
    ///             .entry(&self.0) // We add the first "entry".
    ///             .entry(&self.1) // We add the second "entry".
    ///             .finish()
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     debug3::pprint(Foo(vec![10, 11], vec![12, 13])),
    ///     "Foo [[10, 11], [12, 13]]",
    /// );
    /// ```
    pub fn entry(&mut self, value: &dyn Debug) -> &mut Self {
        self.inner.entry(value);
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
    ///         fmt.debug_named_list("Foo")
    ///             .entries(self.0.iter())
    ///             .entries(self.1.iter())
    ///             .finish()
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     debug3::pprint(Foo(vec![10, 11], vec![12, 13])),
    ///     "Foo [10, 11, 12, 13]",
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
    ///         fmt.debug_named_list("Foo").entries(self.0.iter()).finish() // Ends the struct formatting.
    ///     }
    /// }
    ///
    /// assert_eq!(debug3::pprint(Foo(vec![10, 11])), "Foo [10, 11]",);
    /// ```
    pub fn finish(&mut self) {
        if self.inner.has_fields {
            self.inner.fmt.trailing_comma(true);
        }

        self.inner.fmt.offset(-INDENT);
        self.inner.fmt.end();
        self.inner.fmt.word("]");
    }
}
