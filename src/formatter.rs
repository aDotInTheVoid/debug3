use crate::{
    algorithm::Printer,
    builders::{self, DebugList, DebugMap, DebugSet, DebugStruct, DebugTuple},
    Formatter,
};

impl<'a> Formatter {
    pub fn new() -> Self {
        Self { p: Printer::new() }
    }

    pub(crate) fn write_debug<T: std::fmt::Debug + ?Sized>(&mut self, val: &T) {
        let s = format!("{:?}", val);
        // self.buf.write_str(&s);
        self.p.word(s);
    }

    /// Creates a [`DebugStruct`] builder designed to assist with creation of
    /// [`crate::Debug`] implementations for structs.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::fmt;
    /// use std::net::Ipv4Addr;
    ///
    /// struct Foo {
    ///     bar: i32,
    ///     baz: String,
    ///     addr: Ipv4Addr,
    /// }
    ///
    /// impl fmt::Debug for Foo {
    ///     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    ///         fmt.debug_struct("Foo")
    ///             .field("bar", &self.bar)
    ///             .field("baz", &self.baz)
    ///             .field("addr", &format_args!("{}", self.addr))
    ///             .finish()
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     "Foo { bar: 10, baz: \"Hello World\", addr: 127.0.0.1 }",
    ///     format!("{:?}", Foo {
    ///         bar: 10,
    ///         baz: "Hello World".to_string(),
    ///         addr: Ipv4Addr::new(127, 0, 0, 1),
    ///     })
    /// );
    /// ```
    pub fn debug_struct<'b>(&'b mut self, name: &str) -> DebugStruct<'b> {
        builders::strukt::new(self, name)
    }

    /// Creates a `DebugTuple` builder designed to assist with creation of
    /// `fmt::Debug` implementations for tuple structs.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::fmt;
    /// use std::marker::PhantomData;
    ///
    /// struct Foo<T>(i32, String, PhantomData<T>);
    ///
    /// impl<T> fmt::Debug for Foo<T> {
    ///     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    ///         fmt.debug_tuple("Foo")
    ///             .field(&self.0)
    ///             .field(&self.1)
    ///             .field(&format_args!("_"))
    ///             .finish()
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     "Foo(10, \"Hello\", _)",
    ///     format!("{:?}", Foo(10, "Hello".to_string(), PhantomData::<u8>))
    /// );
    /// ```
    pub fn debug_tuple<'b>(&'b mut self, name: &str) -> DebugTuple<'b> {
        builders::tuple::new(self, name)
    }

    /// Creates a `DebugList` builder designed to assist with creation of
    /// `fmt::Debug` implementations for list-like structures.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::fmt;
    ///
    /// struct Foo(Vec<i32>);
    ///
    /// impl fmt::Debug for Foo {
    ///     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    ///         fmt.debug_list().entries(self.0.iter()).finish()
    ///     }
    /// }
    ///
    /// assert_eq!(format!("{:?}", Foo(vec![10, 11])), "[10, 11]");
    /// ```
    pub fn debug_list<'b>(&'b mut self) -> DebugList<'b> {
        builders::list::new(self)
    }

    /// Creates a `DebugSet` builder designed to assist with creation of
    /// `fmt::Debug` implementations for set-like structures.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::fmt;
    ///
    /// struct Foo(Vec<i32>);
    ///
    /// impl fmt::Debug for Foo {
    ///     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    ///         fmt.debug_set().entries(self.0.iter()).finish()
    ///     }
    /// }
    ///
    /// assert_eq!(format!("{:?}", Foo(vec![10, 11])), "{10, 11}");
    /// ```
    pub fn debug_set<'b>(&'b mut self) -> DebugSet<'b> {
        builders::set::new(self)
    }

    /// Creates a `DebugMap` builder designed to assist with creation of
    /// `fmt::Debug` implementations for map-like structures.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::fmt;
    ///
    /// struct Foo(Vec<(String, i32)>);
    ///
    /// impl fmt::Debug for Foo {
    ///     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    ///         fmt.debug_map().entries(self.0.iter().map(|&(ref k, ref v)| (k, v))).finish()
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     format!("{:?}",  Foo(vec![("A".to_string(), 10), ("B".to_string(), 11)])),
    ///     r#"{"A": 10, "B": 11}"#
    ///  );
    /// ```
    pub fn debug_map<'b>(&'b mut self) -> DebugMap<'b> {
        builders::map::new(self)
    }
}

// impl std::fmt::Write for Formatter {
//     fn write_str(&mut self, s: &str) -> std::fmt::Result {
//         self.buf.write_str(s).map_err(|_| std::fmt::Error)
//     }

//     fn write_char(&mut self, c: char) -> std::fmt::Result {
//         self.buf.write_char(c).map_err(|_| std::fmt::Error)
//     }
// }
