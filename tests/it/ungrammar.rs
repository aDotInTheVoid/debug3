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
                nodes: {
                    "Module": Seq(
                        [
                            Rep(Node(Node(1))),
                            Opt(Node(Node(2))),
                            Token(Token(0)),
                            Node(Node(3)),
                            Alt([Node(Node(4)), Token(Token(1))]),
                        ],
                    ),
                    "Attr": Token(Token(2)),
                    "Visibility": Token(Token(3)),
                    "Name": Token(Token(4)),
                    "ItemList": Seq([Token(Token(5)), Token(Token(6))]),
                },
                tokens: {"mod", ";", "#[lol]", "pub", "@name", "{", "}"},
            }"##]],
    );
}
