use crate::builder::*;

pub trait ElementExt: Element {
    fn begin<C: BuilderContext>(
        self,
        ctx: &mut C,
        id: Id,
    ) -> &mut C {
        ctx.element_begin(self, id);
        ctx
    }
}

impl<T: Element> ElementExt for T {
}
