use tree_sitter::{Node, Tree};

use crate::Debug;

impl Debug for Tree {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_tuple("Tree").field(&self.root_node()).finish()
    }
}

impl Debug for Node<'_> {
    fn fmt(&self, f: &mut crate::Formatter) {
        // d.field("kind", &self.kind());

        match self.child_count() {
            0 => f.debug_tuple("Node").field(&self.kind()).finish(),
            1 => f
                .debug_struct("Node")
                .field("kind", &self.kind())
                .field("child", &self.child(0).unwrap())
                .finish(),
            _ => f
                .debug_struct("Node")
                .field("kind", &self.kind())
                .field("children", &Children(*self))
                .finish(),
        }
    }
}

struct Children<'a>(Node<'a>);

impl Debug for Children<'_> {
    fn fmt(&self, f: &mut crate::Formatter) {
        let mut walk = self.0.walk(); // This isn't very efficient, but eh
        f.debug_list().entries(self.0.children(&mut walk)).finish()
    }
}
