use crate::core::context::{GuiContext, ThreadContext};
use crate::space::*;
use crate::LayoutNode;

pub struct RenderContext<'slf, 'frm, C, S: Space> {
    pub(in crate::core) region: Region<S>,
    pub(in crate::core) gui_ctx: &'frm GuiContext<C>,
    pub(in crate::core) thread_ctx: &'slf ThreadContext<'frm>,
}

impl<'slf, 'frm, C: 'static, S: Space> RenderContext<'slf, 'frm, C, S> {
    pub fn render(&self, node: LayoutNode<C, S>, region: Region<S>, canvas: &mut C) {
        unimplemented!()
    }

    pub fn region(&self) -> Region<S> {
        self.region
    }
}
