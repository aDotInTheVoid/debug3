use debug3::pprint;


#[test]
fn char_lit() {
    let x = "'x'";
    let y: syn::Expr = syn::parse_str(x).unwrap();
    assert_eq!(pprint(y), "\
Lit(
    ExprLit {
        attrs: [],
        lit: Char(LitChar { token: 'x' }),
    },
)")
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
    assert_eq!(pprint(y), "\
Reference(
    ExprReference {
        attrs: [],
        and_token: And { spans: [] },
        raw: Reserved,
        mutability: Some(Mut { span:  }),
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
)");
}
