use crate::prelude::*;
use crate::util::linked_queue::{Queue, QueueFiller};

pub trait Panel {
    fn open(&self, max_area: Area) -> Area;

    fn close<'ctx, 'win>(&self, ctx: Context<'ctx, 'win>, id: Id, children: Queue<'win, LayoutNode<'win>>) -> LayoutNode<'win>;
}

pub fn panel<'ctx, 'win, T: Panel>(mut ctx: Context<'ctx, 'win>, id: Id, panel: &T) -> LayoutNode<'win> {
    let child_max_area = panel.open(ctx.max_area());

    let mut children = Queue::default();
    ctx.open_socket(SocketName::default(), child_max_area, &mut QueueFiller::new(&mut children, ctx.buffer()));

    panel.close(ctx, id, children)
}
