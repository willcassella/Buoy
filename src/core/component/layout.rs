use crate::render::CommandList;
use crate::space::*;
use crate::util::arena::{ABox, Arena};

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

auto_impl_upcast!(dyn DynLayout);

pub enum LayoutResult {
    Deferred,
    Complete { min_area: Area },
}

impl Layout for () {
    fn render(self, _region: Region, _cmds: &mut CommandList) {
        // NullLayout only takes up space
    }
}

impl<T> Layout for T
where
    T: FnOnce(Region, &mut CommandList),
{
    fn render(self, region: Region, cmds: &mut CommandList) {
        self(region, cmds);
    }
}

pub struct LayoutNode<'a> {
    pub min_area: Area,
    pub layout: ABox<'a, dyn DynLayout + 'a>,
}

impl<'a> LayoutNode<'a> {
    pub fn null(buf: &'a Arena) -> Self {
        LayoutNode {
            min_area: Area::zero(),
            layout: ABox::upcast(buf.alloc(())),
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
