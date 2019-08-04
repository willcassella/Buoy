use crate::core::common::*;
use crate::util::arena::{Arena, ABox};
use crate::util::into_any::IntoAny;

use crate::space::Area;

mod context;
pub use self::context::{Context, SubContext};

mod id;
pub use self::id::Id;

mod socket;
pub use self::socket::{Socket, SocketName};

mod socket_tree;
pub(in crate::core) use self::socket_tree::{ElementNode, SocketTree, ElementQNode};

mod layout;
pub use self::layout::{Layout, LayoutNode};

pub trait Element: IntoAny {
    fn run<'ctx, 'frm>(&self, ctx: Context<'ctx, 'frm>, id: Id) -> LayoutNode<'frm>;
}

impl Element for () {
    fn run<'ctx, 'frm>(&self, ctx: Context<'ctx, 'frm>, _id: Id) -> LayoutNode<'frm> {
        ctx.new_layout_null()
    }
}

pub trait AllocElement<'frm> {
    fn alloc(self, buf: &'frm Arena) -> ABox<'frm, dyn Element>;
}

impl<'frm> AllocElement<'frm> for ABox<'frm, dyn Element> {
    fn alloc(self, _buf: &'frm Arena) -> ABox<'frm, dyn Element> {
        self
    }
}

impl<'frm, T: Element + 'static> AllocElement<'frm> for ABox<'frm, T> {
    fn alloc(self, _buf: &'frm Arena) -> ABox<'frm, dyn Element> {
        self.unsize()
    }
}

impl<'frm, T: Element + 'static> AllocElement<'frm> for T {
    fn alloc(self, buf: &'frm Arena) -> ABox<'frm, dyn Element> {
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

    fn open<'a, 'ctx, 'frm>(
        self,
        ctx: &'a mut Context<'ctx, 'frm>,
        max_area: Area,
    ) -> SubContext<'a, 'ctx, 'frm> {
        ctx.open_sub(max_area, self.get_id(), self.get_element())
    }
}