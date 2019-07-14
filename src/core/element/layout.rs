use crate::render::CommandList;
use crate::space::*;
use crate::util::linked_buffer::{LinkedBuffer, LBBox};

pub trait Layout {
    fn render(&self, region: Region, cmds: &mut CommandList);
}

impl Layout for () {
    fn render(&self, _region: Region, _cmds: &mut CommandList) {
        // NullLayout only takes up space
    }
}

impl<T> Layout for T
where
    T: Fn(Region, &mut CommandList)
{
    fn render(&self, region: Region, cmds: &mut CommandList) {
        self(region, cmds);
    }
}

pub struct LayoutNode<'win> {
    pub min_area: Area,
    pub layout: LBBox<'win, dyn Layout + 'win>,
}

impl<'win> LayoutNode<'win> {
    pub fn null(buf: &'win LinkedBuffer) -> Self {
        LayoutNode {
            min_area: Area::zero(),
            layout: buf.alloc(()).unsize(),
        }
    }
}
