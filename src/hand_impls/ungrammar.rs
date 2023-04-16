impl crate::Debug for ungrammar::Node {
    fn fmt(&self, f: &mut crate::Formatter) {
        // This is a newtyped usize, so this is fine (but I'm not happy about it).
        f.write_debug(self)
    }
}
impl crate::Debug for ungrammar::Token {
    fn fmt(&self, f: &mut crate::Formatter) {
        // This is a newtyped usize, so this is fine (but I'm not happy about it).
        f.write_debug(self)
    }
}

impl crate::Debug for ungrammar::Grammar {
    fn fmt(&self, f: &mut crate::Formatter) {
        struct Nodes<'a>(&'a ungrammar::Grammar);
        impl crate::Debug for Nodes<'_> {
            fn fmt(&self, f: &mut crate::Formatter) {
                f.debug_list()
                    .entries(self.0.iter().map(|n| &self.0[n]))
                    .finish()
            }
        }

        struct Tokens<'a>(&'a ungrammar::Grammar);
        impl crate::Debug for Tokens<'_> {
            fn fmt(&self, f: &mut crate::Formatter) {
                f.debug_list()
                    .entries(self.0.tokens().map(|t| &self.0[t]))
                    .finish()
            }
        }

        f.debug_struct("Grammar")
            .field("nodes", &Nodes(&self))
            .field("tokens", &Tokens(&self))
            .finish()
    }
}
