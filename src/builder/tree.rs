use std::collections::VecDeque;

use crate::prelude::*;
use super::builder_context::{Node, NodeKind};

impl TreeProvider for VecDeque<Node> {
    fn socket<'window, 'ctx>(
        &mut self,
        mut ctx: TreeContext<'window, 'ctx>,
        name: SocketName,
    ) -> bool {
        assert_eq!(name, SocketName::default(), "Only default sockets are supported at the moment");

        while ctx.remaining_capacity() != 0 {
            // Get the first element
            let mut node = match self.pop_front() {
                Some(node) => node,
                None => return false,
            };

            let (element, id) = match node.kind {
                NodeKind::Element(element, id) => (element, id),
                NodeKind::Filter(_) => unimplemented!(),
                NodeKind::Socket(_) => unimplemented!(),
            };

            // Run it
            ctx.element(id, &*element, &mut node.children);
        }

        !self.is_empty()
    }
}
