use crate::space::*;
use crate::render::CommandList;

pub trait Layout {
    fn render(
        &self,
        region: Region,
        cmds: &mut CommandList
    );
}

// TODO: Remove this and move layout storage into Context
pub struct LayoutObj {
    pub min_area: Area,
    pub imp: Box<dyn Layout>,
}

impl<T> Layout for T where
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

#[derive(Clone, Copy, Debug)]
pub struct NullLayout;

impl Layout for NullLayout {
    fn render(
        &self,
        _region: Region,
        _cmds: &mut CommandList
    ) {
        // NullLayout only takes up space
    }
}
