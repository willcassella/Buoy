use std::collections::VecDeque;

use crate::core::*;
use super::context::{Node, NodeKind};

impl context::TreeProvider for VecDeque<Node> {
    fn socket(
        &mut self,
        socket: socket::Id,
        listener: &mut context::TreeListener,
    ) -> bool {
        assert_eq!(socket, socket::Id::default(), "Only default sockets are supported at the moment");

        loop {
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
            let resume = listener.element(id, element, &mut node.children);

            // If it needs to resume, push that and quit
            // TODO: I think ultimately the context should handle this
            match resume {
                Some(resume) => {
                    self.push_front(Node {
                        kind: NodeKind::Element(resume, id),
                        children: node.children,
                    });
                    return true
                },
                None => (),
            }
        }

        false
    }
}
