use debug3::pprint;
use syn::{Expr, ExprCall, ExprField};

#[test]
fn char_lit() {
    let x = "'x'";
    let y: syn::Expr = syn::parse_str(x).unwrap();
    assert_eq!(
        pprint(y),
        "\
Lit(
    ExprLit {
        attrs: [],
        lit: Char(LitChar { token: 'x' }),
    },
)"
    )
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
    assert_eq!(
        pprint(y),
        "\
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
                            ident: ,
                            arguments: None,
                        },
                    ],
                },
            },
        ),
    },
)"
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
    assert_eq!(
        pprint(no_punct.args),
        "\
[
    Lit(
        ExprLit {
            attrs: [],
            lit: Int(LitInt { token: 1 }),
        },
    ),
]"
    );
    assert_eq!(
        pprint(punct.args),
        "\
[
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
]"
    );
    assert_eq!(
        pprint(trailing_comma.args),
        "\
[
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
]"
    );
}

#[test]
fn tuple_index_span() {
    let expr: ExprField = syn::parse_str("answer.42").unwrap();
    assert_eq!(
        pprint(expr),
        "\
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
                        ident: ,
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
}"
    );
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
