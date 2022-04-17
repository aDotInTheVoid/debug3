use syn::punctuated::Punctuated;

use crate::{Debug, Formatter};

macro_rules! lit {
    ($($ty:ident),*) => {
        $(impl Debug for syn :: $ty {
            fn fmt(&self, f: &mut Formatter) {
                f.debug_struct(stringify!($ty))
                    .field("token", &format_args!("{}", self.token()))
                    .finish()
            }
        })*
    };
}

lit! {
    LitByte,
    LitByteStr,
    LitChar,
    LitFloat,
    LitInt,
    LitStr
}

impl<T: Debug, P: Debug> Debug for Punctuated<T, P> {
    fn fmt(&self, f: &mut Formatter) {
        let mut l = f.debug_list();
        for i in self.pairs() {
            match i {
                syn::punctuated::Pair::Punctuated(t, p) => {
                    l.entry(&t);
                    l.entry(&p);
                }
                syn::punctuated::Pair::End(p) => {
                    l.entry(&p);
                }
            }
        }

        l.finish()
    }
}

impl Debug for syn::ExprReference {
    fn fmt(&self, f: &mut Formatter) {
        struct Reserved;

        impl Debug for Reserved {
            fn fmt(&self, f: &mut Formatter) {
                f.debug_struct("Reserved").finish()
            }
        }

        f.debug_struct("ExprReference")
            .field("attrs", &self.attrs)
            .field("and_token", &self.and_token)
            .field("raw", &Reserved)
            .field("mutability", &self.mutability)
            .field("expr", &self.expr)
            .finish()
    }
}

macro_rules! token {
    ($($name:ident)*) => {
        $(
            impl Debug for syn :: token :: $name {
                fn fmt(&self, f: &mut Formatter) {
                    f.debug_struct(stringify!($name)).finish()
                }
            }
        )*
    };
}

token! {
    // Keywords
    Abstract
    As
    Async
    Auto
    Await
    Become
    Box
    Break
    Const
    Continue
    Crate
    Default
    Do
    Dyn
    Else
    Enum
    Extern
    Final
    Fn
    For
    If
    Impl
    In
    Let
    Loop
    Macro
    Match
    Mod
    Move
    Mut
    Override
    Priv
    Pub
    Ref
    Return
    SelfType
    SelfValue
    Static
    Struct
    Super
    Trait
    Try
    Type
    Typeof
    Union
    Unsafe
    Unsized
    Use
    Virtual
    Where
    While
    Yield
    // Punctuation
    Add
    AddEq
    And
    AndAnd
    AndEq
    At
    Bang
    Caret
    CaretEq
    Colon
    Colon2
    Comma
    Div
    DivEq
    Dollar
    Dot
    Dot2
    Dot3
    DotDotEq
    Eq
    EqEq
    Ge
    Gt
    Le
    Lt
    MulEq
    Ne
    Or
    OrEq
    OrOr
    Pound
    Question
    RArrow
    LArrow
    Rem
    RemEq
    FatArrow
    Semi
    Shl
    ShlEq
    Shr
    ShrEq
    Star
    Sub
    SubEq
    Tilde
    // Delimeters
    Brace
    Bracket
    Paren
    Group
    // Special for some reason
    Underscore
}
