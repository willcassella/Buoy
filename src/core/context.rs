use crate::core::component::*;
use crate::core::id::Id;
use crate::message::*;
use crate::space::*;
use crate::util::arena::{ABox, Arena};
use crate::util::drain_filter::DrainFilter;
use crate::util::ref_move::{ref_move, Anchor};

mod window;
pub use window::Window;

mod frame;
pub use frame::FrameContext;

mod thread;
pub use thread::ThreadContext;

pub struct Context<'frm, 'thrd, 'slf> {
    pub(in crate::core) window_context: &'frm Window,
    pub(in crate::core) frame_context: &'frm FrameContext,
    pub(in crate::core) thread_context: &'thrd ThreadContext<'frm>,

    pub(in crate::core) id: Id,
    pub(in crate::core) max_area: Area,
    pub(in crate::core) children: Vec<(SocketName, SubComponent<'frm, 'thrd>)>,
    pub(in crate::core) subctx_stack: &'slf mut Vec<(SocketName, SubComponent<'frm, 'thrd>)>,
}

impl<'frm, 'thrd, 'slf> Context<'frm, 'thrd, 'slf> {
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

    pub fn sub_component<'a, C: Anchor<dyn DynComponent + 'frm>>(
        &'a mut self,
        max_area: Area,
        id: Id,
        component: C,
    ) -> SubContext<'frm, 'thrd, 'a> {
        // Look up the renderer for this type
        let renderer = self
            .thread_context
            .renderer_for(self.window_context, component.get_type_id());
        let index = ref_move(component, |c| renderer.alloc(c));

        // Clear the subcontext stack before using it
        assert!(self.subctx_stack.is_empty());

        SubContext {
            window_context: self.window_context,
            frame_context: self.frame_context,
            thread_context: self.thread_context,
            ctx_children: &mut self.children,

            max_area,
            root: SubComponent {
                id,
                renderer,
                index,
                children: Vec::new(),
            },
            stack: self.subctx_stack,
        }
    }

    pub fn socket<S: Socket<'frm>>(&mut self, name: SocketName, max_area: Area, socket: &mut S) {
        // Fill the socket
        let mut iter = self
            .children
            .buoy_drain_filter(|(socket, _)| *socket == name);
        while socket.remaining_capacity() != 0 {
            let mut component = match iter.next() {
                Some((_, component)) => component,
                None => break,
            };

            // Run the child
            let ctx = Context {
                window_context: self.window_context,
                frame_context: self.frame_context,
                thread_context: self.thread_context,

                id: component.id,
                max_area,
                children: std::mem::take(&mut component.children),
                subctx_stack: self.subctx_stack,
            };

            let layout = component.renderer.layout(ctx, component.index);
            socket.push(layout);
        }
    }

    pub fn new_layout<L: Layout + 'frm>(&self, min_area: Area, layout: L) -> LayoutNode<'frm> {
        LayoutNode {
            min_area,
            layout: ABox::upcast(self.buffer().alloc(layout)),
        }
    }

    #[inline]
    pub fn new_layout_null(&self) -> LayoutNode<'frm> {
        LayoutNode::null(self.buffer())
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

pub struct SubComponent<'frm, 'thrd> {
    id: Id,
    renderer: &'thrd dyn Renderer<'frm>,
    index: ComponentIndex,
    children: Vec<(SocketName, SubComponent<'frm, 'thrd>)>,
}

pub struct SubContext<'frm, 'thrd, 'slf> {
    window_context: &'frm Window,
    frame_context: &'frm FrameContext,
    thread_context: &'thrd ThreadContext<'frm>,
    ctx_children: &'slf mut Vec<(SocketName, SubComponent<'frm, 'thrd>)>,

    max_area: Area,
    root: SubComponent<'frm, 'thrd>,
    stack: &'slf mut Vec<(SocketName, SubComponent<'frm, 'thrd>)>,
}

impl<'frm, 'thrd, 'slf> SubContext<'frm, 'thrd, 'slf> {
    pub fn close(mut self) -> LayoutNode<'frm> {
        while !self.stack.is_empty() {
            self.pop();
        }

        // Create a context for running the component
        let ctx = Context {
            window_context: self.window_context,
            frame_context: self.frame_context,
            thread_context: self.thread_context,

            id: self.root.id,
            max_area: self.max_area,
            children: std::mem::take(&mut self.root.children),
            subctx_stack: self.stack,
        };

        self.root.renderer.layout(ctx, self.root.index)
    }

    pub fn pop(&mut self) -> &mut Self {
        let (socket, component) = self.stack.pop().expect("Bad call to 'pop'");

        // Get the parent node
        let parent = match self.stack.last_mut() {
            Some(parent) => &mut parent.1,
            None => &mut self.root,
        };

        parent.children.push((socket, component));
        self
    }

    pub fn push_into<C: Anchor<dyn DynComponent + 'frm>>(
        &mut self,
        socket: SocketName,
        id: Id,
        component: C,
    ) -> &mut Self {
        // Look up the renderer for this type
        let renderer = self
            .thread_context
            .renderer_for(self.window_context, component.get_type_id());

        // Allocate the component, then push it onto the stack
        let index = ref_move(component, |c| renderer.alloc(c));
        self.stack.push((
            socket,
            SubComponent {
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

    // pub fn message<T: Message>(&mut self, id: Id) -> (Inbox<T>, Outbox<T>) {
    //     self.ctx.message(id)
    // }

    pub fn read_message<T: Message>(&self, inbox: Inbox<T>) -> Option<T> {
        self.frame_context.read_message(inbox)
    }

    pub fn write_message<T: Message>(&mut self, outbox: Outbox<T>, value: T) {
        self.thread_context.write_message(outbox, value)
    }
}
