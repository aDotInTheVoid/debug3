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

mod debug_set {
    use debug3::{Debug, Formatter};

    #[test]
    fn test_empty() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) {
                fmt.debug_set().finish()
            }
        }

        assert_eq!("{}", debug3::pprint_new(Foo));
        assert_eq!("{}", debug3::pprint_new(Foo));
    }

    #[test]
    fn test_single() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) {
                fmt.debug_set().entry(&true).finish()
            }
        }

        assert_eq!("{true}", debug3::pprint_new(Foo));
    }

    #[test]
    fn test_multiple() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) {
                fmt.debug_set()
                    .entry(&true)
                    .entry(&format_args!("{}/{}", 10, 20))
                    .finish()
            }
        }

        assert_eq!("{true, 10/20}", debug3::pprint_new(Foo));
    }

    #[test]
    fn test_nested() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) {
                fmt.debug_set()
                    .entry(&true)
                    .entry(&format_args!("{}/{}", 10, 20))
                    .finish()
            }
        }

        struct Bar;

        impl Debug for Bar {
            fn fmt(&self, fmt: &mut Formatter<'_>) {
                fmt.debug_set().entry(&Foo).entry(&"world").finish()
            }
        }

        assert_eq!("{{true, 10/20}, \"world\"}", debug3::pprint_new(Bar));
    }
}

mod debug_list {
    use debug3::{Debug, Formatter};

    #[test]
    fn test_empty() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) {
                fmt.debug_list().finish()
            }
        }

        assert_eq!("[]", debug3::pprint_new(Foo));
        assert_eq!("[]", debug3::pprint_new(Foo));
    }

    #[test]
    fn test_single() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) {
                fmt.debug_list().entry(&true).finish()
            }
        }

        assert_eq!("[true]", debug3::pprint_new(Foo));
    }

    #[test]
    fn test_multiple() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) {
                fmt.debug_list()
                    .entry(&true)
                    .entry(&format_args!("{}/{}", 10, 20))
                    .finish()
            }
        }

        assert_eq!("[true, 10/20]", debug3::pprint_new(Foo));
    }

    #[test]
    fn test_nested() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) {
                fmt.debug_list()
                    .entry(&true)
                    .entry(&format_args!("{}/{}", 10, 20))
                    .finish()
            }
        }

        struct Bar;

        impl Debug for Bar {
            fn fmt(&self, fmt: &mut Formatter<'_>) {
                fmt.debug_list().entry(&Foo).entry(&"world").finish()
            }
        }

        assert_eq!("[[true, 10/20], \"world\"]", debug3::pprint_new(Bar));
    }
}

#[test]
fn list_many() {
    let x: &[&[&[i32]]] = &[
        &[&[1], &[2], &[3]],
        &[
            &[
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 34, 432, 123, 1234, 132432, 123, 312123,
            ],
            &[4, 5, 6],
        ],
    ];

    assert_eq!(
        "\
[
    [[1], [2], [3]],
    [
        [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 34, 432, 123, 1234, 132432, 123, 312123],
        [4, 5, 6],
    ],
]",
        debug3::pprint_new(x)
    );
}

#[test]
fn empty_tuple() {
    assert_eq!("()", debug3::pprint_new(()));
}

#[test]
fn unary_tuple() {
    assert_eq!("(1,)", debug3::pprint_new((1,)));
}

#[test]
fn binary_typle() {
    assert_eq!("(1, 2)", debug3::pprint_new((1, 2)));
}

#[test]
fn tuple_break() {
    let x = (
        "SDBSsdfdasfasdF",
        "FDSssssjgkl;kjgdfskl;jlkgkl;fsgjfdgsfk;gl",
        "dfgskldjfgkldfjkl;dfgj;lfg",
    );
    assert_eq!(
        "\
(
    \"SDBSsdfdasfasdF\",
    \"FDSssssjgkl;kjgdfskl;jlkgkl;fsgjfdgsfk;gl\",
    \"dfgskldjfgkldfjkl;dfgj;lfg\",
)",
        debug3::pprint_new(x)
    );
}

mod debug_tuple {
    use debug3::{Debug, Formatter};

    #[test]
    fn test_empty() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) {
                fmt.debug_tuple("Foo").finish()
            }
        }

        assert_eq!("Foo", debug3::pprint_new(Foo));
        assert_eq!("Foo", debug3::pprint_new(Foo));
    }

    #[test]
    fn test_single() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) {
                fmt.debug_tuple("Foo").field(&true).finish()
            }
        }

        assert_eq!("Foo(true)", debug3::pprint_new(Foo));
    }

    #[test]
    fn test_multiple() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) {
                fmt.debug_tuple("Foo")
                    .field(&true)
                    .field(&format_args!("{}/{}", 10, 20))
                    .finish()
            }
        }

        assert_eq!("Foo(true, 10/20)", debug3::pprint_new(Foo));
    }

    #[test]
    fn test_nested() {
        struct Foo;

        impl Debug for Foo {
            fn fmt(&self, fmt: &mut Formatter<'_>) {
                fmt.debug_tuple("Foo")
                    .field(&true)
                    .field(&format_args!("{}/{}", 10, 20))
                    .finish()
            }
        }

        struct Bar;

        impl Debug for Bar {
            fn fmt(&self, fmt: &mut Formatter<'_>) {
                fmt.debug_tuple("Bar").field(&Foo).field(&"world").finish()
            }
        }

        assert_eq!("Bar(Foo(true, 10/20), \"world\")", debug3::pprint_new(Bar));
    }
}
