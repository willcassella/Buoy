use crate::core::common::*;

use crate::space::Area;

mod context;
pub(crate) use self::context::Children;
pub use self::context::{Context, SubContext};

mod id;
pub use self::id::Id;

mod socket;
pub use self::socket::{Socket, SocketName};

mod layout;
pub use self::layout::{Layout, LayoutNode};

// An 'Element' is something run in the the context of a socket
// This is the starting point for any UI tree
pub trait Element {
    fn run<'ctx, 'win>(&self, ctx: Context<'ctx, 'win>, id: Id) -> LayoutNode<'win>;
}

impl Element for () {
    fn run<'ctx, 'win>(&self, ctx: Context<'ctx, 'win>, _id: Id) -> LayoutNode<'win> {
        ctx.new_layout_null()
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