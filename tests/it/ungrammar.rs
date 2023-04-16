#![cfg(feature = "ungrammar")]

use expect_test::{expect, Expect};
use ungrammar::Grammar;

fn check_ok(g: &str, expect: Expect) {
    let gram: Grammar = g.parse().unwrap();
    crate::check(&gram, expect);
}

#[test]
fn basic() {
    check_ok(
        r"
Module =
  Attr* Visibility?
  'mod' Name
  (ItemList | ';')

Attr = '#[lol]'
Visibility = 'pub'
Name = '@name'
ItemList = '{' '}'
    ",
        expect![[r##"
            Grammar {
                nodes: [
                    NodeData {
                        name: "Module",
                        rule: Seq(
                            [
                                Rep(Node(Node(1))),
                                Opt(Node(Node(2))),
                                Token(Token(0)),
                                Node(Node(3)),
                                Alt([Node(Node(4)), Token(Token(1))]),
                            ],
                        ),
                    },
                    NodeData {
                        name: "Attr",
                        rule: Token(Token(2)),
                    },
                    NodeData {
                        name: "Visibility",
                        rule: Token(Token(3)),
                    },
                    NodeData {
                        name: "Name",
                        rule: Token(Token(4)),
                    },
                    NodeData {
                        name: "ItemList",
                        rule: Seq([Token(Token(5)), Token(Token(6))]),
                    },
                ],
                tokens: [
                    TokenData { name: "mod" },
                    TokenData { name: ";" },
                    TokenData { name: "#[lol]" },
                    TokenData { name: "pub" },
                    TokenData { name: "@name" },
                    TokenData { name: "{" },
                    TokenData { name: "}" },
                ],
            }"##]],
    );
}
