use crate::prelude::*;
use crate::render::commands::ColoredQuad;
use crate::render::{color, CommandList};

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct Fill {
    pub color: color::RGBA8,
}

fn generate_quad(color: color::RGBA8, region: Region, cmds: &mut CommandList) {
    cmds.add_colored_quads(&[ColoredQuad::new(From::from(region), color)]);
}

impl Fill {
    pub fn new(color: color::RGBA8) -> Self {
        Fill { color }
    }
}

impl Element for Fill {
    fn run(&self, ctx: Context, id: Id) -> LayoutObj {
        let color = self.color;
        LayoutObj::new(
            Area::zero(),
            move |region: Region, cmds: &mut CommandList| {
                generate_quad(color, region, cmds);
            },
        )
        .upcast()
    }
}
