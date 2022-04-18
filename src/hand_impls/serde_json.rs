use serde_json::{
    map::{OccupiedEntry, VacantEntry},
    Map, Number, Value,
};

use crate::{Debug, Formatter};

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter) {
        match self {
            Value::Null => f.debug_tuple("Null").finish(),
            Value::Bool(v) => f.debug_tuple("Bool").field(&v).finish(),
            Value::Number(v) => v.fmt(f),
            Value::String(v) => f.debug_tuple("String").field(v).finish(),
            Value::Array(v) => f.debug_named_list("Array").entries(v).finish(),
            Value::Object(v) => v.fmt(f),
        }
    }
}

impl Debug for Map<String, Value> {
    fn fmt(&self, f: &mut Formatter) {
        let mut m = f.debug_struct("Object");
        for (k, v) in self {
            m.field(k, v);
        }
        m.finish()
    }
}

impl Debug for Number {
    fn fmt(&self, f: &mut Formatter) {
        let mut n = f.debug_tuple("Number");
        if let Some(i) = self.as_i64() {
            n.field(&i);
        } else if let Some(i) = self.as_u64() {
            n.field(&i);
        } else if let Some(f) = self.as_f64() {
            n.field(&f);
        } else {
            struct UnsupportedNumber;
            impl Debug for UnsupportedNumber {
                fn fmt(&self, f: &mut Formatter) {
                    f.word("UnsupportedNumber");
                }
            }
            n.field(&UnsupportedNumber);
        }
        n.finish()
    }
}

impl Debug for VacantEntry<'_> {
    fn fmt(&self, f: &mut Formatter) {
        f.debug_tuple("VacantEntry").field(&self.key()).finish()
    }
}

impl Debug for OccupiedEntry<'_> {
    fn fmt(&self, f: &mut Formatter) {
        f.debug_struct("OccupiedEntry")
            .field("key", self.key())
            .field("value", self.get())
            .finish()
    }
}
