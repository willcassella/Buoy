use crate::prelude::*;
use crate::builder::*;

pub trait ElementExt: Element {
    fn begin<'a, 'b, 'ctx>(
        self,
        ctx: &'a mut BuilderContext<'b, 'ctx>,
        id: Id,
    ) -> &'a mut BuilderContext<'b, 'ctx> {
        ctx.element_begin(self, id);
        ctx
    }
}

impl<T: Element> ElementExt for T {
}
