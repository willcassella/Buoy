use crate::prelude::*;

pub trait Wrap {
    fn open(&self, max_area: Area) -> Area {
        max_area
    }

    fn close_some(
        self,
        ctx: &mut Context,
        socket: &mut dyn Socket,
        child: LayoutObj,
    );

    fn close_none(
        self,
        ctx: &mut Context,
        socket: &mut dyn Socket,
    );
}

pub fn wrap<T: Wrap>(
    wrap: T,
    ctx: &mut Context,
    socket: &mut dyn Socket,
) {
    let mut child_socket = None;

    let child_max_area = wrap.open(ctx.max_area());
    ctx.socket(SocketName::default(), &mut child_socket, child_max_area);

    match child_socket {
        Some(child) => wrap.close_some(ctx, socket, child),
        None => wrap.close_none(ctx, socket),
    }
}
