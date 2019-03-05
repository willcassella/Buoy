use crate::Context;
use crate::layout::Area;
use crate::element::UIRender;

pub trait Panel {
    fn open(
        &self,
        max_area: Area
    ) -> Area;

    fn close(
        self,
        ctx: &mut Context,
        children: Vec<UIRender>
    );
}

pub fn panel<T: Panel>(
    panel: T,
    ctx: &mut Context,
) {
    let child_max_area = panel.open(ctx.max_area());

    let children = ctx.begin_awaitable_socket(child_max_area, Vec::new());
        ctx.anchor_default();
    ctx.end();

    // Wait for socket to fill up
    ctx.await_sockets(move |ctx: &mut Context| {
        let children = ctx.take_socket(children);
        panel.close(ctx, children);
    });
}
