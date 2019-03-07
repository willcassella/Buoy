use crate::Context;
use crate::layout::{Region, Area};
use crate::render::{CommandList, color};
use crate::render::commands::ColoredQuad;
use crate::element::{UIWidgetImpl, UISocket, UIRender};
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
    fn close_some(
        self,
        ctx: &mut Context,
        socket: &mut dyn UISocket,
        child: UIRender,
    ) {
        let color = self.color;
        ctx.render_new(socket, child.min_area, move |region: Region, cmds: &mut CommandList| {
            Self::generate_quad(color, region, cmds);
            child.imp.render(region, cmds);
        });
    }

    fn close_none(
        self,
        ctx: &mut Context,
        socket: &mut dyn UISocket,
    ) {
        let color = self.color;
        ctx.render_new(socket, Area::zero(), move |region: Region, cmds: &mut CommandList| {
            Self::generate_quad(color, region, cmds);
        });
    }
}

impl UIWidgetImpl for SolidFill {
    type Next = ();

    fn run(
        mut self,
        ctx: &mut Context,
        socket: &mut dyn UISocket,
    ) -> Option<Self::Next> {
        archetype::wrap(self, ctx, socket);
        None
    }
}
