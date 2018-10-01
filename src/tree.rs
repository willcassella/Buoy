use std::rc::Rc;
use context::{Context, WidgetInfo};
use layout::{Area, Region};
use commands::CommandList;

pub trait Generator {
    fn get_type(&self) -> i32 {
        1
    }

    fn run(self: Box<Self>, ctx: &mut Context);
}

pub trait Socket {
    fn get_child_max(&self, self_max: Area) -> Area {
        self_max
    }

    fn child(
        self: Box<Self>,
        ctx: &mut Context,
        child_min: Area,
        child: Box<Element>
    );

    fn close(self: Box<Self>, _ctx: &mut Context) {
    }
}

pub trait Filter {
    fn generator(&self, alias: &Rc<Filter>, ctx: &mut Context, mut info: WidgetInfo, generator: Box<Generator>) {
        // Make this filter run on the thing we're pushing
        info.add_filter(alias.clone());
        ctx.push_generator(info, generator);
            ctx.children();
        ctx.pop(); // generator
    }

    fn socket(&self, alias: &Rc<Filter>, ctx: &mut Context, mut info: WidgetInfo, socket: Box<Socket>) {
        info.add_filter(alias.clone());
        ctx.push_socket(info, socket);
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