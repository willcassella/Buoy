use crate::core::context::*;
use crate::core::device::*;
use crate::core::id::Id;
use crate::message::*;
use crate::space::*;
use crate::util::arena::Arena;
use crate::util::drain_filter::DrainFilter;
use crate::util::ref_move::{ref_move, Anchor};

pub enum LayoutResult<T> {
    // TODO: Deferred,
    Complete { min_area: Area, layout: T },
    CompleteNode(LayoutNode),
}

pub struct LayoutNode {
    pub type_id: TypeId,
    pub index: LayoutIndex,
    pub min_area: Area,
}

pub struct LayoutContext<'slf, 'thrd, 'frm, C> {
    pub(in crate::core) gui_context: &'frm GuiContext<C>,
    pub(in crate::core) frame_context: &'frm FrameContext,
    pub(in crate::core) thread_context: &'thrd ThreadContext<'frm, C>,

    pub(in crate::core) id: Id,
    pub(in crate::core) max_area: Area,
    pub(in crate::core) children: Vec<(SocketName, SubDevice<'thrd, 'frm, C>)>,
    pub(in crate::core) subctx_stack: &'slf mut Vec<(SocketName, SubDevice<'thrd, 'frm, C>)>,
}

impl<'slf, 'thrd, 'frm, C: 'static> LayoutContext<'slf, 'thrd, 'frm, C> {
    #[inline]
    pub fn id(&self) -> Id {
        self.id
    }

    #[inline]
    pub fn max_area(&self) -> Area {
        self.max_area
    }

    // TODO: It would be nice if I didn't have to expose this
    #[inline]
    pub fn buffer(&self) -> &'frm Arena {
        self.thread_context.buffer()
    }

    pub fn sub_device<'a, D: Anchor<dyn Device + 'frm>>(
        &'a mut self,
        max_area: Area,
        id: Id,
        device: D,
    ) -> LayoutSubContext<'a, 'thrd, 'frm, C> {
        // Look up the renderer for this type
        let renderer = self
            .thread_context
            .renderer_for(self.gui_context, device.get_type_id());
        let index = ref_move(device, |d| renderer.alloc(d));

        // Clear the subcontext stack before using it
        assert!(self.subctx_stack.is_empty());

        LayoutSubContext {
            gui_context: self.gui_context,
            frame_context: self.frame_context,
            thread_context: self.thread_context,
            ctx_children: &mut self.children,

            max_area,
            root: SubDevice {
                id,
                renderer,
                index,
                children: Vec::new(),
            },
            stack: self.subctx_stack,
        }
    }

    pub fn socket<S: Socket>(&mut self, name: SocketName, max_area: Area, socket: &mut S) {
        // Fill the socket
        let mut iter = self
            .children
            .buoy_drain_filter(|(socket, _)| *socket == name);
        while socket.remaining_capacity() != 0 {
            let mut device = match iter.next() {
                Some((_, device)) => device,
                None => break,
            };

            // Run the child
            let ctx = LayoutContext {
                gui_context: self.gui_context,
                frame_context: self.frame_context,
                thread_context: self.thread_context,

                id: device.id,
                max_area,
                children: std::mem::take(&mut device.children),
                subctx_stack: self.subctx_stack,
            };

            let layout_node = match device.renderer.layout(device.index, ctx) {
                RendererLayoutResult::Complete(layout_node) => layout_node,
            };
            socket.push(layout_node);
        }
    }

    pub fn layout<T>(&self, min_area: Area, layout: T) -> LayoutResult<T> {
        LayoutResult::Complete { min_area, layout }
    }

    #[inline]
    pub fn message<T: Message>(&mut self, id: Id) -> (Inbox<T>, Outbox<T>) {
        (Inbox::new(id), Outbox::new(id))
    }

    #[inline]
    pub fn read_message<T: Message>(&self, inbox: Inbox<T>) -> Option<T> {
        self.frame_context.read_message(inbox)
    }

    #[inline]
    pub fn write_message<T: Message>(&mut self, outbox: Outbox<T>, value: T) {
        self.thread_context.write_message(outbox, value)
    }
}

pub struct SubDevice<'thrd, 'frm, C> {
    id: Id,
    renderer: &'thrd dyn RendererWrapper<'frm, C>,
    index: DeviceIndex,
    children: Vec<(SocketName, SubDevice<'thrd, 'frm, C>)>,
}

pub struct LayoutSubContext<'slf, 'thrd, 'frm, C> {
    gui_context: &'frm GuiContext<C>,
    frame_context: &'frm FrameContext,
    thread_context: &'thrd ThreadContext<'frm, C>,
    ctx_children: &'slf mut Vec<(SocketName, SubDevice<'thrd, 'frm, C>)>,

    max_area: Area,
    root: SubDevice<'thrd, 'frm, C>,
    stack: &'slf mut Vec<(SocketName, SubDevice<'thrd, 'frm, C>)>,
}

impl<'slf, 'thrd, 'frm, C: 'static> LayoutSubContext<'slf, 'thrd, 'frm, C> {
    // TODO: Replace this with LayoutResult<!> once never is stabilized
    pub fn close(mut self) -> LayoutResult<()> {
        while !self.stack.is_empty() {
            self.pop();
        }

        // Create a context for running the device
        let ctx = LayoutContext {
            gui_context: self.gui_context,
            frame_context: self.frame_context,
            thread_context: self.thread_context,

            id: self.root.id,
            max_area: self.max_area,
            children: std::mem::take(&mut self.root.children),
            subctx_stack: self.stack,
        };

        let layout_node = match self.root.renderer.layout(self.root.index, ctx) {
            RendererLayoutResult::Complete(layout_node) => layout_node,
        };

        LayoutResult::CompleteNode(layout_node)
    }

    pub fn pop(&mut self) -> &mut Self {
        let (socket, device) = self.stack.pop().expect("Bad call to 'pop'");

        // Get the parent node
        let parent = match self.stack.last_mut() {
            Some(parent) => &mut parent.1,
            None => &mut self.root,
        };

        parent.children.push((socket, device));
        self
    }

    pub fn push_into<D: Anchor<dyn Device + 'frm>>(
        &mut self,
        socket: SocketName,
        id: Id,
        device: D,
    ) -> &mut Self {
        // Look up the renderer for this type
        let renderer = self
            .thread_context
            .renderer_for(self.gui_context, device.get_type_id());

        // Allocate the device, then push it onto the stack
        let index = ref_move(device, |d| renderer.alloc(d));
        self.stack.push((
            socket,
            SubDevice {
                id,
                renderer,
                index,
                children: Vec::new(),
            },
        ));
        self
    }

    pub fn connect_socket(&mut self, target: SocketName, socket: SocketName) -> &mut Self {
        // Get the parent
        let parent = match self.stack.last_mut() {
            Some(parent) => &mut parent.1,
            None => &mut self.root,
        };

        let children_iter = self.ctx_children.buoy_drain_filter(|child| {
            if child.0 != socket {
                return false;
            }

            child.0 = target;
            true
        });

        // Insert the children into the parent
        parent.children.extend(children_iter);
        self
    }

    pub fn connect_all_sockets(&mut self) -> &mut Self {
        // Get the parent
        let parent = match self.stack.last_mut() {
            Some(parent) => &mut parent.1,
            None => &mut self.root,
        };

        // Append all current children
        parent.children.append(&mut self.ctx_children);
        self
    }

    pub fn message<T: Message>(&mut self, id: Id) -> (Inbox<T>, Outbox<T>) {
        (Inbox::new(id), Outbox::new(id))
    }

    pub fn read_message<T: Message>(&self, inbox: Inbox<T>) -> Option<T> {
        self.frame_context.read_message(inbox)
    }

    pub fn write_message<T: Message>(&mut self, outbox: Outbox<T>, value: T) {
        self.thread_context.write_message(outbox, value)
    }
}
