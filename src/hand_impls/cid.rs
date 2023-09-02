use crate::Debug;

impl<const N: usize> Debug for cid::CidGeneric<N> {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_tuple("Cid").field(&self.to_string()).finish()
    }
}
