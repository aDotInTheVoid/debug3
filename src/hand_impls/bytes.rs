use crate::{Debug, Formatter};

// bytes Debug impl writes something nice like b"abc"

impl Debug for bytes::Bytes {
    fn fmt(&self, f: &mut Formatter) {
        f.write_debug(self);
    }
}

impl Debug for bytes::BytesMut {
    fn fmt(&self, f: &mut Formatter) {
        f.write_debug(self);
    }
}