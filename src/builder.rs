use std::any::Any;
use crate::Context;
use crate::element::{UIWidgetImpl, UISocket};

mod context;
pub use self::context::BuilderContext;

pub trait Builder: Sized + Clone + Any {
    fn run(
        self,
        ctx: &mut BuilderContext,
    );
}

impl<T: Builder> UIWidgetImpl for T {
    type Next = ();

    fn run(
        self,
        ctx: &mut Context,
        _socket: &mut dyn UISocket,
    ) -> Option<Self::Next> {
        // Run the builder
        let mut builder_ctx = BuilderContext::new(ctx);
        Builder::run(self, &mut builder_ctx);
        None
    }
}
