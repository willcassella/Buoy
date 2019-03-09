use std::collections::VecDeque;

use crate::core::*;
use super::context::{Node, NodeKind};

pub struct BuilderTree {
    next: VecDeque<Node>,
    previous: Vec<VecDeque<Node>>,
}

impl BuilderTree {
    pub fn new(roots: VecDeque<Node>) -> Self {
        BuilderTree {
            next: roots,
            previous: Vec::new(),
        }
    }
}

impl context::TreeProvider for BuilderTree {
    fn pop(
        &mut self,
        socket: socket::Id,
    ) -> Option<context::TreeNode<Box<dyn DynElement>>> {
        assert_eq!(socket, socket::Id::default(), "Only default sockets are supported at the moment");

        self.next.pop_front().map(|node| {
            let prev = std::mem::replace(&mut self.next, node.children);
            self.previous.push(prev);

            match node.kind {
                NodeKind::Element(element, id) => context::TreeNode{ element, id },
                NodeKind::Filter(_) => unimplemented!(),
                NodeKind::Socket(_) => unimplemented!(),
            }
        })
    }

    fn push_some(
        &mut self,
        socket: socket::Id,
        node: context::TreeNode<Box<dyn DynElement>>,
    ) {
        assert_eq!(socket, socket::Id::default(), "Only default sockets are supported at the moment");

        let prev = self.previous.pop().expect("Bad call to push");
        let node = Node {
            kind: NodeKind::Element(node.element, node.id),
            children: std::mem::replace(&mut self.next, prev),
        };
        self.next.push_front(node);
    }

    fn push_none(
        &mut self,
    ) {
        let prev = self.previous.pop().expect("Bad call to push");
        std::mem::replace(&mut self.next, prev);
    }
}
