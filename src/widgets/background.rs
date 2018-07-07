use super::super::context::Context;
use super::super::widget::{Widget, FlexArea};
use super::super::layout{Area, Layout, LayoutHandler};
use super::super::command_list::{Color, Quad, ColoredQuad, CommandList};

#[repr(C)]
#[derive(Default, Clone, Copy, Debug)]
pub struct Background {
    pub color: Color,
}

impl Widget for Background {
    fn run(self: Box<Self>, ctx: &mut Context, bounds: FlexArea) {
        ctx.sync_yield_child(bounds, self);
    }
}

impl LayoutHandler for Background {
    fn handle(self: Box<Self>, ctx: &mut Context, bounds: FlexArea, layout: Box<Layout>) {
        ctx.push_layout(bounds, Box::new(move |area: Area, commands: &mut CommandList| {
            let quad = ColoredQuad::new(Quad::new(area.x, area.y, area.width, area.height), self.color);
            commands.add_colored_quads(&[quad]);
            layout.run(area, commands);
        }));
    }
}
