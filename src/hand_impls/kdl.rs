use crate::Debug;
use kdl::{KdlDocument, KdlEntry, KdlIdentifier, KdlNode, KdlValue};

impl Debug for KdlDocument {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_list().entries(self.nodes()).finish()
    }
}

fn field_if(f: &mut crate::builders::DebugStruct, name: &str, value: Option<impl Debug>) {
    if let Some(v) = value {
        f.field(name, &v);
    }
}
fn field_slice(f: &mut crate::builders::DebugStruct, name: &str, slice: &[impl Debug]) {
    match slice {
        [] => {}
        [v] => {
            f.field(name, v);
        }
        vs => {
            f.field(name, &vs);
        }
    }
}

impl Debug for KdlEntry {
    fn fmt(&self, f: &mut crate::Formatter) {
        if self.ty().is_none() && self.name().is_none() {
            Debug::fmt(self.value(), f)
        } else {
            let mut d = f.debug_struct("KdlEntry");
            field_if(&mut d, "ty", self.ty());
            field_if(&mut d, "name", self.name());
            d.field("value", self.value());
            d.finish()
        }
    }
}
impl Debug for KdlIdentifier {
    fn fmt(&self, f: &mut crate::Formatter) {
        Debug::fmt(&self.value(), f)
    }
}
impl Debug for KdlNode {
    fn fmt(&self, f: &mut crate::Formatter) {
        let mut d = f.debug_struct("KdlNode");
        field_if(&mut d, "ty", self.ty());
        d.field("name", &self.name());
        field_slice(&mut d, "entries", self.entries());
        field_if(&mut d, "children", self.children());
        d.finish()
    }
}
impl Debug for KdlValue {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            Self::RawString(arg0) => Debug::fmt(arg0, f),
            Self::String(arg0) => Debug::fmt(arg0, f),
            Self::Base2(arg0) => Debug::fmt(arg0, f),
            Self::Base8(arg0) => Debug::fmt(arg0, f),
            Self::Base10(arg0) => Debug::fmt(arg0, f),
            Self::Base10Float(arg0) => Debug::fmt(arg0, f),
            Self::Base16(arg0) => Debug::fmt(arg0, f),
            Self::Bool(arg0) => Debug::fmt(arg0, f),
            Self::Null => f.debug_struct("Null").finish(),
        }
    }
}
