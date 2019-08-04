use crate::prelude::*;

pub trait Wrap {
    fn open(&self, max_area: Area) -> Area {
        max_area
    }

    fn close_some<'ctx, 'frm>(&self, ctx: Context<'ctx, 'frm>, id: Id, child: LayoutNode<'frm>) -> LayoutNode<'frm>;

    fn close_none<'ctx, 'frm>(&self, ctx: Context<'ctx, 'frm>, id: Id) -> LayoutNode<'frm>;
}

pub fn wrap<'ctx, 'frm, W: Wrap>(mut ctx: Context<'ctx, 'frm>, id: Id, wrap: &W) -> LayoutNode<'frm> {
    let child_max_area = wrap.open(ctx.max_area());

    let mut child = None;
    ctx.open_socket(SocketName::default(), child_max_area, &mut child);

    match child {
        Some(child) => wrap.close_some(ctx, id, child),
        None => wrap.close_none(ctx, id),
    }
}
