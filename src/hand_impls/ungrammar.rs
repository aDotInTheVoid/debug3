use ungrammar::{NodeData, TokenData};

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
        f.debug_struct("Grammar")
            .field("nodes", &Nodes(self))
            .field("tokens", &Tokens(self))
            .finish()
    }
}

struct RuleRef<'a>(&'a ungrammar::Grammar, &'a ungrammar::Rule);
impl crate::Debug for RuleRef<'_> {
    fn fmt(&self, f: &mut crate::Formatter) {
        let w = |r| RuleRef(self.0, r);
        let ws = |rs| RulesRef(self.0, rs);

        match self.1 {
            ungrammar::Rule::Labeled { label, rule } => f
                .debug_struct("Labeled")
                .field("label", label)
                .field("rule", &w(rule))
                .finish(),
            ungrammar::Rule::Node(node) => {
                f.debug_tuple("Node").field(&self.0[*node].name).finish();
            }
            ungrammar::Rule::Token(token) => {
                f.debug_tuple("Token").field(&self.0[*token].name).finish();
            }
            ungrammar::Rule::Seq(rules) => {
                f.debug_tuple("Seq").field(&ws(rules)).finish();
            }
            ungrammar::Rule::Alt(rules) => {
                f.debug_tuple("Alt").field(&ws(rules)).finish();
            }
            ungrammar::Rule::Opt(rule) => {
                f.debug_tuple("Opt").field(&w(rule)).finish();
            }
            ungrammar::Rule::Rep(rule) => {
                f.debug_tuple("Rep").field(&w(rule)).finish();
            }
        }
    }
}

struct RulesRef<'a>(&'a ungrammar::Grammar, &'a [ungrammar::Rule]);
impl crate::Debug for RulesRef<'_> {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_list()
            .entries(self.1.iter().map(|r| RuleRef(self.0, r)))
            .finish()
    }
}

struct Nodes<'a>(&'a ungrammar::Grammar);
impl crate::Debug for Nodes<'_> {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_map()
            .entries(self.0.iter().map(|n| {
                let NodeData { name, rule } = &self.0[n];
                (name, RuleRef(self.0, rule))
            }))
            .finish()
    }
}

struct Tokens<'a>(&'a ungrammar::Grammar);
impl crate::Debug for Tokens<'_> {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_set()
            .entries(self.0.tokens().map(|t| {
                let TokenData { name } = &self.0[t];
                name
            }))
            .finish()
    }
}
