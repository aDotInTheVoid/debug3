//! Helpers for implementing [`Debug`] on your own types.

use crate::{Debug, Formatter};

pub(crate) mod list;
pub(crate) mod map;
pub(crate) mod named_list;
pub(crate) mod set;
pub(crate) mod strukt;
pub(crate) mod tuple;

pub use list::DebugList;
pub use map::DebugMap;
pub use named_list::DebugNamedList;
pub use set::DebugSet;
pub use strukt::DebugStruct;
pub use tuple::DebugTuple;

struct DebugInner<'a> {
    fmt: &'a mut Formatter,
    has_fields: bool,
}

impl<'a> DebugInner<'a> {
    fn entry(&mut self, entry: &dyn Debug) {
        if self.has_fields {
            self.fmt.trailing_comma(false);
        }

        // TODO: Should this be suroundid with ibox(0) and end, like
        // `DebugStruct::field`?
        entry.fmt(self.fmt);

        self.has_fields = true;
    }
}
