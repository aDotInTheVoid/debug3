// This is an enourmous pain, because everything in proc_macro2 is
// behind 3 layers of #[cfgs] and indirections, and has like 20 different
// feature flags

use proc_macro2::TokenTree;

impl crate::Debug for proc_macro2::Span {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Span")
            .field("start", &self.start())
            .field("end", &self.end())
            .finish();
    }
}

impl crate::Debug for proc_macro2::TokenStream {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_named_list("TokenStream")
            .entries(self.clone())
            .finish()
    }
}

impl crate::Debug for proc_macro2::Ident {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Ident")
            .field("sym", &self.to_string())
            .field("span", &self.span())
            .finish();
    }
}

impl crate::Debug for proc_macro2::TokenTree {
    fn fmt(&self, f: &mut crate::Formatter) {
        // Each of these has the name in the struct type in the derived debug,
        // so don't bother with an extra layer of indirection
        match self {
            TokenTree::Group(t) => t.fmt(f),
            TokenTree::Ident(t) => t.fmt(f),
            TokenTree::Punct(t) => t.fmt(f),
            TokenTree::Literal(t) => t.fmt(f),
        }
    }
}

/*
impl Debug for Literal {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut debug = fmt.debug_struct("Literal");
        debug.field("lit", &format_args!("{}", self.repr));
        debug_span_field_if_nontrivial(&mut debug, self.span);
        debug.finish()
    }
} */

impl crate::Debug for proc_macro2::Literal {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Literal")
            .field("lit", &format_args!("{}", self))
            .field("span", &self.span())
            .finish()
    }
}

impl crate::Debug for proc_macro2::Punct {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Punct")
            .field("char", &self.as_char())
            .field("spacing", &self.spacing())
            .field("span", &self.spacing())
            .finish()
    }
}

impl crate::Debug for proc_macro2::Group {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Group")
            .field("delimiter", &self.delimiter())
            .field("stream", &self.stream())
            .field("span", &self.span())
            .finish();
    }
}
