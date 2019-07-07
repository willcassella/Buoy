use crate::core::common::*;

use crate::space::Area;

mod context;
pub use self::context::{Context, Builder};
pub(crate) use self::context::{Children};

mod id;
pub use self::id::Id;

mod socket;
pub use self::socket::{Socket, SocketName};

mod layout;
pub use self::layout::{Layout, LayoutObj};

pub struct ElementId;

// An 'Element' is something run in the the context of a socket
// This is the starting point for any UI tree
pub trait Element {
    fn run(
        &self,
        ctx: Context,
    ) -> LayoutObj;
}

impl Element for () {
    fn run<'window, 'ctx>(
        &self,
        _ctx: Context<'window, 'ctx>,
    ) -> LayoutObj {
        LayoutObj::new(Area::zero(), ()).upcast()
    }
}

pub trait ElementExt: Element {
    fn begin<'a, 'b, 'window, 'ctx>(
        self,
        builder: &'a mut Builder<'b, 'window, 'ctx>,
        socket: SocketName,
        id: Id,
    ) -> &'a mut Builder<'b, 'window, 'ctx>;
}

impl<T: Element + 'static> ElementExt for T {
    fn begin<'a, 'b, 'window, 'ctx>(
        self,
        builder: &'a mut Builder<'b, 'window, 'ctx>,
        socket: SocketName,
        id: Id,
    ) -> &'a mut Builder<'b, 'window, 'ctx> {
        builder.begin_element(socket, id, self)
    }
}