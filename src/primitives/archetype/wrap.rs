use crate::prelude::*;

pub trait Wrap {
    fn open(&self, max_area: Area) -> Area {
        max_area
    }

    fn close_some<'a, C: Context<'a>, L: Layout>(
        self,
        ctx: C,
        child: LayoutObj<L>,
    );

    fn close_none<'a, C: Context<'a>>(
        self,
        ctx: C,
    );
}

pub fn wrap<'a, W: Wrap, C: Context<'a>>(
    wrap: W,
    mut ctx: C,
) {
    let mut child_socket = None;

    let child_max_area = wrap.open(ctx.max_area());
    ctx.socket(SocketName::default(), &mut child_socket, child_max_area);

    match child_socket {
        Some(child) => wrap.close_some(ctx, child),
        None => wrap.close_none(ctx),
    }
}
