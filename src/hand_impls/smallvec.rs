use crate::{Debug, Formatter};
use smallvec::{Array, SmallVec};

impl<A: Array> Debug for SmallVec<A>
where
    A::Item: Debug,
{
    fn fmt(&self, f: &mut Formatter) {
        f.debug_list().entries(self.iter()).finish()
    }
}
