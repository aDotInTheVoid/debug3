use crate::algorithm::{BeginToken, BreakToken, Breaks, Formatter};
use std::borrow::Cow;

impl Formatter {
    pub(crate) fn ibox(&mut self, indent: isize) {
        self.scan_begin(BeginToken {
            offset: indent,
            breaks: Breaks::Inconsistent,
        });
    }

    pub(crate) fn cbox(&mut self, indent: isize) {
        self.scan_begin(BeginToken {
            offset: indent,
            breaks: Breaks::Consistent,
        });
    }

    pub(crate) fn end(&mut self) {
        self.scan_end();
    }

    pub(crate) fn word<S: Into<Cow<'static, str>>>(&mut self, wrd: S) {
        let s = wrd.into();
        self.scan_string(s);
    }

    pub(crate) fn word_s(&mut self, wrd: &str) {
        self.word(wrd.to_owned());
    }

    fn spaces(&mut self, n: usize) {
        self.scan_break(BreakToken {
            blank_space: n,
            ..BreakToken::default()
        });
    }

    pub(crate) fn zerobreak(&mut self) {
        self.spaces(0);
    }

    pub(crate) fn space(&mut self) {
        self.spaces(1);
    }

    // pub(crate) fn nbsp(&mut self) {
    //     self.word(" ");
    // }

    // pub(crate) fn hardbreak(&mut self) {
    //     self.spaces(algorithm::SIZE_INFINITY as usize);
    // }

    pub(crate) fn space_if_nonempty(&mut self) {
        self.scan_break(BreakToken {
            blank_space: 1,
            if_nonempty: true,
            ..BreakToken::default()
        });
    }

    // pub(crate) fn hardbreak_if_nonempty(&mut self) {
    //     self.scan_break(BreakToken {
    //         blank_space: algorithm::SIZE_INFINITY as usize,
    //         if_nonempty: true,
    //         ..BreakToken::default()
    //     });
    // }

    pub(crate) fn trailing_comma(&mut self, is_last: bool) {
        if is_last {
            self.scan_break(BreakToken {
                pre_break: Some(','),
                ..BreakToken::default()
            });
        } else {
            self.word(",");
            self.space();
        }
    }

    pub(crate) fn trailing_comma_or_space(&mut self, is_last: bool) {
        if is_last {
            self.scan_break(BreakToken {
                blank_space: 1,
                pre_break: Some(','),
                ..BreakToken::default()
            });
        } else {
            self.word(",");
            self.space();
        }
    }

    // pub(crate) fn neverbreak(&mut self) {
    //     self.scan_break(BreakToken {
    //         never_break: true,
    //         ..BreakToken::default()
    //     });
    // }
}
