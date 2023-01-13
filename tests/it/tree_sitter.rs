#![cfg(feature = "tree-sitter")]

use debug3::pprint;

use expect_test::{expect, Expect};
use tree_sitter::Parser;

fn check(input: &str, expected: Expect) {
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_c::language()).unwrap();
    let parsed = parser.parse(input, None).unwrap();
    let pp = pprint(&parsed);
    expected.assert_eq(&pp);
}

#[test]
fn basic() {
    check("int double(int x) { return x * x; }", expect![[r#"
        Tree(
            Node {
                kind: "translation_unit",
                child: Node {
                    kind: "function_definition",
                    children: [
                        Node("primitive_type"),
                        Node {
                            kind: "function_declarator",
                            children: [
                                Node("identifier"),
                                Node {
                                    kind: "parameter_list",
                                    children: [
                                        Node("("),
                                        Node {
                                            kind: "parameter_declaration",
                                            children: [Node("primitive_type"), Node("identifier")],
                                        },
                                        Node(")"),
                                    ],
                                },
                            ],
                        },
                        Node {
                            kind: "compound_statement",
                            children: [
                                Node("{"),
                                Node {
                                    kind: "return_statement",
                                    children: [
                                        Node("return"),
                                        Node {
                                            kind: "binary_expression",
                                            children: [
                                                Node("identifier"),
                                                Node("*"),
                                                Node("identifier"),
                                            ],
                                        },
                                        Node(";"),
                                    ],
                                },
                                Node("}"),
                            ],
                        },
                    ],
                },
            },
        )"#]]);
}
