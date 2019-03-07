use std::any::Any;

use crate::core::{Context, socket};
use crate::builder::BuilderContext;

mod id;
pub use id::Id;

mod dyn_element;
pub use dyn_element::DynElement;

pub trait Element: Sized + Clone + Any {
    type Next: DynElement;

    fn run(
        self,
        ctx: &mut Context,
        socket: &mut dyn socket::Socket,
    ) -> Option<Self::Next>;

    fn upcast(
        self,
    ) -> Box<dyn DynElement> {
        Box::new(self)
    }

    fn downcast<D: Element>(
        self,
    ) -> Result<D, Self> {
        Err(self) // TODO: This should handle when Self == D
    }

    fn begin<'a, 'b, 'ctx>(
        self,
        ctx: &'a mut BuilderContext<'b, 'ctx>,
        id: Id,
    ) -> &'a mut BuilderContext<'b, 'ctx> {
        ctx.element_begin(self, id);
        ctx
    }
}

impl Element for () {
    type Next = ();

    fn run(
        self,
        _ctx: &mut Context,
        _socket: &mut dyn socket::Socket,
    ) -> Option<Self::Next> {
        None
    }
}
