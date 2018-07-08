use layout::{Area, Region};
use commands::CommandList;
use context::Context;

pub trait Generator {
    fn get_type(&self) -> i32 {
        1
    }

    fn run(self: Box<Self>, ctx: &mut Context);
}

pub trait Socket {
    fn get_type(&self) -> i32 {
        1
    }

    fn get_child_max(&self, self_max: Area) -> Area {
        self_max
    }

    fn child(
        self: Box<Self>,
        ctx: &mut Context,
        child_min: Area,
        child_element: Box<Element>
    );

    fn close(self: Box<Self>, _ctx: &mut Context) {
    }
}

pub trait Filter {
    fn generator(&self, ctx: &mut Context, generator: Box<Generator>);

    fn socket(&self, ctx: &mut Context, socket: Box<Socket>);
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