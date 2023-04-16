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
                            Rep(Node("Attr")),
                            Opt(Node("Visibility")),
                            Token("mod"),
                            Node("Name"),
                            Alt([Node("ItemList"), Token(";")]),
                        ],
                    ),
                    "Attr": Token("#[lol]"),
                    "Visibility": Token("pub"),
                    "Name": Token("@name"),
                    "ItemList": Seq([Token("{"), Token("}")]),
                },
                tokens: {"mod", ";", "#[lol]", "pub", "@name", "{", "}"},
            }"##]],
    );
}

#[test]
fn all_forms() {
    check_ok(
        r##"
Labeled = a:'a'
Node = SomeOtherNode
Token = 'a'
Seq = 'a' 'b'
Alt = 'a' | 'b'
Opt = 'a'?
Rep = 'a'*

SomeOtherNode = 'a'
"##,
        expect![[r#"
            Grammar {
                nodes: {
                    "Labeled": Labeled {
                        label: "a",
                        rule: Token("a"),
                    },
                    "Node": Node("SomeOtherNode"),
                    "SomeOtherNode": Token("a"),
                    "Token": Token("a"),
                    "Seq": Seq([Token("a"), Token("b")]),
                    "Alt": Alt([Token("a"), Token("b")]),
                    "Opt": Opt(Token("a")),
                    "Rep": Rep(Token("a")),
                },
                tokens: {"a", "b"},
            }"#]],
    );
}
