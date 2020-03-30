use crate::prelude::*;
use crate::util::queue::{Queue, QueueFiller};

pub trait Panel {
    fn open(&self, max_area: Area) -> Area;

    fn close<'ctx, 'frm>(
        self,
        id: Id,
        ctx: Context<'ctx, 'frm>,
        children: Queue<'frm, LayoutNode<'frm>>,
    ) -> LayoutNode<'frm>;
}

pub fn panel<'ctx, 'frm, T: Panel>(
    panel: T,
    id: Id,
    mut ctx: Context<'ctx, 'frm>,
) -> LayoutNode<'frm> {
    let child_max_area = panel.open(ctx.max_area());

    let mut children = Queue::default();
    ctx.open_socket(
        SocketName::default(),
        child_max_area,
        &mut QueueFiller::new(&mut children, ctx.buffer()),
    );

    panel.close(id, ctx, children)
}
