mod debug_struct {
    use debug3::{Debug, Formatter};

    #[test]
    fn test_empty() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter) {
                fmt.debug_struct("Foo").finish()
            }
        }

        assert_eq!("Foo", debug3::pprint(Foo));
        assert_eq!("Foo", debug3::pprint(Foo));
    }

    #[test]
    fn test_single() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter) {
                fmt.debug_struct("Foo").field("bar", &true).finish()
            }
        }

        assert_eq!("Foo { bar: true }", debug3::pprint(Foo));
    }

    #[test]
    fn test_multiple() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter) {
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

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter) {
                fmt.debug_struct("Foo")
                    .field("bar", &true)
                    .field("baz", &format_args!("{}/{}", 10, 20))
                    .finish()
            }
        }

        struct Bar;

        impl Debug for Bar {
            fn fmt(&self, fmt: &mut Formatter) {
                fmt.debug_struct("Bar")
                    .field("foo", &Foo)
                    .field("hello", &"world")
                    .finish()
            }
        }

        assert_eq!(
            "\
Bar {
    foo: Foo { bar: true, baz: 10/20 },
    hello: \"world\",
}",
            debug3::pprint(Bar)
        );
    }

    #[test]
    fn test_only_non_exhaustive() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter) {
                fmt.debug_struct("Foo").finish_non_exhaustive()
            }
        }

        assert_eq!("Foo { .. }", debug3::pprint(Foo));
        assert_eq!("Foo { .. }", debug3::pprint(Foo));
    }

    #[test]
    fn test_multiple_and_non_exhaustive() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter) {
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

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter) {
                fmt.debug_struct("Foo")
                    .field("bar", &true)
                    .field("baz", &format_args!("{}/{}", 10, 20))
                    .finish_non_exhaustive()
            }
        }

        struct Bar;

        impl Debug for Bar {
            fn fmt(&self, fmt: &mut Formatter) {
                fmt.debug_struct("Bar")
                    .field("foo", &Foo)
                    .field("hello", &"world")
                    .finish_non_exhaustive()
            }
        }

        assert_eq!(
            "\
Bar {
    foo: Foo { bar: true, baz: 10/20, .. },
    hello: \"world\",
    ..
}",
            debug3::pprint(Bar)
        );
    }
}

mod debug_tuple {
    use debug3::{Debug, Formatter};

    #[test]
    fn test_empty() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter) {
                fmt.debug_tuple("Foo").finish()
            }
        }

        assert_eq!("Foo", debug3::pprint(Foo));
        assert_eq!("Foo", debug3::pprint(Foo));
    }

    #[test]
    fn test_single() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter) {
                fmt.debug_tuple("Foo").field(&true).finish()
            }
        }

        assert_eq!("Foo(true)", debug3::pprint(Foo));
    }

    #[test]
    fn test_multiple() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter) {
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

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter) {
                fmt.debug_tuple("Foo")
                    .field(&true)
                    .field(&format_args!("{}/{}", 10, 20))
                    .finish()
            }
        }

        struct Bar;

        impl Debug for Bar {
            fn fmt(&self, fmt: &mut Formatter) {
                fmt.debug_tuple("Bar").field(&Foo).field(&"world").finish()
            }
        }

        assert_eq!("Bar(Foo(true, 10/20), \"world\")", debug3::pprint(Bar));
    }
}

mod debug_map {
    use debug3::{Debug, Formatter};

    #[test]
    fn test_empty() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter) {
                fmt.debug_map().finish()
            }
        }

        assert_eq!("{}", debug3::pprint(Foo));
        assert_eq!("{}", debug3::pprint(Foo));
    }

    #[test]
    fn test_single() {
        struct Entry;

        impl Debug for Entry {
            fn fmt(&self, fmt: &mut Formatter) {
                fmt.debug_map().entry(&"bar", &true).finish()
            }
        }

        struct KeyValue;

        impl Debug for KeyValue {
            fn fmt(&self, fmt: &mut Formatter) {
                fmt.debug_map().key(&"bar").value(&true).finish()
            }
        }

        assert_eq!(debug3::pprint(Entry), debug3::pprint(KeyValue));
        assert_eq!("{\"bar\": true}", debug3::pprint(Entry));
    }

    #[test]
    fn test_multiple() {
        struct Entry;

        impl Debug for Entry {
            fn fmt(&self, fmt: &mut Formatter) {
                fmt.debug_map()
                    .entry(&"bar", &true)
                    .entry(&10, &format_args!("{}/{}", 10, 20))
                    .finish()
            }
        }

        struct KeyValue;

        impl Debug for KeyValue {
            fn fmt(&self, fmt: &mut Formatter) {
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

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter) {
                fmt.debug_map()
                    .entry(&"bar", &true)
                    .entry(&10, &format_args!("{}/{}", 10, 20))
                    .finish()
            }
        }

        struct Bar;

        impl Debug for Bar {
            fn fmt(&self, fmt: &mut Formatter) {
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
    #[should_panic]
    fn test_invalid_key_when_entry_is_incomplete() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter) {
                fmt.debug_map().key(&"bar").key(&"invalid").finish()
            }
        }

        debug3::pprint(Foo);
    }

    #[test]
    #[should_panic]
    fn test_invalid_finish_incomplete_entry() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter) {
                fmt.debug_map().key(&"bar").finish()
            }
        }

        debug3::pprint(Foo);
    }

    #[test]
    #[should_panic]
    fn test_invalid_value_before_key() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter) {
                fmt.debug_map().value(&"invalid").key(&"bar").finish()
            }
        }

        debug3::pprint(Foo);
    }
}

mod debug_set {
    use debug3::{Debug, Formatter};

    #[test]
    fn test_empty() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter) {
                fmt.debug_set().finish()
            }
        }

        assert_eq!("{}", debug3::pprint(Foo));
        assert_eq!("{}", debug3::pprint(Foo));
    }

    #[test]
    fn test_single() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter) {
                fmt.debug_set().entry(&true).finish()
            }
        }

        assert_eq!("{true}", debug3::pprint(Foo));
    }

    #[test]
    fn test_multiple() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter) {
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

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter) {
                fmt.debug_set()
                    .entry(&true)
                    .entry(&format_args!("{}/{}", 10, 20))
                    .finish()
            }
        }

        struct Bar;

        impl Debug for Bar {
            fn fmt(&self, fmt: &mut Formatter) {
                fmt.debug_set().entry(&Foo).entry(&"world").finish()
            }
        }

        assert_eq!("{{true, 10/20}, \"world\"}", debug3::pprint(Bar));
    }
}

mod debug_list {
    use debug3::{Debug, Formatter};

    #[test]
    fn test_empty() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter) {
                fmt.debug_list().finish()
            }
        }

        assert_eq!("[]", debug3::pprint(Foo));
        assert_eq!("[]", debug3::pprint(Foo));
    }

    #[test]
    fn test_single() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter) {
                fmt.debug_list().entry(&true).finish()
            }
        }

        assert_eq!("[true]", debug3::pprint(Foo));
    }

    #[test]
    fn test_multiple() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter) {
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

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter) {
                fmt.debug_list()
                    .entry(&true)
                    .entry(&format_args!("{}/{}", 10, 20))
                    .finish()
            }
        }

        struct Bar;

        impl Debug for Bar {
            fn fmt(&self, fmt: &mut Formatter) {
                fmt.debug_list().entry(&Foo).entry(&"world").finish()
            }
        }

        assert_eq!("[[true, 10/20], \"world\"]", debug3::pprint(Bar));
    }
}
