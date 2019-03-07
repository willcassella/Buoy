use crate::Context;
use crate::layout::Area;
use crate::element::{UIRender, UISocket, socket};

pub trait Panel {
    fn open(
        &self,
        max_area: Area
    ) -> Area;

    fn close(
        self,
        ctx: &mut Context,
        socket: &mut dyn UISocket,
        children: Vec<UIRender>
    );
}

pub fn panel<T: Panel>(
    panel: T,
    ctx: &mut Context,
    socket: &mut dyn UISocket,
) {
    let mut children = Vec::new();

    let child_max_area = panel.open(ctx.max_area());
    while ctx.socket(socket::Id::default(), &mut children, child_max_area) { }

    panel.close(ctx, socket, children);
}
