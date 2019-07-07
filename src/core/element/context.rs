use std::marker::PhantomData;
use std::collections::HashMap;
use std::collections::VecDeque;

use crate::space::*;
use crate::input::*;
use crate::core::element::*;
use crate::core::filter::*;

pub struct Context<'window, 'ctx> {
    pub(crate) p: PhantomData<&'ctx ()>,

    pub(crate) max_area: Area,
    pub(crate) element_id: Id,
    pub(crate) children: Children,

    pub(crate) prev_input: &'window InputCache,
    pub(crate) global_data: &'window mut GlobalData,
}

pub(crate) struct Node {
    id: Id,
    elem: Box<dyn Element>,
    children: Children,
}

pub(crate) type Children = HashMap<SocketName, VecDeque<Node>>;

pub struct Builder<'a, 'window, 'ctx> {
    pub(crate) ctx: &'a mut Context<'window, 'ctx>,
    pub(crate) max_area: Area,
    pub(crate) root: Node,
    pub(crate) stack: Vec<(Node, SocketName)>,
}

impl<'a, 'window, 'ctx> Builder<'a, 'window, 'ctx> {
    pub fn finish(
        mut self,
    ) -> LayoutObj {
        while !self.stack.is_empty() {
            self.end();
        }

        // Need to create a context for this element
        let sub_ctx = Context {
            p: PhantomData,

            max_area: self.max_area,
            element_id: self.root.id,
            children: self.root.children,

            prev_input: self.ctx.prev_input,
            global_data: self.ctx.global_data,
        };

        self.root.elem.run(sub_ctx)
    }

    pub fn end<'b>(
        &'b mut self,
    ) -> &'b mut Builder<'a, 'window, 'ctx> {

        let (node, socket) = self.stack.pop().expect("Bad call to 'end'");

        // Get the parent node
        let parent = match self.stack.last_mut() {
            Some(parent) => &mut parent.0,
            None => &mut self.root,
        };

        parent.children.entry(socket).or_default().push_back(node);
        self
    }

    pub fn begin_element<'b, E: Element + 'static>(
        &'b mut self,
        socket: SocketName,
        id: Id,
        elem: E,
    ) -> &'b mut Builder<'a, 'window, 'ctx> {
        let node = Node {
            id,
            elem: Box::new(elem),
            children: Children::new(),
        };

        self.stack.push((node, socket));
        self
    }

    pub fn connect_socket<'b>(
        &'b mut self,
        target: SocketName,
        socket: SocketName,
    ) -> &'b mut Builder<'a, 'window, 'ctx> {
        // Get the current children
        let mut children = match self.ctx.children.remove_entry(&socket) {
            Some((_, children)) => children,
            None => return self,
        };

        // Get the parent
        let parent = match self.stack.last_mut() {
            Some(parent) => &mut parent.0,
            None => &mut self.root,
        };

        // Insert the children into the parent
        parent.children.entry(target).or_default().append(&mut children);
        self
    }
}

impl<'window, 'ctx> Context<'window, 'ctx> {
    // Returns the id of the currently running element
    pub fn element_id(&self) -> Id {
        self.element_id
    }

    pub fn max_area(&self) -> Area {
        self.max_area
    }

    pub fn begin_element<'a, E: Element + 'static>(
        &'a mut self,
        max_area: Area,
        id: Id,
        elem: E,
    ) -> Builder<'a, 'window, 'ctx> {
        Builder {
            ctx: self,
            max_area,
            root: Node {
                id,
                elem: Box::new(elem),
                children: Children::new(),
            },
            stack: Vec::new(),
        }
    }

    pub fn open_socket(
        &mut self,
        name: SocketName,
        socket: &mut dyn Socket,
        max_area: Area,
    ) {
        let children = match self.children.get_mut(&name) {
            Some(children) => children,
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
                p: PhantomData,
                max_area,
                element_id: child.id,
                children: child.children,
                prev_input: self.prev_input,
                global_data: self.global_data,
            };

            socket.push(child.elem.run(sub_ctx));
        }
    }

    pub fn next_frame_pre_filter<F: Filter>(
        &mut self,
        _filter: F,
    ) {
        unimplemented!()
    }

    pub fn next_frame_post_filter<F: Filter>(
        &mut self,
        _filter: F,
    ) {
        unimplemented!()
    }

    pub fn new_input<F: InputState>(&mut self) -> Input<F> {
        let id = self.global_data.next_input_id.increment();
        Input::new(id)
    }

    pub fn read_input<F: InputState>(&self, input: Input<F>) -> F {
        if input.id.frame_id != self.global_data.next_input_id.frame_id.prev() {
            panic!("Attempt to read state from wrong frame");
        }

        if let Some(v) = self.prev_input.get(&input.id) {
            v.downcast_ref::<F>().expect("Mismatched types").clone()
        } else {
            Default::default()
        }
    }
}
