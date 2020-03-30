use crate::core::element::*;
use crate::core::filter::*;
use crate::message::*;
use crate::space::*;
use crate::util::arena::{Arena, ABox};
use crate::util::queue::QNode;

pub struct Context<'slf, 'frm> {
    pub(in crate::core) max_area: Area,
    pub(in crate::core) children: SocketTree<'frm>,
    pub(in crate::core) filter_stack: FilterStack,

    pub(in crate::core) incoming_messages: &'frm MessageMap,
    pub(in crate::core) outgoing_messages: &'slf mut MessageMap,

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
        elem: E,
    ) -> SubContext<'a, 'slf, 'frm> {
        let buf = self.buffer;

        // Clear the subcontext stack before using it
        self.subctx_stack.clear();

        SubContext {
            max_area,
            root: ElementNode::new(elem.alloc(buf), self.filter_stack.clone()),
            ctx: self,
        }
    }

    pub fn open_socket<S: Socket<'frm>>(
        &mut self,
        name: SocketName,
        max_area: Area,
        socket: &mut S
    ) {
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

                incoming_messages: self.incoming_messages,
                outgoing_messages: self.outgoing_messages,

                buffer: self.buffer,
                subctx_stack: self.subctx_stack,
            };

            socket.push(run_element(sub_ctx, child.elem));
        }
    }

    pub fn new_layout<L: Layout + 'frm>(&self, min_area: Area, layout: L) -> LayoutNode<'frm> {
        LayoutNode {
            min_area,
            layout: self.buffer.alloc(layout).upcast(),
        }
    }

    pub fn new_layout_null(&self) -> LayoutNode<'frm> {
        LayoutNode::null(self.buffer)
    }

    pub fn message<T: Message>(&mut self, id: Id) -> (Inbox<T>, Outbox<T>) {
        (Inbox::new(id), Outbox::new(id))
    }

    pub fn read_message<T: Message>(&self, inbox: Inbox<T>) -> Option<T> {
        let value = match self.incoming_messages.get(&inbox.id()) {
            Some(value) => &**value,
            None => return None,
        };

        value.downcast_ref::<T>().cloned()
    }

    pub fn write_message<T: Message>(&mut self, outbox: Outbox<T>, value: T) {
        self.outgoing_messages.insert(outbox.id(), Box::new(value));
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

            incoming_messages: self.ctx.incoming_messages,
            outgoing_messages: self.ctx.outgoing_messages,

            buffer: self.ctx.buffer,
            subctx_stack: self.ctx.subctx_stack,
        };

        run_element(ctx, self.root.elem)
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
        elem: E,
    ) -> &'a mut Self {
        // TODO(perf) - Could potentially do this as a single allocation
        let node = QNode::new(ElementNode::new(elem.alloc(self.ctx.buffer), self.ctx.filter_stack.clone()));
        let node = self.ctx.buffer.alloc(node);

        self.ctx.subctx_stack.push((node, socket));
        self
    }

    pub fn connect_socket(
        &mut self,
        target: SocketName,
        socket: SocketName,
    ) -> &mut Self {
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

    pub fn connect_all_sockets(
        &mut self,
    ) -> &mut Self {
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

    pub fn message<T: Message>(&mut self, id: Id) -> (Inbox<T>, Outbox<T>) {
        self.ctx.message(id)
    }

    pub fn read_message<T: Message>(&self, inbox: Inbox<T>) -> Option<T> {
        self.ctx.read_message(inbox)
    }

    pub fn write_message<T: Message>(&mut self, outbox: Outbox<T>, value: T) {
        self.ctx.write_message(outbox, value)
    }
}

pub fn run_element<'ctx, 'frm>(
    mut ctx: Context<'ctx, 'frm>,
    elem: Elem<'frm>,
    // mut super_filters: FilterStack,
    // mut local_filters: FilterStackMut,
) -> LayoutNode<'frm> {
    debug_assert!(ctx.filter_stack.is_empty());

    // Run the element against the given filter stack
    let sub_filters = FilterStack::default();

    // while let Some(filter) = filters.pop() {
    //     match filter.predicate(elem.data.into_any(), elem.id) {
    //         PredicateResult::RunFilter => {
    //             ctx.filter_stack = sub_filters.append_to(filters);
    //             return filter.run(elem, ctx);
    //         },
    //         PredicateResult::Pass => {
    //             // Continue popping filter stack
    //         },
    //         PredicateResult::PassRecurse => {
    //             sub_filters.append(filter);
    //         }
    //     }
    // }

    ctx.filter_stack = sub_filters;
    elem.run(ctx)
}
