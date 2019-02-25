use crate::layout::{Area, Region};
use crate::render::CommandList;

pub trait UIRenderImpl {
    fn render(
        &self,
        region: Region,
        cmds: &mut CommandList
    );
}

pub struct UIRender {
    pub min_area: Area,
    pub imp: Box<dyn UIRenderImpl>,
}

impl<T> UIRenderImpl for T where
    T: Fn(Region, &mut CommandList)
{
    fn render(
        &self,
        region: Region,
        cmds: &mut CommandList
    ) {
        self(region, cmds);
    }
}
