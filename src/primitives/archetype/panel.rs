use crate::prelude::*;
use crate::util::queue::{Queue, QueueFiller};

pub trait Panel {
    fn open(&self, max_area: Area) -> Area;

    fn close<'ctx, 'frm>(&self, ctx: Context<'ctx, 'frm>, id: Id, children: Queue<'frm, LayoutNode<'frm>>) -> LayoutNode<'frm>;
}

pub fn panel<'ctx, 'frm, T: Panel>(mut ctx: Context<'ctx, 'frm>, id: Id, panel: &T) -> LayoutNode<'frm> {
    let child_max_area = panel.open(ctx.max_area());

    let mut children = Queue::default();
    ctx.open_socket(SocketName::default(), child_max_area, &mut QueueFiller::new(&mut children, ctx.buffer()));

    panel.close(ctx, id, children)
}
