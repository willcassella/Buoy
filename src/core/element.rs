use std::any::Any;
use crate::core::id::Id;
use crate::util::arena::{Arena, ABox};
use crate::util::into_any::IntoAny;
use crate::space::Area;

mod context;
pub use self::context::{Context, SubContext};

mod socket;
pub use self::socket::{Socket, SocketName};

mod socket_tree;
pub(in crate::core) use self::socket_tree::{ElementNode, SocketTree, ElementQNode};

mod layout;
pub use self::layout::{Layout, LayoutNode};

pub trait Element: Any {
    fn run<'ctx, 'frm>(self, ctx: Context<'ctx, 'frm>, id: Id) -> LayoutNode<'frm>;
}

pub trait DynElement: IntoAny {
    // Gross hack because rust doesn't support arbitrary self types.
    // 'self' in this case is NOT actually a real Box, it's used because Box is the only way
    // to pass ownership of unsized types through trait through a trait object method.
    unsafe fn run<'ctx, 'frm>(self: Box<Self>, ctx: Context<'ctx, 'frm>, id: Id) -> LayoutNode<'frm>;
}

impl<T: Element> DynElement for T {
    unsafe fn run<'ctx, 'frm>(self: Box<Self>, ctx: Context<'ctx, 'frm>, id: Id) -> LayoutNode<'frm> {
        let this = std::ptr::read(Box::into_raw(self));
        this.run(ctx, id)
    }
}

pub struct Elem<'frm, T: ?Sized + DynElement = dyn DynElement> {
    pub id: Id,
    pub data: ABox<'frm, T>,
}

impl<'frm, T: ?Sized + DynElement> Elem<'frm, T> {
    pub fn run<'ctx>(mut self, ctx: Context<'ctx, 'frm>) -> LayoutNode<'frm> {
        unsafe {
            let data_box = Box::from_raw(&mut *self.data);
            ABox::forget_inner(self.data);
            data_box.run(ctx, self.id)
        }
    }
}

pub trait AllocElement<'frm> {
    fn alloc(self, buf: &'frm Arena) -> Elem<'frm>;
}

impl<'frm> AllocElement<'frm> for Elem<'frm> {
    fn alloc(self, _buf: &'frm Arena) -> Elem<'frm> {
        self
    }
}

impl<'frm, T: Element> AllocElement<'frm> for Elem<'frm, T> {
    fn alloc(self, _buf: &'frm Arena) -> Elem<'frm> {
        Elem {
            id: self.id,
            data: self.data.unsize(),
        }
    }
}

impl<'frm> AllocElement<'frm> for (Id, ABox<'frm, dyn DynElement>) {
    fn alloc(self, _buf: &'frm Arena) -> Elem<'frm> {
        Elem {
            id: self.0,
            data: self.1,
        }
    }
}

impl<'frm, T: Element> AllocElement<'frm> for (Id, T) {
    fn alloc(self, buf: &'frm Arena) -> Elem<'frm> {
        Elem {
            id: self.0,
            data: buf.alloc(self.1).unsize(),
        }
    }
}

pub trait Builder: Sized {
    type Element: Element;

    fn get_id(&self) -> Id;

    fn get_socket(&self) -> SocketName;

    fn get_element(self) -> Self::Element;

    fn begin(self, sub_ctx: &mut SubContext) {
        sub_ctx.begin(self.get_socket(), (self.get_id(), self.get_element()));
    }

    fn open<'a, 'ctx, 'frm>(
        self,
        ctx: &'a mut Context<'ctx, 'frm>,
        max_area: Area,
    ) -> SubContext<'a, 'ctx, 'frm> {
        ctx.open_sub(max_area, (self.get_id(), self.get_element()))
    }
}
