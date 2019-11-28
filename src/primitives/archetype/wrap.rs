use crate::prelude::*;

pub trait Wrap {
    fn open(
        &self,
        max_area: Area
    ) -> Area {
        max_area
    }

    fn close_some<'ctx, 'frm>(
        self,
        id: Id,
        ctx: Context<'ctx, 'frm>,
        child: LayoutNode<'frm>
    ) -> LayoutNode<'frm>;

    fn close_none<'ctx, 'frm>(
        self,
        id: Id,
        ctx: Context<'ctx, 'frm>
    ) -> LayoutNode<'frm>;
}

pub fn wrap<'ctx, 'frm, W: Wrap>(
    wrap: W,
    id: Id,
    mut ctx: Context<'ctx, 'frm>
) -> LayoutNode<'frm> {
    let child_max_area = wrap.open(ctx.max_area());

    let mut child = None;
    ctx.open_socket(SocketName::default(), child_max_area, &mut child);

    match child {
        Some(child) => wrap.close_some(id, ctx, child),
        None => wrap.close_none(id, ctx),
    }
}
