use crate::builder::*;

pub trait ElementExt: Element + Sized {
    fn begin<'b, 'a, 'window, 'ctx>(
        self,
        ctx: &'b mut BuilderContext<'a, 'window, 'ctx>,
        id: Id,
    ) -> &'b mut BuilderContext<'a, 'window, 'ctx> {
        ctx.element_begin(self, id);
        ctx
    }
}

impl<T: Element> ElementExt for T {
}
