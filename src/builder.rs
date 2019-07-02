use std::any::Any;

use crate::prelude::*;

mod builder_context;
pub use self::builder_context::BuilderContext;
use self::builder_context::NodeKind;

mod tree;

mod element_ext;
pub use self::element_ext::ElementExt;

pub trait Builder: Any {
    fn run<'a, 'window, 'ctx>(
        &self,
        ctx: &mut BuilderContext<'a, 'window, 'ctx>,
    );
}

impl<T: Builder> Element for T {
    fn run<'window, 'ctx>(
        &self,
        mut ctx: Context<'window, 'ctx>,
    ) {
        let max_area = ctx.max_area();

        // Run the builder
        let mut builder_ctx = BuilderContext::new(&mut ctx);
        Builder::run(self, &mut builder_ctx);

        // The builder may not have actually output a root (should change the API to fix this...)
        let root = match builder_ctx.get_root() {
            Some(root) => root,
            None => return,
        };

        // The root might not actually be an element
        // Is there a good way of changing to API to fix this?
        match root.kind {
            NodeKind::Element(element, id) => {
                let layout = ctx.subcontext(max_area, id, &*element, &mut ());
                if let Some(layout) = layout {
                    ctx.layout(layout);
                }
            },
            _ => {},
        }
    }
}
