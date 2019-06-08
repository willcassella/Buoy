use std::any::Any;

use crate::prelude::*;

mod builder_context;
pub use self::builder_context::{BuilderContext, BuilderContextImpl};

mod tree;

mod element_ext;
pub use self::element_ext::ElementExt;

pub trait Builder: Sized + Clone + Any {
    fn run<C: BuilderContext>(
        self,
        ctx: &mut C,
    );
}

impl<T: Builder> Element for T {
    fn run<'a, C: Context<'a>>(
        self,
        mut ctx: C,
    ) {
        let max_area = ctx.max_area();

        // Run the builder
        let mut builder_ctx = BuilderContextImpl::new(&mut ctx);
        Builder::run(self, &mut builder_ctx);

        // Create a sub-context for consuming the built tree
        let mut tree = builder_ctx.into_tree();
        let mut sub_ctx = ctx.subcontext(&mut tree);
    }
}
