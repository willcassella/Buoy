use crate::Context;
use crate::layout::Area;
use crate::element::{IntoUIElement, Widget, WidgetObj};
use crate::render::{UIRenderObj, NullUIRender};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Min{
    pub area: Area,
}

impl Min {
    pub fn width(mut self, width: f32) -> Self {
        self.area.width = width;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.area.height = height;
        self
    }
}

impl Default for Min {
    fn default() -> Self {
        Min {
            area: Area::zero(),
        }
    }
}

impl Widget for Min {
    fn open(&self, mut max_area: Area) -> Area {
        max_area.width = max_area.width.max(self.area.width);
        max_area.height = max_area.height.max(self.area.height);
        max_area
    }

    fn close_some(self, ctx: &mut Context, mut child: UIRenderObj) {
        child.min_area.width = child.min_area.width.max(self.area.width);
        child.min_area.height = child.min_area.height.max(self.area.height);
        ctx.render(child);
    }

    fn close_none(self, ctx: &mut Context) {
        ctx.render_new(self.area, Box::new(NullUIRender));
    }
}

impl IntoUIElement for Min {
    type Target = WidgetObj<Min>;
}