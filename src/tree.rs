use layout::{Bounds, Area};
use command_list::CommandList;
use super::context::Context;

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

    fn get_child_max(&self, self_max: Bounds) -> Bounds {
        self_max
    }

    fn child(
        self: Box<Self>,
        ctx: &mut Context,
        child_min: Bounds,
        child_element: Box<Element>
    );

    fn close(self: Box<Self>, ctx: &mut Context) {
    }
}

pub trait Filter {
    fn generator(&self, ctx: &mut Context, generator: Box<Generator>);

    fn socket(&self, ctx: &mut Context, socket: Box<Socket>);
}

pub trait Element {
    fn render(&self, area: Area, command_list: &mut CommandList);
}

impl<T> Element for T where
    T: Fn(Area, &mut CommandList)
{
    fn render(&self, area: Area, command_list: &mut CommandList) {
        self(area, command_list);
    }
}