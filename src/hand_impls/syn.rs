use syn::{punctuated::Punctuated, LitByte, LitByteStr, LitChar, LitFloat, LitInt, LitStr};

use crate::{Debug, Formatter};

macro_rules! lit {
    ($($ty:path),*) => {
        $(impl Debug for $ty {
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
