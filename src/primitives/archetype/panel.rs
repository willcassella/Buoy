use crate::layout::Area;
use crate::core::*;

pub trait Panel {
    fn open(
        &self,
        max_area: Area
    ) -> Area;

    fn close(
        self,
        ctx: &mut Context,
        socket: &mut dyn Socket,
        children: Vec<Render>
    );
}

pub fn panel<T: Panel>(
    panel: T,
    ctx: &mut Context,
    socket: &mut dyn Socket,
) {
    let mut children = Vec::new();

    let child_max_area = panel.open(ctx.max_area());
    while ctx.socket(socket::Id::default(), &mut children, child_max_area) { }

    panel.close(ctx, socket, children);
}
