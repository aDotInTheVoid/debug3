use crate::Debug;
use miette::SourceSpan;

impl Debug for SourceSpan {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("SourceSpan")
            .field("offset", &self.offset())
            .field("length", &self.len())
            .finish()
    }
}
