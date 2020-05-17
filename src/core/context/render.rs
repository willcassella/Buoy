use crate::core::context::{GuiContext, ThreadContext};
use crate::space::Region;
use crate::LayoutNode;

pub struct RenderContext<'slf, 'frm, C> {
    pub(crate) region: Region,
    pub(crate) gui_ctx: &'frm GuiContext<C>,
    pub(crate) thread_ctx: &'slf ThreadContext<'frm, C>,
}

impl<'slf, 'frm, C: 'static> RenderContext<'slf, 'frm, C> {
    pub fn render(&self, node: LayoutNode, region: Region, canvas: &mut C) {
        // Get the renderer for this node
        let renderer = self.thread_ctx.renderer_for(self.gui_ctx, node.type_id);

        // Create a render context
        let ctx = RenderContext {
            region,
            gui_ctx: self.gui_ctx,
            thread_ctx: self.thread_ctx,
        };

        renderer.render(node.index, ctx, canvas);
    }

    pub fn region(&self) -> Region {
        self.region
    }
}
