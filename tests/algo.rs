use debug3::{pprint_new, Debug, Formatter};

#[test]
fn works() {
    #[derive(Debug)]
    struct Basic {
        a: String,
        b: i32,
    }

    assert_eq!(
        pprint_new(Basic {
            a: "Hello World".to_string(),
            b: 10
        }),
        "Basic { a: \"Hello World\", b: 10 }",
    );

    assert_eq!(
        pprint_new(Basic {
            a: "This is a very long string, so that the function will be required to spill"
                .to_string(),
            b: 10
        }),
        "Basic {
    a: \"This is a very long string, so that the function will be required to spill\",
    b: 10,
}",
    );
}

#[test]
fn empty_struct() {
    #[derive(Debug, std::fmt::Debug)]
    struct Empty {}

    assert_eq!(pprint_new(Empty {}), "Empty {}");
}

#[test]
fn non_exaustive() {
    struct Bar {
        bar: i32,
        bas: &'static str,
        #[allow(dead_code)]
        hidden: f32,
    }

    impl Debug for Bar {
        fn fmt(&self, fmt: &mut Formatter<'_>) {
            fmt.debug_struct("Bar")
                .field("bar", &self.bar)
                .field("bas", &self.bas)
                .finish_non_exhaustive()
        }
    }

    assert_eq!(
        pprint_new(Bar {
            bar: 10,
            bas: "Hey",
            hidden: 0.1
        }),
        "Bar { bar: 10, bas: \"Hey\", .. }",
    );

    assert_eq!(
        pprint_new(Bar {
            bar: 10,
            bas: "Hey Now, Your an all star",
            hidden: 0.1
        }),
        "\
Bar {
    bar: 10,
    bas: \"Hey Now, Your an all star\",
    ..
}",
    );
}

#[test]
fn non_exaustive_empty() {
    struct Foo;

    impl Debug for Foo {
        fn fmt(&self, f: &mut Formatter<'_>) {
            f.debug_struct("Foo").finish_non_exhaustive()
        }
    }

    assert_eq!(pprint_new(Foo {}), "Foo { .. }");
}

mod debug_struct {
    use debug3::{Debug, Formatter};

    #[test]
    fn test_empty() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) {
                fmt.debug_struct("Foo").finish()
            }
        }

        assert_eq!("Foo {}", debug3::pprint_new(Foo));
        assert_eq!("Foo {}", debug3::pprint_new(Foo));
    }

    #[test]
    fn test_single() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) {
                fmt.debug_struct("Foo").field("bar", &true).finish()
            }
        }

        assert_eq!("Foo { bar: true }", debug3::pprint_new(Foo));
    }

    #[test]
    fn test_multiple() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) {
                fmt.debug_struct("Foo")
                    .field("bar", &true)
                    .field("baz", &format_args!("{}/{}", 10, 20))
                    .finish()
            }
        }

        assert_eq!("Foo { bar: true, baz: 10/20 }", debug3::pprint_new(Foo));
    }

    #[test]
    fn test_nested() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) {
                fmt.debug_struct("Foo")
                    .field("bar", &true)
                    .field("baz", &format_args!("{}/{}", 10, 20))
                    .finish()
            }
        }

        struct Bar;

        impl Debug for Bar {
            fn fmt(&self, fmt: &mut Formatter<'_>) {
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
            debug3::pprint_new(Bar)
        );
    }

    #[test]
    fn test_only_non_exhaustive() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) {
                fmt.debug_struct("Foo").finish_non_exhaustive()
            }
        }

        assert_eq!("Foo { .. }", debug3::pprint_new(Foo));
        assert_eq!("Foo { .. }", debug3::pprint_new(Foo));
    }

    #[test]
    fn test_multiple_and_non_exhaustive() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) {
                fmt.debug_struct("Foo")
                    .field("bar", &true)
                    .field("baz", &format_args!("{}/{}", 10, 20))
                    .finish_non_exhaustive()
            }
        }

        assert_eq!("Foo { bar: true, baz: 10/20, .. }", debug3::pprint_new(Foo));
    }

    #[test]
    fn test_nested_non_exhaustive() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) {
                fmt.debug_struct("Foo")
                    .field("bar", &true)
                    .field("baz", &format_args!("{}/{}", 10, 20))
                    .finish_non_exhaustive()
            }
        }

        struct Bar;

        impl Debug for Bar {
            fn fmt(&self, fmt: &mut Formatter<'_>) {
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
            debug3::pprint_new(Bar)
        );
    }
}
