use crate::core::*;
use crate::layout::Region;
use crate::render::CommandList;

#[derive(Clone, Copy)]
pub struct NullUIRender;

impl render::RenderImpl for NullUIRender {
    fn render(
        &self,
        _region: Region,
        _cmds: &mut CommandList
    ) {
        // Null renders only take up space
    }
}
