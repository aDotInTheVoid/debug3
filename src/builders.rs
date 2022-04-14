#![allow(unused_imports)]

use crate::{Debug, Formatter, Write};

pub(crate) mod list;
pub(crate) mod map;
pub(crate) mod pad;
pub(crate) mod set;
pub(crate) mod strukt;
pub(crate) mod tuple;

pub use list::DebugList;
pub use map::DebugMap;
pub use set::DebugSet;
pub use strukt::DebugStruct;
pub use tuple::DebugTuple;

struct DebugInner<'a, 'b: 'a> {
    fmt: &'a mut Formatter<'b>,
    has_fields: bool,
}

impl<'a, 'b: 'a> DebugInner<'a, 'b> {
    fn entry(&mut self, entry: &dyn Debug) {
        if self.is_pretty() {
            if !self.has_fields {
                self.fmt.write_str("\n");
            }
            let mut slot = None;
            let mut state = Default::default();
            let mut writer = pad::PadAdapter::wrap(self.fmt, &mut slot, &mut state);
            entry.fmt(&mut writer);
            writer.write_str(",\n")
        } else {
            if self.has_fields {
                self.fmt.write_str(", ");
            }
            entry.fmt(self.fmt)
        }

        self.has_fields = true;
    }

    fn is_pretty(&self) -> bool {
        self.fmt.alternate()
    }
}
