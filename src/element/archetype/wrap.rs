use crate::Context;
use crate::layout::Area;
use crate::element::UIRender;

pub trait Wrap {
    fn open(&self, max_area: Area) -> Area {
        max_area
    }

    fn close_some(
        self,
        ctx: &mut Context,
        child: UIRender,
    );

    fn close_none(
        self,
        ctx: &mut Context
    );
}

pub fn wrap<T: Wrap>(
    wrap: T,
    ctx: &mut Context,
) {
    let child_max_area = wrap.open(ctx.max_area());

    let socket_ref = ctx.begin_awaitable_socket(child_max_area, None);
        ctx.anchor_default();
    ctx.end();

    // Wait for sockets to fill
    ctx.await_sockets(move |ctx: &mut Context| {
        match ctx.take_socket(socket_ref) {
            Some(child) => wrap.close_some(ctx, child),
            None => wrap.close_none(ctx),
        }
    });
}
