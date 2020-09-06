use crate::core::context::*;
use crate::core::device::*;
use crate::core::id::Id;
use crate::message::*;
use crate::space::*;
use crate::util::arena::Arena;
use crate::util::drain_filter::DrainFilter;
use std::marker::PhantomData;
use std::cell::Cell;

pub struct LayoutNode<C, S: Space> {
    min_size: S::Size,
    layout: Box<dyn LayoutWrapper<C, Space = S>>,
}

impl<C, S: Space> LayoutNode<C, S> {
    pub fn min_size(&self) -> S::Size {
        self.min_size
    }
}

// Gross
enum LayoutKind {
    Width,
    Height,
    Both,
}

struct LayoutReceiver<C> {
    callback: Box<dyn FnOnce(Context<C, Space1D>, &mut C)>,
}

struct LayoutReceiver1D<D> {
    device: D,
}

struct LayoutReceiver2D<D> {
    device: D,
    horiz_region: Cell<Option<Region1D>>,
    vert_region: Cell<Option<Region1D>>,
}

pub struct LayoutStackLayer2D<C> {
    horiz_sockets: Vec<(SocketName, Size1D, Vec<LayoutReceiver<C>>)>,
    vert_sockets: Vec<(SocketName, Size1D, Vec<LayoutReceiver<C>>)>,
}

pub struct Context<'thrd, 'frm, C, S> {
    pub(in crate::core) gui_ctx: &'frm GuiContext<C>,
    pub(in crate::core) frame_ctx: &'frm FrameContext,
    pub(in crate::core) thread_ctx: &'thrd ThreadContext<'frm>,
    pub(in crate::core) layout_stack: Vec<LayoutStackLayer2D<C>>,

    // pub(in crate::core) children: Vec<(SocketName, SubDevice<'thrd, 'frm, C>)>,
    _p: PhantomData<&'thrd S>,
}

impl<'thrd, 'frm, C: 'static, S: Space> Context<'thrd, 'frm, C, S> {
    // TODO: It would be nice if I didn't have to expose this
    #[inline]
    pub fn buffer(&self) -> &'frm Arena {
        self.thread_ctx.buffer()
    }

    pub fn sub_layout<D: Device<C, Space = S>>(&self, device: D) -> SubLayoutContext<C, S> {
        unimplemented!()
    }

    pub fn open_socket(&mut self, name: SocketSource<S>) -> Option<LayoutNode<C, S>> {
        unimplemented!()
    }

    pub fn layout<T: Layout<C, Space = S>>(&mut self, min_size: S::Size, layout: T) {
        unimplemented!()
    }

    #[inline]
    pub fn message<T: Message>(&mut self, id: Id) -> Outbox<T> {
        Outbox::new(id)
    }

    #[inline]
    pub fn read_message<T: Message, I: Into<Inbox<T>>>(&self, inbox: I) -> Option<T> {
        self.frame_ctx.read_message(inbox)
    }

    #[inline]
    pub fn write_message<T: Message>(&mut self, outbox: Outbox<T>, value: T) {
        self.thread_ctx.write_message(outbox, value)
    }
}

pub struct StackLayer2D {
    children: Vec<(SocketName, Size2D, )>
}

pub struct SubLayoutContext<'slf, 'thrd, 'frm, C, S: Space> {
    _p: PhantomData<(&'slf (), &'thrd (), &'frm (), C, S)>,
    stack: Vec<()>
}

impl<'slf, 'thrd, 'frm: 'thrd, C: 'static, S: Space> SubLayoutContext<'slf, 'thrd, 'frm, C, S> {
    // Closes this SubLayout. This should return something that can be rendered
    pub fn finish(self) -> LayoutNode<C, S> {
        unimplemented!()
    }

    pub fn pop(&mut self) {
        unimplemented!()
    }
}

impl<'slf, 'thrd, 'frm: 'thrd, C: 'static> SubLayoutContext<'slf, 'thrd, 'frm, C, Space1D> {
    pub fn push_device<D: Device<C, Space = Space1D>>(&mut self, socket: SocketName, device: D) {
        let device = Box::new(device);

        unimplemented!()
    }

    pub fn layout<L: Layout<C, Space = Space1D>>(&mut self, socket: SocketName, layout: L) {
        unimplemented!()
    }

    pub fn socket(&mut self, parent: SocketName, socket: SocketName) {
        unimplemented!()
    }
}

impl<'slf, 'thrd, 'frm: 'thrd, C: 'static> SubLayoutContext<'slf, 'thrd, 'frm, C, Space2D> {
    pub fn push_device_horiz<D: Device<C, Space = Space1D>>(
        &mut self,
        horiz_socket: SocketName,
        device: D,
    ) {
        unimplemented!()
    }

    pub fn push_device_vert<D: Device<C, Space = Space1D>>(
        &mut self,
        vert_socket: SocketName,
        device: D,
    ) {
        unimplemented!()
    }

    pub fn push_device_2d<D: Device<C, Space = Space2D>>(
        &mut self,
        horiz_socket: SocketName,
        vert_socket: SocketName,
        device: D,
    ) {
        unimplemented!()
    }

    pub fn layout_2d<D: Device<C, Space = Space2D>>(
        &mut self,
        horiz_socket: SocketName,
        vert_socket: SocketName,
        device: D,
    ) {
        unimplemented!()
    }

    pub fn socket_2d(
        &mut self,
        horiz_socket: SocketName,
        vert_socket: SocketName,
        socket: SocketName,
    ) {
        unimplemented!()
    }
}
