use crate::prelude::*;

pub trait Wrap {
    fn open(&self, max_area: Area) -> Area {
        max_area
    }

    fn close_some<L: Layout>(
        &self,
        ctx: Context,
        id: Id,
        child: LayoutObj<L>,
    ) -> LayoutObj;

    fn close_none(
        &self,
        ctx: Context,
        id: Id,
    ) -> LayoutObj;
}

pub fn wrap<W: Wrap>(
    mut ctx: Context,
    id: Id,
    wrap: &W,
) -> LayoutObj {
    let child_max_area = wrap.open(ctx.max_area());

    let mut child = None;
    ctx.open_socket(SocketName::default(), child_max_area, &mut child);

    match child {
        Some(child) => wrap.close_some(ctx, id, child),
        None => wrap.close_none(ctx, id),
    }
}
