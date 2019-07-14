use crate::prelude::*;

pub trait Panel {
    fn open(&self, max_area: Area) -> Area;

    fn close<'ctx, 'win>(&self, ctx: Context<'ctx, 'win>, id: Id, children: Vec<LayoutNode<'win>>) -> LayoutNode<'win>;
}

pub fn panel<'ctx, 'win, T: Panel>(mut ctx: Context<'ctx, 'win>, id: Id, panel: &T) -> LayoutNode<'win> {
    let child_max_area = panel.open(ctx.max_area());

    let mut children = Vec::new();
    ctx.open_socket(SocketName::default(), child_max_area, &mut children);

    panel.close(ctx, id, children)
}
