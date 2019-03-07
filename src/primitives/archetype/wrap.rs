use crate::Context;
use crate::layout::Area;
use crate::element::{UIRender, UISocket, socket};

pub trait Wrap {
    fn open(&self, max_area: Area) -> Area {
        max_area
    }

    fn close_some(
        self,
        ctx: &mut Context,
        socket: &mut dyn UISocket,
        child: UIRender,
    );

    fn close_none(
        self,
        ctx: &mut Context,
        socket: &mut dyn UISocket,
    );
}

pub fn wrap<T: Wrap>(
    wrap: T,
    ctx: &mut Context,
    socket: &mut dyn UISocket,
) {
    let mut child_socket = None;

    let child_max_area = wrap.open(ctx.max_area());
    ctx.socket(socket::Id::default(), &mut child_socket, child_max_area);

    match child_socket {
        Some(child) => wrap.close_some(ctx, socket, child),
        None => wrap.close_none(ctx, socket),
    }
}
