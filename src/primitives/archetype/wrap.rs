use crate::prelude::*;

pub trait Wrap {
    fn open(&self, max_area: Area) -> Area {
        max_area
    }

    fn close_some<'frm, 'thrd, 'ctx>(
        self,
        ctx: Context<'frm, 'thrd, 'ctx>,
        child: LayoutNode<'frm>,
    ) -> LayoutNode<'frm>;

    fn close_none<'frm, 'thrd, 'ctx>(self, ctx: Context<'frm, 'thrd, 'ctx>) -> LayoutNode<'frm>;
}

pub fn wrap<'frm, 'thrd, 'ctx, W: Wrap>(
    wrap: W,
    mut ctx: Context<'frm, 'thrd, 'ctx>,
) -> LayoutNode<'frm> {
    let child_max_area = wrap.open(ctx.max_area());

    let mut child = None;
    ctx.socket(SocketName::default(), child_max_area, &mut child);

    match child {
        Some(child) => wrap.close_some(ctx, child),
        None => wrap.close_none(ctx),
    }
}
