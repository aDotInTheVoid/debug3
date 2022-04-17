// This is an enourmous pain, because everything in proc_macro2 is
// behind 3 layers of #[cfgs] and indirections, and has like 20 different
// feature flags

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
        // TODO
    }
}

impl crate::Debug for proc_macro2::Ident {
    fn fmt(&self, f: &mut crate::Formatter) {
        // TODO
    }
}

impl crate::Debug for proc_macro2::Literal {
    fn fmt(&self, f: &mut crate::Formatter) {
        // TODO
    }
}

impl crate::Debug for proc_macro2::Punct {
    fn fmt(&self, f: &mut crate::Formatter) {
        // TODO
    }
}

impl crate::Debug for proc_macro2::Group {
    fn fmt(&self, f: &mut crate::Formatter) {
        // TODO
    }
}