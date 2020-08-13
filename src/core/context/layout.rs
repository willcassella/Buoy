use crate::core::context::*;
use crate::core::device::*;
use crate::core::id::Id;
use crate::message::*;
use crate::space::*;
use crate::util::arena::Arena;
use crate::util::drain_filter::DrainFilter;
use crate::util::ref_move::{ref_move, Anchor};

pub enum LayoutResult<T> {
    None,
    // TODO: Deferred,
    Complete { min_area: Area, layout: T },
    CompleteNode(LayoutNode),
}

pub struct LayoutNode {
    pub type_id: TypeId,
    pub index: LayoutIndex,
    pub min_area: Area,
}

pub struct LayoutContext<'thrd, 'frm, C> {
    pub(in crate::core) gui_ctx: &'frm GuiContext<C>,
    pub(in crate::core) frame_ctx: &'frm FrameContext,
    pub(in crate::core) thread_ctx: &'thrd ThreadContext<'frm, C>,

    pub(in crate::core) max_area: Area,
    pub(in crate::core) children: Vec<(SocketName, SubDevice<'thrd, 'frm, C>)>,
}

impl<'thrd, 'frm, C: 'static> LayoutContext<'thrd, 'frm, C> {
    #[inline]
    pub fn max_area(&self) -> Area {
        self.max_area
    }

    pub fn socket_children_len(&self, socket: SocketName) -> usize {
        self.children.iter().filter(|(k, _)| *k == socket).count()
    }

    // TODO: It would be nice if I didn't have to expose this
    #[inline]
    pub fn buffer(&self) -> &'frm Arena {
        self.thread_ctx.buffer()
    }

    pub fn device_tree<D: Anchor<dyn Device + 'frm>, T: LayoutTree<'frm, C>>(
        &mut self,
        max_area: Area,
        device: D,
        subtree: T,
    ) -> LayoutResult<()> {
        // Look up the renderer for this type
        let renderer = self
            .thread_ctx
            .renderer_for(self.gui_ctx, device.get_type_id());

        // Allocate the device with its renderer
        let index = ref_move(device, |d| renderer.alloc(d));
        let mut sub_device = SubDevice {
            renderer,
            index,
            children: Vec::new(),
        };

        // Visit the subtree
        let visitor = LayoutTreeVisitor {
            parent: &mut sub_device,
            gui_ctx: self.gui_ctx,
            thread_ctx: self.thread_ctx,
            ctx_children: &mut self.children,
        };
        subtree.visit(visitor);

        // Create a context for running the device
        let ctx = LayoutContext {
            gui_ctx: self.gui_ctx,
            frame_ctx: self.frame_ctx,
            thread_ctx: self.thread_ctx,

            max_area,
            children: sub_device.children,
        };

        match sub_device.renderer.layout(sub_device.index, ctx) {
            RendererLayoutResult::None => LayoutResult::None,
            RendererLayoutResult::Complete(layout_node) => LayoutResult::CompleteNode(layout_node),
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
                gui_ctx: self.gui_ctx,
                frame_ctx: self.frame_ctx,
                thread_ctx: self.thread_ctx,

                max_area,
                children: std::mem::take(&mut device.children),
            };

            match device.renderer.layout(device.index, ctx) {
                RendererLayoutResult::None => (),
                RendererLayoutResult::Complete(layout_node) => socket.push(layout_node),
            };
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
        self.frame_ctx.read_message(inbox)
    }

    #[inline]
    pub fn write_message<T: Message>(&mut self, outbox: Outbox<T>, value: T) {
        self.thread_ctx.write_message(outbox, value)
    }
}

pub struct LayoutTreeVisitor<'slf, 'thrd, 'frm, C> {
    parent: &'slf mut SubDevice<'thrd, 'frm, C>,
    gui_ctx: &'frm GuiContext<C>,
    thread_ctx: &'thrd ThreadContext<'frm, C>,
    ctx_children: &'slf mut Vec<(SocketName, SubDevice<'thrd, 'frm, C>)>,
}

impl<'slf, 'thrd, 'frm: 'thrd, C: 'static> LayoutTreeVisitor<'slf, 'thrd, 'frm, C> {
    pub fn socket(&mut self, parent: SocketName, name: SocketName, limit: Option<usize>) {
        let mut limit = limit.unwrap_or(usize::MAX);
        let children_iter = self.ctx_children.buoy_drain_filter(|child| {
            if limit == 0 {
                return false;
            }

            if child.0 != name {
                return false;
            }

            child.0 = parent;
            limit -= 1;
            true
        });

        // Insert the children into the parent
        self.parent.children.extend(children_iter);
    }

    pub fn device<D: Anchor<dyn Device + 'frm>>(&mut self, socket: SocketName, device: D) {
        // Look up the renderer for this type
        let renderer = self
            .thread_ctx
            .renderer_for(self.gui_ctx, device.get_type_id());

        // TODO: Determine if it's viable to just call into 'device_tree' with a no-op tree
        // Might not be as performant for debug builds.

        // Allocate the device and add it to the parent
        let index = ref_move(device, |d| renderer.alloc(d));
        self.parent.children.push((
            socket,
            SubDevice {
                renderer,
                index,
                children: Vec::new(),
            },
        ));
    }

    pub fn device_tree<D: Anchor<dyn Device + 'frm>, T: LayoutTree<'frm, C>>(
        &mut self,
        socket: SocketName,
        device: D,
        subtree: T,
    ) {
        // Look up the renderer for this type
        let renderer = self
            .thread_ctx
            .renderer_for(self.gui_ctx, device.get_type_id());

        // Allocate the device with its renderer
        let index = ref_move(device, |d| renderer.alloc(d));
        let mut sub_device = SubDevice {
            renderer,
            index,
            children: Vec::new(),
        };

        // Visit the subtree
        let visitor = LayoutTreeVisitor {
            parent: &mut sub_device,
            gui_ctx: self.gui_ctx,
            thread_ctx: self.thread_ctx,
            ctx_children: self.ctx_children,
        };
        subtree.visit(visitor);

        // Add the device to the parent
        self.parent.children.push((socket, sub_device));
    }
}

pub trait LayoutTree<'frm, C> {
    fn visit<'ctx, 'thrd>(self, visitor: LayoutTreeVisitor<'ctx, 'thrd, 'frm, C>);
}

impl<'frm, C, F> LayoutTree<'frm, C> for F
where
    F: for<'ctx, 'thrd> FnOnce(LayoutTreeVisitor<'ctx, 'thrd, 'frm, C>)
{
    fn visit<'ctx, 'thrd>(self, visitor: LayoutTreeVisitor<'ctx, 'thrd, 'frm, C>) {
        self(visitor)
    }
}

impl<'frm, C> LayoutTree<'frm, C> for () {
    fn visit<'ctx, 'thrd>(self, _visitor: LayoutTreeVisitor<'ctx, 'thrd, 'frm, C>) {}
}

pub(in crate::core) struct SubDevice<'thrd, 'frm, C> {
    renderer: &'thrd dyn RendererWrapper<'frm, C>,
    index: DeviceIndex,
    children: Vec<(SocketName, SubDevice<'thrd, 'frm, C>)>,
}
