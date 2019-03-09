use std::any::Any;
use crate::core::*;

mod context;
pub use self::context::BuilderContext;

mod tree;

pub trait Builder: Sized + Clone + Any {
    fn run(
        self,
        ctx: &mut BuilderContext,
    );
}

impl<T: Builder> Element for T {
    type Resume = ();

    fn run(
        self,
        ctx: &mut Context,
        socket: &mut dyn Socket,
    ) -> Option<Self::Resume> {
        let max_area = ctx.max_area();

        // Run the builder
        let mut builder_ctx = BuilderContext::new(ctx);
        Builder::run(self, &mut builder_ctx);

        // Create a sub-context for consuming the built tree
        let mut tree = builder_ctx.into_tree();
        let mut sub_ctx = Context::new_sub(ctx, &mut tree);

        // TODO: Should this require opening a new socket?
        sub_ctx.socket(socket::Id::default(), socket, max_area);

        None
    }
}
