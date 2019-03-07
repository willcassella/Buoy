use crate::layout::{Area, Region};
use crate::render::CommandList;

pub trait RenderImpl {
    fn render(
        &self,
        region: Region,
        cmds: &mut CommandList
    );
}

pub struct Render {
    pub min_area: Area,
    pub imp: Box<dyn RenderImpl>,
}

impl<T> RenderImpl for T where
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
