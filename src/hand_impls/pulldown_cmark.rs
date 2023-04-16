use pulldown_cmark::CowStr;

use crate::Debug;

impl Debug for CowStr<'_> {
    fn fmt(&self, f: &mut crate::Formatter) {
        let s: &str = self;
        Debug::fmt(&s, f)
    }
}
