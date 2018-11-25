use crate::Context;
use crate::layout::{Region, Area};
use crate::element::{IntoUIElement, Widget, WidgetObj};
use crate::render::{UIRenderObj, CommandList, color};
use crate::render::commands::ColoredQuad;

#[repr(C)]
#[derive(Default, Clone, Copy, Debug)]
pub struct SolidFill {
    pub color: color::RGBA8,
}

impl SolidFill {
    pub fn new(color: color::RGBA8) -> Self {
        SolidFill {
            color,
        }
    }

    fn generate_quad(&self, region: Region, cmds: &mut CommandList) {
        cmds.add_colored_quads(&[ColoredQuad::new(From::from(region), self.color)]);
    }
}

impl Widget for SolidFill {
    fn close_some(self, ctx: &mut Context, child: UIRenderObj) {
        ctx.render_new(child.min_area, Box::new(move |region: Region, cmds: &mut CommandList| {
            self.generate_quad(region, cmds);
            child.render.render(region, cmds);
        }));
    }

    fn close_none(self, ctx: &mut Context) {
        ctx.render_new(Area::zero(), Box::new(move |region: Region, cmds: &mut CommandList| {
            self.generate_quad(region, cmds);
        }));
    }
}

impl IntoUIElement for SolidFill {
    type Target = WidgetObj<SolidFill>;
}