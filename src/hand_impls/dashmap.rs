use std::hash::{BuildHasher, Hash};

use dashmap::{
    mapref::entry::{OccupiedEntry, VacantEntry},
    DashMap, DashSet,
};

use crate::{Debug, Formatter};

impl<K, V, S> Debug for DashMap<K, V, S>
where
    K: Debug + Eq + Hash,
    V: Debug,
    S: Clone + BuildHasher,
{
    fn fmt(&self, f: &mut Formatter) {
        let mut m = f.debug_map();
        for entry in self.iter() {
            let (k, v) = entry.pair();
            m.entry(k, v);
        }
        m.finish()
    }
}

impl<T, S> Debug for DashSet<T, S>
where
    T: Eq + Hash + Debug,
    S: Clone + BuildHasher,
{
    fn fmt(&self, f: &mut Formatter) {
        let mut s = f.debug_set();
        for x in self.iter() {
            s.entry(x.key());
        }
        s.finish()
    }
}

impl<K, V, S> Debug for OccupiedEntry<'_, K, V, S>
where
    K: Debug + Hash + Eq,
    V: Debug,
    S: BuildHasher + Clone,
{
    fn fmt(&self, f: &mut Formatter) {
        f.debug_struct("OccupiedEntry")
            .field("key", self.key())
            .field("value", self.get())
            .finish()
    }
}

impl<K, V, S> Debug for VacantEntry<'_, K, V, S>
where
    K: Debug + Hash + Eq,
    S: BuildHasher + Clone,
{
    fn fmt(&self, f: &mut Formatter) {
        f.debug_tuple("VacantEntry").field(self.key()).finish()
    }
}
