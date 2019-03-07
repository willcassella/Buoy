use std::any::Any;
use crate::core::*;

mod context;
pub use self::context::BuilderContext;

pub trait Builder: Sized + Clone + Any {
    fn run(
        self,
        ctx: &mut BuilderContext,
    );
}

impl<T: Builder> Element for T {
    type Next = ();

    fn run(
        self,
        ctx: &mut Context,
        _socket: &mut dyn Socket,
    ) -> Option<Self::Next> {
        // Run the builder
        let mut builder_ctx = BuilderContext::new(ctx);
        Builder::run(self, &mut builder_ctx);
        None
    }
}
