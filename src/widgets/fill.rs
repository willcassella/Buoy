use context::{Context, WidgetInfo};
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
    pub fn new(color: Color) -> Box<Self> {
        Box::new(Fill {
            color,
        })
    }

    pub fn push(self: Box<Self>, ctx: &mut Context, info: WidgetInfo) {
        ctx.push_socket(info, self);
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