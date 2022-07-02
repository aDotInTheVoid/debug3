#![cfg(feature = "kdl")]

use debug3::pprint;
use expect_test::{expect, Expect};
use kdl::KdlDocument;

fn check(kdl: &str, expacted: Expect) {
    let doc: KdlDocument = kdl.parse().expect("failed to parse KDL");
    expacted.assert_eq(&pprint(doc));
}

#[test]
fn kdl_basic() {
    let doc = r#"
hello 1 2 3

world prop="value" {
    child 1
    child 2
}
"#;

    check(
        doc,
        expect![[r#"
        [
            KdlNode {
                name: "hello",
                entries: [1, 2, 3],
            },
            KdlNode {
                name: "world",
                entries: KdlEntry {
                    name: "prop",
                    value: "value",
                },
                children: [
                    KdlNode {
                        name: "child",
                        entries: 1,
                    },
                    KdlNode {
                        name: "child",
                        entries: 2,
                    },
                ],
            },
        ]"#]],
    );
}

#[test]
fn kdl_2() {
    let doc = r#"
  // indented comment
  "formatted" 1 /* comment */ \
    2;
"#;

    check(
        doc,
        expect![[r#"
        [
            KdlNode {
                name: "formatted",
                entries: [1, 2],
            },
        ]"#]],
    );
}

#[test]
fn kdl_3() {
    let doc = r####"
    contents {
        section "First section" {
            paragraph "This is the first paragraph"
            paragraph "This is the second paragraph"
        }
    }
    "####;
    check(
        doc,
        expect![[r#"
        [
            KdlNode {
                name: "contents",
                children: [
                    KdlNode {
                        name: "section",
                        entries: "First section",
                        children: [
                            KdlNode {
                                name: "paragraph",
                                entries: "This is the first paragraph",
                            },
                            KdlNode {
                                name: "paragraph",
                                entries: "This is the second paragraph",
                            },
                        ],
                    },
                ],
            },
        ]"#]],
    );
}

#[test]
fn kdl_4() {
    let doc = r###"
        node "this\nhas\tescapes"
        other r"C:\Users\zkat\"
    "###;
    check(
        doc,
        expect![[r#"
        [
            KdlNode {
                name: "node",
                entries: "this\nhas\tescapes",
            },
            KdlNode {
                name: "other",
                entries: "C:\\Users\\zkat\\",
            },
        ]"#]],
    );
}

#[test]
fn kdl_5() {
    let doc = r###"other-raw r#"hello"world"#"###;
    check(
        doc,
        expect![[r#"
        [
            KdlNode {
                name: "other-raw",
                entries: "hello\"world",
            },
        ]"#]],
    );
}

#[test]
fn kdl_num() {
    let doc = r#"
    num 1.234e-42
    my-hex 0xdeadbeef
    my-octal 0o755
    my-binary 0b10101101
    bignum 1_000_000
"#;
    check(
        doc,
        expect![[r#"
        [
            KdlNode {
                name: "num",
                entries: 1.234e-42,
            },
            KdlNode {
                name: "my-hex",
                entries: 3735928559,
            },
            KdlNode {
                name: "my-octal",
                entries: 493,
            },
            KdlNode {
                name: "my-binary",
                entries: 173,
            },
            KdlNode {
                name: "bignum",
                entries: 1000000,
            },
        ]"#]],
    );
}

#[test]
fn kdl_ty() {
    let doc = r#"
        numbers (u8)10 (i32)20 myfloat=(f32)1.5 {
          strings (uuid)"123e4567-e89b-12d3-a456-426614174000" (date)"2021-02-03" filter=(regex)r"$\d+"
          (author)person name="Alex"
        }"#;
    check(
        doc,
        expect![[r#"
        [
            KdlNode {
                name: "numbers",
                entries: [
                    KdlEntry { ty: "u8", value: 10 },
                    KdlEntry { ty: "i32", value: 20 },
                    KdlEntry {
                        ty: "f32",
                        name: "myfloat",
                        value: 1.5,
                    },
                ],
                children: [
                    KdlNode {
                        name: "strings",
                        entries: [
                            KdlEntry {
                                ty: "uuid",
                                value: "123e4567-e89b-12d3-a456-426614174000",
                            },
                            KdlEntry {
                                ty: "date",
                                value: "2021-02-03",
                            },
                            KdlEntry {
                                ty: "regex",
                                name: "filter",
                                value: "$\\d+",
                            },
                        ],
                    },
                    KdlNode {
                        ty: "author",
                        name: "person",
                        entries: KdlEntry {
                            name: "name",
                            value: "Alex",
                        },
                    },
                ],
            },
        ]"#]],
    );
}

#[test]
fn gttg_1() {
    let doc = r#"
    (PC)shanondale player="nixon" {
        PC {
            Stats {
                Dex 10
            }
        }
    }"#;
    check(doc, expect![[r#"
        [
            KdlNode {
                ty: "PC",
                name: "shanondale",
                entries: KdlEntry {
                    name: "player",
                    value: "nixon",
                },
                children: [
                    KdlNode {
                        name: "PC",
                        children: [
                            KdlNode {
                                name: "Stats",
                                children: [
                                    KdlNode {
                                        name: "Dex",
                                        entries: 10,
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
        ]"#]]);
}
