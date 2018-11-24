use crate::layout::{Area, Region};
use crate::commands::CommandList;

pub trait Element {
    fn render(&self, region: Region, cmds: &mut CommandList);
}

pub struct ElementObj {
    pub min_area: Area,
    pub element: Box<Element>,
}

impl<T> Element for T where
    T: Fn(Region, &mut CommandList)
{
    fn render(&self, region: Region, cmds: &mut CommandList) {
        self(region, cmds);
    }
}

#[derive(Clone, Copy)]
pub struct NullElement;

impl Element for NullElement {
    fn render(&self, _region: Region, _cmds: &mut CommandList) {
        // Null elements only take up space
    }
}