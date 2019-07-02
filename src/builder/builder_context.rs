use std::rc::Rc;
use std::collections::VecDeque;

use crate::prelude::*;

pub(crate) enum NodeKind {
    Element(Box<dyn Element>, Id),
    Filter(Rc<dyn Filter>),
    Socket(SocketName),
}

pub(crate) struct Node {
    pub kind: NodeKind,
    pub children: VecDeque<Node>,
}

pub struct BuilderContext<'a, 'window, 'ctx> {
    pub(crate) ctx: &'a mut Context<'window, 'ctx>,
    pub(crate) children: VecDeque<Node>,
    pub(crate) stack: Vec<Node>,
    pub(crate) root: Option<Node>,
}

impl<'a, 'window, 'ctx> BuilderContext<'a, 'window, 'ctx> {
    pub(crate) fn new(
        ctx: &'a mut Context<'window, 'ctx>,
    ) -> Self {
        BuilderContext{
            ctx,
            children: VecDeque::new(),
            stack: Vec::new(),
            root: None,
        }
    }

    pub(crate) fn get_root(
        mut self,
    ) -> Option<Node> {
        // Empty the stack
        while !self.stack.is_empty() {
            self.end();
        }

        self.root
    }

    pub fn element_id(&self) -> Id {
        self.ctx.element_id()
    }

    pub fn max_area(&self) -> Area {
        self.ctx.max_area()
    }

    pub fn element_begin<E: Element>(
        &mut self,
        element: E,
        id: Id,
    ) {
        let kind = NodeKind::Element(Box::new(element), id);
        self.node_begin(kind);
    }

    pub fn filter_begin<F: Filter + 'static>(
        &mut self,
        filter: F,
    ) {
        let kind = NodeKind::Filter(Rc::new(filter));
        self.node_begin(kind);
    }

    fn node_begin(
        &mut self,
        kind: NodeKind,
    ) {
        // There can only be one root node
        assert!(self.root.is_none());

        // Store the current set of children as the new node's children for now,
        // even though they're actually its siiblings
        let node = Node {
            kind: kind,
            children: std::mem::replace(&mut self.children, VecDeque::new()),
        };

        self.stack.push(node);
    }

    pub fn end(
        &mut self,
    ) {
        let mut node = self.stack.pop().expect("Call to 'end' beyond last element");

        // Current children are the node's children
        std::mem::swap(&mut self.children, &mut node.children);

        // If there's a parent element, add this as a child
        if !self.stack.is_empty() {
            self.children.push_back(node);
        } else {
            self.root = Some(node);
        }
    }

    pub fn filter_next_frame(
        &mut self,
        filter: Rc<dyn Filter>,
    ) {
        self.ctx.filter_next_frame(filter)
    }

    pub fn filter_late_next_frame(
        &mut self,
        filter: Rc<dyn Filter>,
    ) {
        self.ctx.filter_late_next_frame(filter)
    }

    pub fn new_input<F: InputState>(&mut self) -> Input<F> {
        self.ctx.new_input()
    }

    pub fn read_input<F: InputState>(&self, input: Input<F>) -> F {
        self.ctx.read_input(input)
    }
}
