use crate::{Context, Wrapper, ElementObj};
use crate::layout::{Region, Area};
use crate::color::Color;
use crate::commands::{CommandList, ColoredQuad};

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
}

impl Fill {
    fn generate_quad(&self, region: Region, cmds: &mut CommandList) {
        cmds.add_colored_quads(&[ColoredQuad::new(From::from(region), self.color)]);
    }
}

impl Wrapper for Fill {
    fn child_element(self: Box<Self>, ctx: &mut Context, child: ElementObj) {
        ctx.new_element(child.min_area, Box::new(move |region: Region, cmds: &mut CommandList| {
            self.generate_quad(region, cmds);
            child.element.render(region, cmds);
        }));
    }

    fn close(self: Box<Self>, ctx: &mut Context) {
        ctx.new_element(Area::zero(), Box::new(move |region: Region, cmds: &mut CommandList| {
            self.generate_quad(region, cmds);
        }));
    }
}