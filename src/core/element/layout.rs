use crate::render::CommandList;
use crate::space::*;
use crate::util::arena::{Arena, ABox};

pub trait Layout: DynLayout {
    fn render(self, region: Region, cmds: &mut CommandList);
}

pub trait DynLayout {
    unsafe fn render(self: Box<Self>, region: Region, cmds: &mut CommandList);
}

impl<T: Layout> DynLayout for T {
    unsafe fn render(self: Box<Self>, region: Region, cmds: &mut CommandList) {
        let this = std::ptr::read(Box::into_raw(self));
        this.render(region, cmds)
    }
}

impl Layout for () {
    fn render(self, _region: Region, _cmds: &mut CommandList) {
        // NullLayout only takes up space
    }
}

impl<T> Layout for T
where
    T: FnOnce(Region, &mut CommandList)
{
    fn render(self, region: Region, cmds: &mut CommandList) {
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

    pub fn render(mut self, region: Region, cmds: &mut CommandList) {
        unsafe {
            let fake_box = Box::from_raw(&mut *self.layout);
            ABox::forget_inner(self.layout);
            fake_box.render(region, cmds);
        }
    }
}
