use crate::prelude::*;
use crate::util::queue::{Queue, QueueFiller};

pub trait Panel {
    fn open(&self, max_area: Area) -> Area;

    fn close<'frm, 'thrd, 'ctx>(
        self,
        ctx: Context<'frm, 'thrd, 'ctx>,
        children: Queue<'frm, LayoutNode<'frm>>,
    ) -> LayoutNode<'frm>;
}

pub fn panel<'frm, 'thrd, 'ctx, T: Panel>(
    panel: T,
    mut ctx: Context<'frm, 'thrd, 'ctx>,
) -> LayoutNode<'frm> {
    let child_max_area = panel.open(ctx.max_area());

    let mut children = Queue::default();
    ctx.socket(
        SocketName::default(),
        child_max_area,
        &mut QueueFiller::new(&mut children, ctx.buffer()),
    );

    panel.close(ctx, children)
}
