use crate::layout::Region;
use crate::render::CommandList;
use crate::element::UIRenderImpl;

#[derive(Clone, Copy)]
pub struct NullUIRender;

impl UIRenderImpl for NullUIRender {
    fn render(
        &self,
        _region: Region,
        _cmds: &mut CommandList
    ) {
        // Null renders only take up space
    }
}
