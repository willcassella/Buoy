use std::rc::Rc;
use std::collections::VecDeque;

use crate::prelude::*;

pub(crate) enum NodeKind {
    Element(Box<dyn DynElement>, Id),
    Filter(Rc<dyn DynFilter>),
    Socket(SocketName),
}

pub(crate) struct Node {
    pub kind: NodeKind,
    pub children: VecDeque<Node>,
}

pub struct BuilderContext<'a, 'ctx> {
    ctx: &'a mut Context<'ctx>,
    roots: VecDeque<Node>,
    stack: Vec<Node>,
}

impl<'a, 'ctx> BuilderContext<'a, 'ctx> {
    pub(crate) fn new(
        ctx: &'a mut Context<'ctx>,
    ) -> Self {
        BuilderContext{
            ctx,
            roots: VecDeque::new(),
            stack: Vec::new(),
        }
    }

    pub(crate) fn into_tree(
        mut self,
    ) -> VecDeque<Node> {
        // Empty the stack
        while !self.stack.is_empty() {
            self.end();
        }

        self.roots
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
        // Create a new node for this element
        // Back the current root set up as its children
        let node = Node {
            kind: NodeKind::Element(element.upcast(), id),
            children: std::mem::replace(&mut self.roots, VecDeque::new()),
        };

        self.stack.push(node);
    }

    pub fn filter_begin<F: Filter + 'static>(
        &mut self,
        filter: F,
    ) {
        // Create a new node for this element
        // Back the current root set up as its children
        let node = Node {
            kind: NodeKind::Filter(Rc::new(filter)),
            children: std::mem::replace(&mut self.roots, VecDeque::new()),
        };

        self.stack.push(node);
    }

    pub fn end(
        &mut self,
    ) {
        let mut node = self.stack.pop().expect("Call to 'end' beyond last element");

        // Current roots are the node's children
        std::mem::swap(&mut self.roots, &mut node.children);

        // Node is now a root
        self.roots.push_back(node);
    }

    pub fn filter_next_frame(
        &mut self,
        filter: Rc<dyn DynFilter>,
    ) {
        self.ctx.filter_next_frame(filter)
    }

    pub fn filter_late_next_frame(
        &mut self,
        filter: Rc<dyn DynFilter>,
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
