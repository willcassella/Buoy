use crate::prelude::*;

pub trait Panel {
    fn open(&self, max_area: Area) -> Area;

    fn close(&self, ctx: Context, id: Id, children: Vec<LayoutObj>) -> LayoutObj;
}

pub fn panel<T: Panel>(mut ctx: Context, id: Id, panel: &T) -> LayoutObj {
    let child_max_area = panel.open(ctx.max_area());

    let mut children = Vec::new();
    ctx.open_socket(SocketName::default(), child_max_area, &mut children);

    panel.close(ctx, id, children)
}
