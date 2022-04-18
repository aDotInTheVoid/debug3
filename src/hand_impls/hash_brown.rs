use std::borrow::Borrow;

use crate::{Debug, Formatter};
use hashbrown::{hash_map::{
    OccupiedEntry, OccupiedEntryRef, RawOccupiedEntryMut, RawVacantEntryMut, VacantEntry,
    VacantEntryRef,
}, HashMap, HashSet};

// TODO: Be generic over allocator

impl<K: Debug, V: Debug, S> Debug for OccupiedEntry<'_, K, V, S> {
    fn fmt(&self, f: &mut Formatter) {
        f.debug_struct("OccupiedEntry")
            .field("key", self.key())
            .field("value", self.get())
            .finish()
    }
}

impl<K: Debug, V, S> Debug for VacantEntry<'_, K, V, S> {
    fn fmt(&self, f: &mut Formatter) {
        f.debug_tuple("VacantEntry").field(self.key()).finish()
    }
}

impl<K: Borrow<Q>, Q: ?Sized + Debug, V: Debug, S> Debug for OccupiedEntryRef<'_, '_, K, Q, V, S> {
    fn fmt(&self, f: &mut Formatter) {
        f.debug_struct("OccupiedEntryRef")
            .field("key", &self.key())
            .field("value", &self.get())
            .finish()
    }
}

impl<K: Borrow<Q>, Q: ?Sized + Debug, V, S> Debug for VacantEntryRef<'_, '_, K, Q, V, S> {
    fn fmt(&self, f: &mut Formatter) {
        f.debug_tuple("VacantEntryRef").field(&self.key()).finish()
    }
}

impl<K: Debug, V: Debug, S> Debug for RawOccupiedEntryMut<'_, K, V, S> {
    fn fmt(&self, f: &mut Formatter) {
        f.debug_struct("RawOccupiedEntryMut")
            .field("key", self.key())
            .field("value", self.get())
            .finish()
    }
}

impl<K, V, S> Debug for RawVacantEntryMut<'_, K, V, S> {
    fn fmt(&self, f: &mut Formatter) {
        f.debug_struct("RawVacantEntryMut").finish()
    }
}


impl<K: Debug, V: Debug, S> Debug for HashMap<K, V, S> {
    fn fmt(&self, f: &mut Formatter) {
        f.debug_map().entries(self.iter()).finish()
    }
} 

impl<T: Debug, S> Debug for HashSet<T, S> {
    fn fmt(&self, f: &mut Formatter) {
        f.debug_set().entries(self.iter()).finish()
    }
}