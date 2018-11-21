use std::rc::Rc;
use std::u32;
use context::{Context, WidgetInfo};
use layout::{Area, Region};
use commands::CommandList;

pub struct Child {
    pub element: Box<Element>,
    pub min_area: Area,
}

pub trait Widget {
    fn layout_children(&self, self_max: Area) -> (u32, Area) {
        (u32::MAX, self_max)
    }

    fn children(
        self: Box<Self>,
        ctx: &mut Context,
        children: Vec<Child>,
    );
}

pub trait Filter {
    fn run(&self, alias: &Rc<Filter>, ctx: &mut Context, mut info: WidgetInfo, widget: Box<Widget>) {
        info.attach_filter(alias.clone());
        ctx.push_widget(info, widget);
            ctx.children();
        ctx.pop(); // socket
    }
}

pub trait Element {
    fn render(&self, region: Region, cmds: &mut CommandList);
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