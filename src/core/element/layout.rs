use crate::space::*;
use crate::render::CommandList;

pub trait Layout: 'static {
    fn render(
        &self,
        region: Region,
        cmds: &mut CommandList
    );

    fn box_upcast(self) -> Box<dyn Layout>
    where
        Self: Sized
    {
        Box::new(self)
    }
}

impl Layout for () {
    fn render(
        &self,
        _region: Region,
        _cmds: &mut CommandList
    ) {
        // NullLayout only takes up space
    }
}

impl Layout for Box<dyn Layout> {
    fn render(
        &self,
        region: Region,
        cmds: &mut CommandList,
    ) {
        self.as_ref().render(region, cmds)
    }

    fn box_upcast(self) -> Box<dyn Layout> {
        self
    }
}

pub struct LayoutObj<T: Layout = Box<dyn Layout>> {
    pub min_area: Area,
    pub imp: T,
}

impl<L: Layout> LayoutObj<L> {
    pub fn new(min_area: Area, layout: L) -> Self {
        LayoutObj {
            min_area,
            imp: layout,
        }
    }
}

impl LayoutObj<()> {
    pub fn null() -> Self {
        LayoutObj {
            min_area: Area::zero(),
            imp: (),
        }
    }
}

impl<T> Layout for T where
    T: Fn(Region, &mut CommandList) + 'static
{
    fn render(
        &self,
        region: Region,
        cmds: &mut CommandList
    ) {
        self(region, cmds);
    }
}
