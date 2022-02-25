mod debug_struct {
    use debug3 as fmt;

    #[test]
    fn test_empty() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_struct("Foo").finish()
            }
        }

        assert_eq!("Foo", debug3::pprint(Foo));
        assert_eq!("Foo", debug3::pprint(Foo));
    }

    #[test]
    fn test_single() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_struct("Foo").field("bar", &true).finish()
            }
        }

        assert_eq!("Foo { bar: true }", debug3::pprint(Foo));
    }

    #[test]
    fn test_multiple() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_struct("Foo")
                    .field("bar", &true)
                    .field("baz", &format_args!("{}/{}", 10, 20))
                    .finish()
            }
        }

        assert_eq!("Foo { bar: true, baz: 10/20 }", debug3::pprint(Foo));
    }

    #[test]
    fn test_nested() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_struct("Foo")
                    .field("bar", &true)
                    .field("baz", &format_args!("{}/{}", 10, 20))
                    .finish()
            }
        }

        struct Bar;

        impl fmt::Debug for Bar {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_struct("Bar")
                    .field("foo", &Foo)
                    .field("hello", &"world")
                    .finish()
            }
        }

        assert_eq!(
            "Bar { foo: Foo { bar: true, baz: 10/20 }, hello: \"world\" }",
            debug3::pprint(Bar)
        );
    }

    #[test]
    fn test_only_non_exhaustive() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_struct("Foo").finish_non_exhaustive()
            }
        }

        assert_eq!("Foo { .. }", debug3::pprint(Foo));
        assert_eq!("Foo { .. }", debug3::pprint(Foo));
    }

    #[test]
    fn test_multiple_and_non_exhaustive() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_struct("Foo")
                    .field("bar", &true)
                    .field("baz", &format_args!("{}/{}", 10, 20))
                    .finish_non_exhaustive()
            }
        }

        assert_eq!("Foo { bar: true, baz: 10/20, .. }", debug3::pprint(Foo));
    }

    #[test]
    fn test_nested_non_exhaustive() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_struct("Foo")
                    .field("bar", &true)
                    .field("baz", &format_args!("{}/{}", 10, 20))
                    .finish_non_exhaustive()
            }
        }

        struct Bar;

        impl fmt::Debug for Bar {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_struct("Bar")
                    .field("foo", &Foo)
                    .field("hello", &"world")
                    .finish_non_exhaustive()
            }
        }

        assert_eq!(
            "Bar { foo: Foo { bar: true, baz: 10/20, .. }, hello: \"world\", .. }",
            debug3::pprint(Bar)
        );
    }
}

mod debug_tuple {
    use debug3 as fmt;

    #[test]
    fn test_empty() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_tuple("Foo").finish()
            }
        }

        assert_eq!("Foo", debug3::pprint(Foo));
        assert_eq!("Foo", debug3::pprint(Foo));
    }

    #[test]
    fn test_single() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_tuple("Foo").field(&true).finish()
            }
        }

        assert_eq!("Foo(true)", debug3::pprint(Foo));
    }

    #[test]
    fn test_multiple() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_tuple("Foo")
                    .field(&true)
                    .field(&format_args!("{}/{}", 10, 20))
                    .finish()
            }
        }

        assert_eq!("Foo(true, 10/20)", debug3::pprint(Foo));
    }

    #[test]
    fn test_nested() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_tuple("Foo")
                    .field(&true)
                    .field(&format_args!("{}/{}", 10, 20))
                    .finish()
            }
        }

        struct Bar;

        impl fmt::Debug for Bar {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_tuple("Bar").field(&Foo).field(&"world").finish()
            }
        }

        assert_eq!("Bar(Foo(true, 10/20), \"world\")", debug3::pprint(Bar));
    }
}

mod debug_map {
    use debug3 as fmt;

    #[test]
    fn test_empty() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_map().finish()
            }
        }

        assert_eq!("{}", debug3::pprint(Foo));
        assert_eq!("{}", debug3::pprint(Foo));
    }

    #[test]
    fn test_single() {
        struct Entry;

        impl fmt::Debug for Entry {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_map().entry(&"bar", &true).finish()
            }
        }

        struct KeyValue;

        impl fmt::Debug for KeyValue {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_map().key(&"bar").value(&true).finish()
            }
        }

        assert_eq!(debug3::pprint(Entry), debug3::pprint(KeyValue));
        assert_eq!("{\"bar\": true}", debug3::pprint(Entry));
    }

    #[test]
    fn test_multiple() {
        struct Entry;

        impl fmt::Debug for Entry {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_map()
                    .entry(&"bar", &true)
                    .entry(&10, &format_args!("{}/{}", 10, 20))
                    .finish()
            }
        }

        struct KeyValue;

        impl fmt::Debug for KeyValue {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_map()
                    .key(&"bar")
                    .value(&true)
                    .key(&10)
                    .value(&format_args!("{}/{}", 10, 20))
                    .finish()
            }
        }

        assert_eq!(debug3::pprint(Entry), debug3::pprint(KeyValue));
        assert_eq!(debug3::pprint(Entry), debug3::pprint(KeyValue));

        assert_eq!("{\"bar\": true, 10: 10/20}", debug3::pprint(Entry));
    }

    #[test]
    fn test_nested() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_map()
                    .entry(&"bar", &true)
                    .entry(&10, &format_args!("{}/{}", 10, 20))
                    .finish()
            }
        }

        struct Bar;

        impl fmt::Debug for Bar {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_map()
                    .entry(&"foo", &Foo)
                    .entry(&Foo, &"world")
                    .finish()
            }
        }

        assert_eq!(
            "{\"foo\": {\"bar\": true, 10: 10/20}, \
                    {\"bar\": true, 10: 10/20}: \"world\"}",
            debug3::pprint(Bar)
        );
        // TODO: Get the better output

        //         assert_eq!(
        //             "{
        //     \"foo\": {
        //         \"bar\": true,
        //         10: 10/20,
        //     },
        //     {
        //         \"bar\": true,
        //         10: 10/20,
        //     }: \"world\",
        // }",
        //             debug3::pprint(Bar)
        //         );
    }

    #[test]
    fn test_entry_err() {
        // Ensure errors in a map entry don't trigger panics (#65231)
        use std::fmt::Write;

        struct ErrorFmt;

        impl fmt::Debug for ErrorFmt {
            fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
                Err(fmt::Error {})
            }
        }

        struct KeyValue<K, V>(usize, K, V);

        impl<K, V> fmt::Debug for KeyValue<K, V>
        where
            K: fmt::Debug,
            V: fmt::Debug,
        {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                let mut map = fmt.debug_map();

                for _ in 0..self.0 {
                    map.entry(&self.1, &self.2);
                }

                map.finish()
            }
        }

        let mut buf = String::new();

        assert!(write!(&mut buf, "{:?}", StdAs3(KeyValue(1, ErrorFmt, "bar"))).is_err());
        assert!(write!(&mut buf, "{:?}", StdAs3(KeyValue(1, "foo", ErrorFmt))).is_err());
        assert!(write!(&mut buf, "{:?}", StdAs3(KeyValue(2, ErrorFmt, "bar"))).is_err());
        assert!(write!(&mut buf, "{:?}", StdAs3(KeyValue(2, "foo", ErrorFmt))).is_err());

        struct StdAs3<T>(T);
        impl<T: debug3::Debug> std::fmt::Debug for StdAs3<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut f3 = fmt::Formatter::new(f);
                debug3::Debug::fmt(&self.0, &mut f3).map_err(|_| std::fmt::Error)
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_invalid_key_when_entry_is_incomplete() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_map().key(&"bar").key(&"invalid").finish()
            }
        }

        debug3::pprint(Foo);
    }

    #[test]
    #[should_panic]
    fn test_invalid_finish_incomplete_entry() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_map().key(&"bar").finish()
            }
        }

        debug3::pprint(Foo);
    }

    #[test]
    #[should_panic]
    fn test_invalid_value_before_key() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_map().value(&"invalid").key(&"bar").finish()
            }
        }

        debug3::pprint(Foo);
    }
}

mod debug_set {
    use debug3 as fmt;

    #[test]
    fn test_empty() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_set().finish()
            }
        }

        assert_eq!("{}", debug3::pprint(Foo));
        assert_eq!("{}", debug3::pprint(Foo));
    }

    #[test]
    fn test_single() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_set().entry(&true).finish()
            }
        }

        assert_eq!("{true}", debug3::pprint(Foo));
    }

    #[test]
    fn test_multiple() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_set()
                    .entry(&true)
                    .entry(&format_args!("{}/{}", 10, 20))
                    .finish()
            }
        }

        assert_eq!("{true, 10/20}", debug3::pprint(Foo));
    }

    #[test]
    fn test_nested() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_set()
                    .entry(&true)
                    .entry(&format_args!("{}/{}", 10, 20))
                    .finish()
            }
        }

        struct Bar;

        impl fmt::Debug for Bar {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_set().entry(&Foo).entry(&"world").finish()
            }
        }

        assert_eq!("{{true, 10/20}, \"world\"}", debug3::pprint(Bar));
    }
}

mod debug_list {
    use debug3 as fmt;

    #[test]
    fn test_empty() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_list().finish()
            }
        }

        assert_eq!("[]", debug3::pprint(Foo));
        assert_eq!("[]", debug3::pprint(Foo));
    }

    #[test]
    fn test_single() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_list().entry(&true).finish()
            }
        }

        assert_eq!("[true]", debug3::pprint(Foo));
    }

    #[test]
    fn test_multiple() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_list()
                    .entry(&true)
                    .entry(&format_args!("{}/{}", 10, 20))
                    .finish()
            }
        }

        assert_eq!("[true, 10/20]", debug3::pprint(Foo));
    }

    #[test]
    fn test_nested() {
        struct Foo;

        impl fmt::Debug for Foo {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_list()
                    .entry(&true)
                    .entry(&format_args!("{}/{}", 10, 20))
                    .finish()
            }
        }

        struct Bar;

        impl fmt::Debug for Bar {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_list().entry(&Foo).entry(&"world").finish()
            }
        }

        assert_eq!("[[true, 10/20], \"world\"]", debug3::pprint(Bar));
    }
}

#[test]
fn test_formatting_parameters_are_forwarded() {
    use std::collections::{BTreeMap, BTreeSet};
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Foo {
        bar: u32,
        baz: u32,
    }
    let struct_ = Foo { bar: 1024, baz: 7 };
    let tuple = (1024, 7);
    let list = [1024, 7];
    let mut map = BTreeMap::new();
    map.insert("bar", 1024);
    map.insert("baz", 7);
    let mut set = BTreeSet::new();
    set.insert(1024);
    set.insert(7);

    assert_eq!(format!("{:03?}", struct_), "Foo { bar: 1024, baz: 007 }");
    assert_eq!(format!("{:03?}", tuple), "(1024, 007)");
    assert_eq!(format!("{:03?}", list), "[1024, 007]");
    assert_eq!(format!("{:03?}", map), r#"{"bar": 1024, "baz": 007}"#);
    assert_eq!(format!("{:03?}", set), "{007, 1024}");
    assert_eq!(
        format!("{:#03?}", struct_),
        "
Foo {
    bar: 1024,
    baz: 007,
}
    "
        .trim()
    );
    assert_eq!(
        format!("{:#03?}", tuple),
        "
(
    1024,
    007,
)
    "
        .trim()
    );
    assert_eq!(
        format!("{:#03?}", list),
        "
[
    1024,
    007,
]
    "
        .trim()
    );
    assert_eq!(
        format!("{:#03?}", map),
        r#"
{
    "bar": 1024,
    "baz": 007,
}
    "#
        .trim()
    );
    assert_eq!(
        format!("{:#03?}", set),
        "
{
    007,
    1024,
}
    "
        .trim()
    );
}
