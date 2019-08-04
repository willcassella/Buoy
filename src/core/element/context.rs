use std::rc::Rc;

use crate::core::element::*;
use crate::core::filter::*;
use crate::state::*;
use crate::space::*;
use crate::util::arena::{Arena, ABox};
use crate::util::queue::QNode;

pub struct Context<'slf, 'frm> {
    pub(in crate::core) max_area: Area,
    pub(in crate::core) children: SocketTree<'frm>,
    pub(in crate::core) filter_stack: FilterStack,

    pub(in crate::core) prev_frame_state: &'slf StateCache,
    pub(in crate::core) global_data: &'slf mut GlobalData,
    pub(in crate::core) buffer: &'frm Arena,
    pub(in crate::core) subctx_stack: &'slf mut SubContextStack<'frm>,
}

impl<'slf, 'frm> Context<'slf, 'frm> {
    pub fn max_area(&self) -> Area {
        self.max_area
    }

    // TODO: It would be nice if I didn't have to expose this
    pub fn buffer(&self) -> &'frm Arena {
        self.buffer
    }

    pub fn open_sub<'a, E: AllocElement<'frm>>(
        &'a mut self,
        max_area: Area,
        id: Id,
        elem: E,
    ) -> SubContext<'a, 'slf, 'frm> {
        let buf = self.buffer;

        // Clear the subcontext stack before using it
        self.subctx_stack.clear();

        SubContext {
            max_area,
            root: ElementNode::new(id, elem.alloc(buf), self.filter_stack.clone()),
            ctx: self,
        }
    }

    pub fn open_socket<S: Socket<'frm>>(&mut self, name: SocketName, max_area: Area, socket: &mut S) {
        let children = match self.children.get(name) {
            Some(children) => children,
            None => return,
        };

        // Fill the socket
        while socket.remaining_capacity() != 0 {
            let mut child = match children.pop_front_node() {
                Some(child) => QNode::into_inner(ABox::into_inner(child)),
                None => break,
            };

            // Run the child
            let sub_ctx = Context {
                max_area,
                children: child.children.take(),
                filter_stack: FilterStack::default(),

                prev_frame_state: self.prev_frame_state,
                global_data: self.global_data,
                buffer: self.buffer,
                subctx_stack: self.subctx_stack,
            };

            socket.push(run_element(sub_ctx, child.id, child.elem, child.filter_stack));
        }
    }

    pub fn new_layout<L: Layout + 'frm>(&self, min_area: Area, layout: L) -> LayoutNode<'frm> {
        LayoutNode {
            min_area,
            layout: self.buffer.alloc(layout).unsize(),
        }
    }

    pub fn new_layout_null(&self) -> LayoutNode<'frm> {
        LayoutNode::null(self.buffer)
    }

    pub fn next_frame_filter(&mut self, filter: Rc<dyn Filter>) {
        self.global_data.next_frame_filters.append(filter);
    }

    pub fn new_state<T: StateT>(&mut self) -> State<T> {
        let id = self.global_data.next_state_id.increment();
        State::new(id)
    }

    pub fn read_state<T: StateT>(&self, state: State<T>) -> T {
        if state.id.frame_id != self.global_data.next_state_id.frame_id.prev() {
            panic!("Attempt to read state from wrong frame");
        }

        if let Some(v) = self.prev_frame_state.get(&state.id) {
            v.downcast_ref::<T>().expect("Mismatched types").clone()
        } else {
            Default::default()
        }
    }
}

pub type SubContextStack<'frm> = Vec<(ElementQNode<'frm>, SocketName)>;

pub struct SubContext<'slf, 'ctx, 'frm> {
    max_area: Area,
    root: ElementNode<'frm>,
    ctx: &'slf mut Context<'ctx, 'frm>,
}

impl<'slf, 'ctx, 'frm> SubContext<'slf, 'ctx, 'frm> {
    pub fn close(mut self) -> LayoutNode<'frm> {
        while !self.ctx.subctx_stack.is_empty() {
            self.end();
        }

        // Create a context for running the element
        let ctx = Context {
            max_area: self.max_area,
            children: self.root.children.take(),
            filter_stack: FilterStack::default(),

            prev_frame_state: self.ctx.prev_frame_state,
            global_data: self.ctx.global_data,
            buffer: self.ctx.buffer,
            subctx_stack: self.ctx.subctx_stack,
        };

        run_element(ctx, self.root.id, self.root.elem, self.root.filter_stack)
    }

    pub fn end(&mut self) -> &mut Self {
        let (node, socket) = self.ctx.subctx_stack.pop().expect("Bad call to 'end'");

        // Get the parent node
        let parent = match self.ctx.subctx_stack.last_mut() {
            Some(parent) => &mut parent.0,
            None => &mut self.root,
        };

        parent.children.get_or_create(socket).push_back_node(node);
        self
    }

    pub fn begin<'a, E: AllocElement<'frm>>(
        &'a mut self,
        socket: SocketName,
        id: Id,
        elem: E,
    ) -> &'a mut Self {
        // TODO(perf) - Could potentially do this as a single allocation
        let node = QNode::new(ElementNode::new(id, elem.alloc(self.ctx.buffer), self.ctx.filter_stack.clone()));
        let node = self.ctx.buffer.alloc(node);

        self.ctx.subctx_stack.push((node, socket));
        self
    }

    pub fn connect_socket<'a>(
        &'a mut self,
        target: SocketName,
        socket: SocketName,
    ) -> &'a mut Self {
        // Get the current children
        let children = match self.ctx.children.remove(socket) {
            Some(children) => children,
            None => return self,
        };

        // Get the parent
        let parent = match self.ctx.subctx_stack.last_mut() {
            Some(parent) => &mut parent.0,
            None => &mut self.root,
        };

        // Insert the children into the parent
        parent.children.get_or_create(target).append(children);
        self
    }

    pub fn connect_all_sockets<'a>(
        &'a mut self,
    ) -> &'a mut Self {
        // Get the current children
        let children = self.ctx.children.take();

        // Get the parent
        let parent = match self.ctx.subctx_stack.last_mut() {
            Some(parent) => &mut parent.0,
            None => &mut self.root,
        };

        parent.children.append(children);

        self
    }

    pub fn new_state<T: StateT>(&mut self) -> State<T> {
        self.ctx.new_state()
    }

    pub fn read_state<T: StateT>(&self, state: State<T>) -> T {
        self.ctx.read_state(state)
    }
}

pub fn run_element<'ctx, 'frm>(
    mut ctx: Context<'ctx, 'frm>,
    id: Id,
    element: ABox<'frm, dyn Element>,
    mut filters: FilterStack,
) -> LayoutNode<'frm> {
    debug_assert!(ctx.filter_stack.is_empty());

    // Run the element against the given filter stack
    let mut sub_filters = FilterStackBuilder::default();

    while let Some(filter) = filters.pop() {
        match filter.predicate(id, &*element) {
            PredicateResult::RunFilter => {
                ctx.filter_stack = sub_filters.append_to(filters);
                return filter.element(ctx, id, element);
            },
            PredicateResult::Pass => {
                // Continue popping filter stack
            },
            PredicateResult::PassRecurse => {
                sub_filters.append(filter);
            }
        }
    }

    ctx.filter_stack = sub_filters.into_stack();
    element.run(ctx, id)
}
