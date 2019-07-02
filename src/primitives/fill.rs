use crate::prelude::*;
use crate::render::{CommandList, color};
use crate::render::commands::ColoredQuad;

use super::archetype;

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct Fill {
    pub color: color::RGBA8,
}

impl Fill {
    pub fn new(color: color::RGBA8) -> Self {
        Fill {
            color,
        }
    }

    fn generate_quad(color: color::RGBA8, region: Region, cmds: &mut CommandList) {
        cmds.add_colored_quads(&[ColoredQuad::new(From::from(region), color)]);
    }
}

impl archetype::Wrap for Fill {
    fn close_some<'window, 'ctx, L: Layout>(
        &self,
        ctx: Context<'window, 'ctx>,
        child: LayoutObj<L>,
    ) {
        let color = self.color;
        ctx.layout_new(child.min_area, move |region: Region, cmds: &mut CommandList| {
            Self::generate_quad(color, region, cmds);
            child.imp.render(region, cmds);
        });
    }

    fn close_none<'window, 'ctx>(
        &self,
        ctx: Context<'window, 'ctx>,
    ) {
        let color = self.color;
        ctx.layout_new(Area::zero(), move |region: Region, cmds: &mut CommandList| {
            Self::generate_quad(color, region, cmds);
        });
    }
}

impl Element for Fill {
    fn run<'window, 'ctx>(
        &self,
        ctx: Context<'window, 'ctx>,
    ) {
        archetype::wrap(self, ctx)
    }
}
