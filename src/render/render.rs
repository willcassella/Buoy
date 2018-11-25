use crate::layout::{Area, Region};
use crate::render::CommandList;

pub trait UIRender {
    fn render(&self, region: Region, cmds: &mut CommandList);
}

pub struct UIRenderObj {
    pub min_area: Area,
    pub render: Box<UIRender>,
}

impl<T> UIRender for T where
    T: Fn(Region, &mut CommandList)
{
    fn render(&self, region: Region, cmds: &mut CommandList) {
        self(region, cmds);
    }
}

#[derive(Clone, Copy)]
pub struct NullUIRender;

impl UIRender for NullUIRender {
    fn render(&self, _region: Region, _cmds: &mut CommandList) {
        // Null renders only take up space
    }
}