use crate::prelude::*;

pub trait Wrap {
    fn open(&self, max_area: Area) -> Area {
        max_area
    }

    fn close_some<'ctx, 'win>(&self, ctx: Context<'ctx, 'win>, id: Id, child: LayoutNode<'win>) -> LayoutNode<'win>;

    fn close_none<'ctx, 'win>(&self, ctx: Context<'ctx, 'win>, id: Id) -> LayoutNode<'win>;
}

pub fn wrap<'ctx, 'win, W: Wrap>(mut ctx: Context<'ctx, 'win>, id: Id, wrap: &W) -> LayoutNode<'win> {
    let child_max_area = wrap.open(ctx.max_area());

    let mut child = None;
    ctx.open_socket(SocketName::default(), child_max_area, &mut child);

    match child {
        Some(child) => wrap.close_some(ctx, id, child),
        None => wrap.close_none(ctx, id),
    }
}
