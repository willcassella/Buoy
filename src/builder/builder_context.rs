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

pub trait BuilderContext: Sized {
    fn element_id(
        &self,
    ) -> Id;

    fn max_area(
        &self,
    ) -> Area;

    fn element_begin<E: Element>(
        &mut self,
        element: E,
        id: Id,
    );

    fn filter_begin<F: Filter + 'static>(
        &mut self,
        filter: F,
    );

    fn end(
        &mut self,
    );

    fn filter_next_frame(
        &mut self,
        filter: Rc<dyn DynFilter>,
    );

    fn filter_late_next_frame(
        &mut self,
        filter: Rc<dyn DynFilter>,
    );

    fn new_input<F: InputState>(
        &mut self
    ) -> Input<F>;

    fn read_input<F: InputState>(
        &self,
        input: Input<F>
    ) -> F;
}

pub struct BuilderContextImpl<'a, C> {
    pub(crate) ctx: &'a mut C,
    pub(crate) roots: VecDeque<Node>,
    pub(crate) stack: Vec<Node>,
}

impl<'a, 'b, C: Context<'b>> BuilderContextImpl<'a, C> {
    pub(crate) fn new(
        ctx: &'a mut C
    ) -> Self {
        BuilderContextImpl{
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
}

impl<'a, 'b, C: Context<'b>> BuilderContext for BuilderContextImpl<'a, C> {
    fn element_id(&self) -> Id {
        self.ctx.element_id()
    }

    fn max_area(&self) -> Area {
        self.ctx.max_area()
    }

    fn element_begin<E: Element>(
        &mut self,
        element: E,
        id: Id,
    ) {
        // Create a new node for this element
        // Back the current root set up as its children
        let node = Node {
            kind: NodeKind::Element(element.upcast_box(), id),
            children: std::mem::replace(&mut self.roots, VecDeque::new()),
        };

        self.stack.push(node);
    }

    fn filter_begin<F: Filter + 'static>(
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

    fn end(
        &mut self,
    ) {
        let mut node = self.stack.pop().expect("Call to 'end' beyond last element");

        // Current roots are the node's children
        std::mem::swap(&mut self.roots, &mut node.children);

        // Node is now a root
        self.roots.push_back(node);
    }

    fn filter_next_frame(
        &mut self,
        filter: Rc<dyn DynFilter>,
    ) {
        self.ctx.filter_next_frame(filter)
    }

    fn filter_late_next_frame(
        &mut self,
        filter: Rc<dyn DynFilter>,
    ) {
        self.ctx.filter_late_next_frame(filter)
    }

    fn new_input<F: InputState>(&mut self) -> Input<F> {
        self.ctx.new_input()
    }

    fn read_input<F: InputState>(&self, input: Input<F>) -> F {
        self.ctx.read_input(input)
    }
}
