use crate::prelude::*;
use crate::render::{CommandList, color};
use crate::render::commands::ColoredQuad;

use super::archetype;

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct SolidFill {
    pub color: color::RGBA8,
}

impl SolidFill {
    pub fn new(color: color::RGBA8) -> Self {
        SolidFill {
            color,
        }
    }

    fn generate_quad(color: color::RGBA8, region: Region, cmds: &mut CommandList) {
        cmds.add_colored_quads(&[ColoredQuad::new(From::from(region), color)]);
    }
}

impl archetype::Wrap for SolidFill {
    fn close_some<'a, C: Context<'a>, L: Layout>(
        self,
        ctx: C,
        child: LayoutObj<L>,
    ) {
        let color = self.color;
        ctx.layout_new(child.min_area, move |region: Region, cmds: &mut CommandList| {
            Self::generate_quad(color, region, cmds);
            child.imp.render(region, cmds);
        });
    }

    fn close_none<'a, C: Context<'a>>(
        self,
        ctx: C,
    ) {
        let color = self.color;
        ctx.layout_new(Area::zero(), move |region: Region, cmds: &mut CommandList| {
            Self::generate_quad(color, region, cmds);
        });
    }
}

impl Element for SolidFill {
    fn run<'a, C: Context<'a>>(
        self,
        ctx: C,
    ) {
        archetype::wrap(self, ctx)
    }
}
