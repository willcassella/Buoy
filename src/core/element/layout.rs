use crate::render::CommandList;
use crate::space::*;
use crate::util::arena::{Arena, ABox};

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

pub struct LayoutNode<'frm> {
    pub min_area: Area,
    pub layout: ABox<'frm, dyn Layout + 'frm>,
}

impl<'frm> LayoutNode<'frm> {
    pub fn null(buf: &'frm Arena) -> Self {
        LayoutNode {
            min_area: Area::zero(),
            layout: buf.alloc(()).unsize(),
        }
    }
}
