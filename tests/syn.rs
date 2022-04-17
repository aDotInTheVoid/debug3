use debug3::{pprint, Debug};
use syn::{ ExprCall, ExprField};
use expect_test::{expect, Expect};

fn check(actual: impl Debug, expacted: Expect) {
    expacted.assert_eq(&pprint(actual));
}

#[test]
fn char_lit() {
    let x = "'x'";
    let y: syn::Expr = syn::parse_str(x).unwrap();
    check(y, expect![[r#"
        Lit(
            ExprLit {
                attrs: [],
                lit: Char(LitChar { token: 'x' }),
            },
        )"#]]);
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
    check(y, expect![[r#"
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
                                        span: Span {
                                            start: LineColumn { line: 1, column: 5 },
                                            end: LineColumn { line: 1, column: 6 },
                                        },
                                    },
                                    arguments: None,
                                },
                            ],
                        },
                    },
                ),
            },
        )"#]]);
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
    check(no_punct, expect![[r#"
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
                                    span: Span {
                                        start: LineColumn { line: 1, column: 0 },
                                        end: LineColumn { line: 1, column: 3 },
                                    },
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
        }"#]]);
    check(punct, expect![[r#"
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
                                    span: Span {
                                        start: LineColumn { line: 1, column: 0 },
                                        end: LineColumn { line: 1, column: 3 },
                                    },
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
        }"#]]);
    check(trailing_comma, expect![[r#"
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
                                    span: Span {
                                        start: LineColumn { line: 1, column: 0 },
                                        end: LineColumn { line: 1, column: 3 },
                                    },
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
        }"#]]);
}

#[test]
fn tuple_index_span() {
    let expr: ExprField = syn::parse_str("answer.42").unwrap();
    check(expr, expect![[r#"
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
                                    span: Span {
                                        start: LineColumn { line: 1, column: 0 },
                                        end: LineColumn { line: 1, column: 6 },
                                    },
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
                    span: Span {
                        start: LineColumn { line: 1, column: 7 },
                        end: LineColumn { line: 1, column: 9 },
                    },
                },
            ),
        }"#]])
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


