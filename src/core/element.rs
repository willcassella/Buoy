use crate::core::common::*;
use crate::util::linked_buffer::{LinkedBuffer, LBBox};
use crate::util::into_any::IntoAny;

use crate::space::Area;

mod context;
pub use self::context::{Context, SubContext};

mod children;
pub(crate) use self::children::Children;

mod id;
pub use self::id::Id;

mod socket;
pub use self::socket::{Socket, SocketName};

mod layout;
pub use self::layout::{Layout, LayoutNode};

// An 'Element' is something run in the the context of a socket
// This is the starting point for any UI tree
pub trait Element: IntoAny {
    fn run<'ctx, 'win>(&self, ctx: Context<'ctx, 'win>, id: Id) -> LayoutNode<'win>;
}

impl Element for () {
    fn run<'ctx, 'win>(&self, ctx: Context<'ctx, 'win>, _id: Id) -> LayoutNode<'win> {
        ctx.new_layout_null()
    }
}

pub trait AllocElement<'frm> {
    fn alloc(self, buf: &'frm LinkedBuffer) -> LBBox<'frm, dyn Element>;
}

impl<'frm> AllocElement<'frm> for LBBox<'frm, dyn Element> {
    fn alloc(self, _buf: &'frm LinkedBuffer) -> LBBox<'frm, dyn Element> {
        self
    }
}

impl<'frm, T: Element + 'static> AllocElement<'frm> for T {
    fn alloc(self, buf: &'frm LinkedBuffer) -> LBBox<'frm, dyn Element> {
        buf.alloc(self).unsize()
    }
}

pub trait Builder: Sized {
    type Element: Element + 'static;

    fn get_id(&self) -> Id;

    fn get_socket(&self) -> SocketName;

    fn get_element(self) -> Self::Element;

    fn begin(self, sub_ctx: &mut SubContext) {
        sub_ctx.begin(self.get_socket(), self.get_id(), self.get_element());
    }

    fn open<'a, 'slf, 'win>(
        self,
        ctx: &'a mut Context<'slf, 'win>,
        max_area: Area,
    ) -> SubContext<'a, 'slf, 'win> {
        ctx.open_element(max_area, self.get_id(), self.get_element())
    }
}