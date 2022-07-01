#![cfg(syn)]

use debug3::{pprint, Debug};
use expect_test::{expect, Expect};
use syn::{ExprCall, ExprField, ItemMacro};

fn check(actual: impl Debug, expacted: Expect) {
    expacted.assert_eq(&pprint(actual));
}

#[test]
fn char_lit() {
    let x = "'x'";
    let y: syn::Expr = syn::parse_str(x).unwrap();
    check(
        y,
        expect![[r#"
        Lit(
            ExprLit {
                attrs: [],
                lit: Char(LitChar { token: 'x' }),
            },
        )"#]],
    );
}

/*
Lit(
    ExprLit {
        attrs: [],
        lit: Char(
            LitChar {
                token: 'x',
            },
        ),
    },
)
*/

#[test]
fn ref_mut() {
    let x = "&mut x";
    let y: syn::Expr = syn::parse_str(x).unwrap();
    check(
        y,
        expect![[r#"
            Reference(
                ExprReference {
                    attrs: [],
                    and_token: And,
                    raw: Reserved,
                    mutability: Some(Mut),
                    expr: Path(
                        ExprPath {
                            attrs: [],
                            qself: None,
                            path: Path {
                                leading_colon: None,
                                segments: [
                                    PathSegment {
                                        ident: Ident {
                                            sym: "x",
                                            span: Span(1:5: 1:6),
                                        },
                                        arguments: None,
                                    },
                                ],
                            },
                        },
                    ),
                },
            )"#]],
    );
}

/*
Reference(
    ExprReference {
        attrs: [],
        and_token: And,
        raw: Reserved,
        mutability: Some(
            Mut,
        ),
        expr: Path(
            ExprPath {
                attrs: [],
                qself: None,
                path: Path {
                    leading_colon: None,
                    segments: [
                        PathSegment {
                            ident: Ident {
                                sym: x,
                                span: bytes(6..7),
                            },
                            arguments: None,
                        },
                    ],
                },
            },
        ),
    },
)
*/

#[test]
fn punct() {
    let no_punct: ExprCall = syn::parse_str("Foo(1)").unwrap();
    let punct: ExprCall = syn::parse_str("Foo(1,2)").unwrap();
    let trailing_comma: ExprCall = syn::parse_str("Foo(1,2,)").unwrap();
    check(
        no_punct,
        expect![[r#"
            ExprCall {
                attrs: [],
                func: Path(
                    ExprPath {
                        attrs: [],
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments: [
                                PathSegment {
                                    ident: Ident {
                                        sym: "Foo",
                                        span: Span(1:0: 1:3),
                                    },
                                    arguments: None,
                                },
                            ],
                        },
                    },
                ),
                paren_token: Paren,
                args: [
                    Lit(
                        ExprLit {
                            attrs: [],
                            lit: Int(LitInt { token: 1 }),
                        },
                    ),
                ],
            }"#]],
    );
    check(
        punct,
        expect![[r#"
            ExprCall {
                attrs: [],
                func: Path(
                    ExprPath {
                        attrs: [],
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments: [
                                PathSegment {
                                    ident: Ident {
                                        sym: "Foo",
                                        span: Span(1:0: 1:3),
                                    },
                                    arguments: None,
                                },
                            ],
                        },
                    },
                ),
                paren_token: Paren,
                args: [
                    Lit(
                        ExprLit {
                            attrs: [],
                            lit: Int(LitInt { token: 1 }),
                        },
                    ),
                    Comma,
                    Lit(
                        ExprLit {
                            attrs: [],
                            lit: Int(LitInt { token: 2 }),
                        },
                    ),
                ],
            }"#]],
    );
    check(
        trailing_comma,
        expect![[r#"
            ExprCall {
                attrs: [],
                func: Path(
                    ExprPath {
                        attrs: [],
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments: [
                                PathSegment {
                                    ident: Ident {
                                        sym: "Foo",
                                        span: Span(1:0: 1:3),
                                    },
                                    arguments: None,
                                },
                            ],
                        },
                    },
                ),
                paren_token: Paren,
                args: [
                    Lit(
                        ExprLit {
                            attrs: [],
                            lit: Int(LitInt { token: 1 }),
                        },
                    ),
                    Comma,
                    Lit(
                        ExprLit {
                            attrs: [],
                            lit: Int(LitInt { token: 2 }),
                        },
                    ),
                    Comma,
                ],
            }"#]],
    );
}

#[test]
fn tuple_index_span() {
    let expr: ExprField = syn::parse_str("answer.42").unwrap();
    check(
        expr,
        expect![[r#"
            ExprField {
                attrs: [],
                base: Path(
                    ExprPath {
                        attrs: [],
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments: [
                                PathSegment {
                                    ident: Ident {
                                        sym: "answer",
                                        span: Span(1:0: 1:6),
                                    },
                                    arguments: None,
                                },
                            ],
                        },
                    },
                ),
                dot_token: Dot,
                member: Unnamed(
                    Index {
                        index: 42,
                        span: Span(1:7: 1:9),
                    },
                ),
            }"#]],
    )
}

/*
ExprField {
    attrs: [],
    base: Path(
        ExprPath {
            attrs: [],
            qself: None,
            path: Path {
                leading_colon: None,
                segments: [
                    PathSegment {
                        ident: Ident {
                            sym: answer,
                            span: bytes(1..7),
                        },
                        arguments: None,
                    },
                ],
            },
        },
    ),
    dot_token: Dot,
    member: Unnamed(
        Index {
            index: 42,
            span: bytes(8..10),
        },
    ),
}
*/

#[test]
fn attrs() {
    let x: syn::File = syn::parse_str("#![feature(no_core)]").unwrap();
    check(
        x,
        expect![[r#"
            File {
                shebang: None,
                attrs: [
                    Attribute {
                        pound_token: Pound,
                        style: Inner(Bang),
                        bracket_token: Bracket,
                        path: Path {
                            leading_colon: None,
                            segments: [
                                PathSegment {
                                    ident: Ident {
                                        sym: "feature",
                                        span: Span(1:3: 1:10),
                                    },
                                    arguments: None,
                                },
                            ],
                        },
                        tokens: TokenStream [
                            Group {
                                delimiter: Parenthesis,
                                stream: TokenStream [
                                    Ident {
                                        sym: "no_core",
                                        span: Span(1:11: 1:18),
                                    },
                                ],
                                span: Span(1:10: 1:19),
                            },
                        ],
                    },
                ],
                items: [],
            }"#]],
    )
}

/*
File {
    shebang: None,
    attrs: [
        Attribute {
            pound_token: Pound,
            style: Inner(
                Bang,
            ),
            bracket_token: Bracket,
            path: Path {
                leading_colon: None,
                segments: [
                    PathSegment {
                        ident: Ident {
                            sym: feature,
                            span: bytes(4..11),
                        },
                        arguments: None,
                    },
                ],
            },
            tokens: TokenStream [
                Group {
                    delimiter: Parenthesis,
                    stream: TokenStream [
                        Ident {
                            sym: no_core,
                            span: bytes(12..19),
                        },
                    ],
                    span: bytes(11..20),
                },
            ],
        },
    ],
    items: [],
}
*/

#[test]
fn macros() {
    let x: ItemMacro = syn::parse_str(
        "macro_rules! baz {
    () => {
        fn main() {
            let mut x = 3;
            x += 1;
        }
    };
}",
    )
    .unwrap();
    check(
        x,
        expect![[r#"
            ItemMacro {
                attrs: [],
                ident: Some(
                    Ident {
                        sym: "baz",
                        span: Span(1:13: 1:16),
                    },
                ),
                mac: Macro {
                    path: Path {
                        leading_colon: None,
                        segments: [
                            PathSegment {
                                ident: Ident {
                                    sym: "macro_rules",
                                    span: Span(1:0: 1:11),
                                },
                                arguments: None,
                            },
                        ],
                    },
                    bang_token: Bang,
                    delimiter: Brace(Brace),
                    tokens: TokenStream [
                        Group {
                            delimiter: Parenthesis,
                            stream: TokenStream [],
                            span: Span(2:4: 2:6),
                        },
                        Punct {
                            char: '=',
                            spacing: Joint,
                            span: Joint,
                        },
                        Punct {
                            char: '>',
                            spacing: Alone,
                            span: Alone,
                        },
                        Group {
                            delimiter: Brace,
                            stream: TokenStream [
                                Ident {
                                    sym: "fn",
                                    span: Span(3:8: 3:10),
                                },
                                Ident {
                                    sym: "main",
                                    span: Span(3:11: 3:15),
                                },
                                Group {
                                    delimiter: Parenthesis,
                                    stream: TokenStream [],
                                    span: Span(3:15: 3:17),
                                },
                                Group {
                                    delimiter: Brace,
                                    stream: TokenStream [
                                        Ident {
                                            sym: "let",
                                            span: Span(4:12: 4:15),
                                        },
                                        Ident {
                                            sym: "mut",
                                            span: Span(4:16: 4:19),
                                        },
                                        Ident {
                                            sym: "x",
                                            span: Span(4:20: 4:21),
                                        },
                                        Punct {
                                            char: '=',
                                            spacing: Alone,
                                            span: Alone,
                                        },
                                        Literal {
                                            lit: 3,
                                            span: Span(4:24: 4:25),
                                        },
                                        Punct {
                                            char: ';',
                                            spacing: Alone,
                                            span: Alone,
                                        },
                                        Ident {
                                            sym: "x",
                                            span: Span(5:12: 5:13),
                                        },
                                        Punct {
                                            char: '+',
                                            spacing: Joint,
                                            span: Joint,
                                        },
                                        Punct {
                                            char: '=',
                                            spacing: Alone,
                                            span: Alone,
                                        },
                                        Literal {
                                            lit: 1,
                                            span: Span(5:17: 5:18),
                                        },
                                        Punct {
                                            char: ';',
                                            spacing: Alone,
                                            span: Alone,
                                        },
                                    ],
                                    span: Span(3:18: 6:9),
                                },
                            ],
                            span: Span(2:10: 7:5),
                        },
                        Punct {
                            char: ';',
                            spacing: Alone,
                            span: Alone,
                        },
                    ],
                },
                semi_token: None,
            }"#]],
    );
}
