use std::collections::HashMap;

use crate::core::element::*;
use crate::core::filter::*;
use crate::state::*;
use crate::space::*;
use crate::util::linked_buffer::{LinkedBuffer, LinkedBufferBox};
use crate::util::linked_queue::{QueueBuilder};

pub struct Context<'slf, 'win> {
    pub(crate) max_area: Area,
    pub(crate) children: Children<'win>,

    pub(crate) prev_frame_state: &'slf StateCache,
    pub(crate) global_data: &'slf mut GlobalData,
    pub(crate) buffer: &'win LinkedBuffer,
    pub(crate) subctx_stack: &'slf mut SubContextStack<'win>,
}

pub(crate) struct Node<'win> {
    id: Id,
    elem: LinkedBufferBox<'win, dyn Element>,
    children: Children<'win>,
}

#[derive(Default)]
pub(crate) struct Children<'win> {
    default_socket: QueueBuilder<'win, Node<'win>>,
    other_sockets: Option<HashMap<SocketName, QueueBuilder<'win, Node<'win>>>>,
}

impl<'win> Children<'win> {
    pub fn get_or_create(&mut self, socket: SocketName) -> &mut QueueBuilder<'win, Node<'win>> {
        if socket.is_default() {
            &mut self.default_socket
        } else {
            self.other_sockets
                .get_or_insert_with(Default::default)
                .entry(socket)
                .or_default()
        }
    }

    pub fn get(&mut self, socket: SocketName) -> Option<&mut QueueBuilder<'win, Node<'win>>> {
        if socket.is_default() {
            Some(&mut self.default_socket)
        } else {
            self.other_sockets.as_mut().and_then(|sockets| sockets.get_mut(&socket))
        }
    }

    pub fn remove(&mut self, socket: SocketName) -> Option<QueueBuilder<'win, Node<'win>>> {
        if socket.is_default() {
            Some(std::mem::replace(&mut self.default_socket, QueueBuilder::default()))
        } else {
            self.other_sockets.as_mut().and_then(|sockets| sockets.remove(&socket))
        }
    }
}

pub(crate) type SubContextStack<'win> = Vec<(Node<'win>, SocketName)>;

pub struct SubContext<'slf, 'ctx, 'win> {
    pub(crate) max_area: Area,
    pub(crate) root: Node<'win>,
    pub(crate) ctx: &'slf mut Context<'ctx, 'win>,
}

impl<'slf, 'ctx, 'win> SubContext<'slf, 'ctx, 'win> {
    pub fn close(mut self) -> LayoutObj {
        while !self.ctx.subctx_stack.is_empty() {
            self.end();
        }

        // Need to create a context for the root element
        let ctx = Context {
            max_area: self.max_area,
            children: self.root.children,

            global_data: self.ctx.global_data,
            prev_frame_state: self.ctx.prev_frame_state,
            buffer: self.ctx.buffer,
            subctx_stack: self.ctx.subctx_stack,
        };

        self.root.elem.run(ctx, self.root.id)
    }

    pub fn end<'a>(&'a mut self) -> &'a mut Self {
        let (node, socket) = self.ctx.subctx_stack.pop().expect("Bad call to 'end'");

        // Get the parent node
        let parent = match self.ctx.subctx_stack.last_mut() {
            Some(parent) => &mut parent.0,
            None => &mut self.root,
        };

        parent.children.get_or_create(socket).push_back(self.ctx.buffer, node);
        self
    }

    pub fn begin<'a, E: Element + 'static>(
        &'a mut self,
        socket: SocketName,
        id: Id,
        elem: E,
    ) -> &'a mut Self {
        let node = Node {
            id,
            elem: self.ctx.buffer.alloc(elem),
            children: Children::default(),
        };

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

    pub fn new_state<T: StateT>(&mut self) -> State<T> {
        self.ctx.new_state()
    }

    pub fn read_state<T: StateT>(&self, state: State<T>) -> T {
        self.ctx.read_state(state)
    }
}

impl<'slf, 'win> Context<'slf, 'win> {
    pub fn max_area(&self) -> Area {
        self.max_area
    }

    pub fn open_element<'a, E: Element + 'static>(
        &'a mut self,
        max_area: Area,
        id: Id,
        elem: E,
    ) -> SubContext<'a, 'slf, 'win> {
        let buf = self.buffer;

        // Clear the subcontext stack before using it
        self.subctx_stack.clear();

        SubContext {
            max_area,
            root: Node {
                id,
                elem: buf.alloc(elem),
                children: Children::default(),
            },

            ctx: self,
        }
    }

    pub fn open_socket(&mut self, name: SocketName, max_area: Area, socket: &mut dyn Socket) {
        // TODO: This doesn't opening multiple times
        let mut children = match self.children.remove(name) {
            Some(children) => children.finish(),
            None => return,
        };

        // Fill the socket
        while socket.remaining_capacity() != 0 {
            let child = match children.pop_front() {
                Some(child) => child,
                None => break,
            };

            // Run the child
            let sub_ctx = Context {
                max_area,
                children: child.children,

                prev_frame_state: self.prev_frame_state,
                global_data: self.global_data,
                buffer: self.buffer,
                subctx_stack: self.subctx_stack,
            };

            socket.push(child.elem.run(sub_ctx, child.id));
        }
    }

    pub fn next_frame_pre_filter<F: Filter>(&mut self, _filter: F) {
        unimplemented!()
    }

    pub fn next_frame_post_filter<F: Filter>(&mut self, _filter: F) {
        unimplemented!()
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
