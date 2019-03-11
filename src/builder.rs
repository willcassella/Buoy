use std::any::Any;

use crate::prelude::*;

mod context;
pub use self::context::BuilderContext;

mod tree;

mod element_ext;
pub use self::element_ext::ElementExt;

pub trait Builder: Sized + Clone + Any {
    fn run(
        self,
        ctx: &mut BuilderContext,
    );
}

impl<T: Builder> Element for T {
    type Suspended = ();

    fn run(
        self,
        ctx: &mut Context,
        socket: &mut dyn Socket,
    ) -> Option<Self::Suspended> {
        let max_area = ctx.max_area();

        // Run the builder
        let mut builder_ctx = BuilderContext::new(ctx);
        Builder::run(self, &mut builder_ctx);

        // Create a sub-context for consuming the built tree
        let mut tree = builder_ctx.into_tree();
        let mut sub_ctx = Context::new_sub(ctx, &mut tree);

        // TODO: Should this require opening a new socket?
        sub_ctx.socket(SocketName::default(), socket, max_area);

        None
    }
}
