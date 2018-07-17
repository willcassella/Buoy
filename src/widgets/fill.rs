use context::{Context, WidgetId};
use tree::{Socket, Element};
use layout::{Region, Area};
use color::Color;
use commands::{CommandList, ColoredQuad};

#[repr(C)]
#[derive(Default, Clone, Copy, Debug)]
pub struct Fill {
    pub color: Color,
}

impl Fill {
    pub fn new(color: Color) -> Self {
        Fill {
            color,
        }
    }

    pub fn push(self, ctx: &mut Context, id: WidgetId) {
        ctx.push_socket(Box::new(self), id);
    }
}

impl Fill {
    fn generate_quad(&self, region: Region, cmds: &mut CommandList) {
        cmds.add_colored_quads(&[ColoredQuad::new(From::from(region), self.color)]);
    }
}

impl Socket for Fill {
    fn child(self: Box<Self>, ctx: &mut Context, child_min: Area, child_element: Box<Element>) {
        ctx.element(child_min, Box::new(move |region: Region, cmds: &mut CommandList| {
            self.generate_quad(region, cmds);
            child_element.render(region, cmds);
        }));
    }
}