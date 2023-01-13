use tree_sitter::{Node, Tree};

use crate::Debug;

impl Debug for Tree {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_tuple("Tree").field(&self.root_node()).finish()
    }
}

impl Debug for Node<'_> {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Node")
            .field("kind", &self.kind())
            .field("children", &Children(*self))
            .finish()
    }
}

struct Children<'a>(Node<'a>);

impl Debug for Children<'_> {
    fn fmt(&self, f: &mut crate::Formatter) {
        let mut walk = self.0.walk(); // This isn't very efficient, but eh
        f.debug_list().entries(self.0.children(&mut walk)).finish()
    }
}
