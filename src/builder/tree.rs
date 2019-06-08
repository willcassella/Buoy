use std::collections::VecDeque;

use crate::prelude::*;
use super::builder_context::{Node, NodeKind};

impl TreeProvider for VecDeque<Node> {
    fn socket<'a, C: TreeContext<'a>>(
        &mut self,
        mut ctx: C,
        name: SocketName,
    ) -> bool {
        assert_eq!(name, SocketName::default(), "Only default sockets are supported at the moment");

        while ctx.remaining_capacity() != 0 {
            // Get the first element
            let node = match self.pop_front() {
                Some(node) => node,
                None => return false,
            };

            let (element, id) = match node.kind {
                NodeKind::Element(element, id) => (element, id),
                NodeKind::Filter(_) => unimplemented!(),
                NodeKind::Socket(_) => unimplemented!(),
            };

            // Run it
            ctx.element(id, element, node.children);
        }

        !self.is_empty()
    }
}
