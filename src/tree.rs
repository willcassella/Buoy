use context::Context;
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

pub trait Filter: Sync + Send + FilterClone {
    fn generator(&self, ctx: &mut Context, generator: Box<Generator>) {
        let self_id = ctx.self_id();
        ctx.push_generator(generator, self_id);
            ctx.children();
        ctx.pop(); // generator
    }

    fn socket(&self, ctx: &mut Context, socket: Box<Socket>) {
        let self_id = ctx.self_id();
        ctx.push_socket(socket, self_id);
            ctx.children();
        ctx.pop(); // socket
    }
}

pub trait FilterClone {
    fn filter_clone(&self) -> Box<Filter>;
}

impl<T> FilterClone for T where
    T: Filter + Clone + 'static
{
    fn filter_clone(&self) -> Box<Filter> {
        Box::new(self.clone())
    }
}

impl Clone for Box<Filter> {
    fn clone(&self) -> Self {
        self.filter_clone()
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