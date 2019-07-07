use crate::prelude::*;

pub trait Wrap {
    fn open(&self, max_area: Area) -> Area {
        max_area
    }

    fn close_some<'window, 'ctx, L: Layout>(
        &self,
        ctx: Context<'window, 'ctx>,
        child: LayoutObj<L>,
    ) -> LayoutObj;

    fn close_none<'window, 'ctx>(
        &self,
        ctx: Context<'window, 'ctx>,
    ) -> LayoutObj;
}

pub fn wrap<'window, 'ctx, W: Wrap>(
    wrap: &W,
    mut ctx: Context<'window, 'ctx>,
) -> LayoutObj {
    let mut child_socket = None;

    let child_max_area = wrap.open(ctx.max_area());
    ctx.open_socket(SocketName::default(), &mut child_socket, child_max_area);

    match child_socket {
        Some(child) => wrap.close_some(ctx, child),
        None => wrap.close_none(ctx),
    }
}
