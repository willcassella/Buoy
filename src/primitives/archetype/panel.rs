use crate::prelude::*;

pub trait Panel {
    fn open(
        &self,
        max_area: Area
    ) -> Area;

    fn close<'a, C: Context<'a>>(
        self,
        ctx: C,
        children: Vec<LayoutObj>
    );
}

pub fn panel<'a, T: Panel, C: Context<'a>>(
    panel: T,
    mut ctx: C,
) {
    let mut children = Vec::new();

    let child_max_area = panel.open(ctx.max_area());
    while ctx.socket(SocketName::default(), &mut children, child_max_area) { }

    panel.close(ctx, children)
}
