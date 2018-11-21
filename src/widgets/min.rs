use crate::{Context, Wrapper, ElementObj, NullElement};
use crate::layout::Area;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Min{
    pub area: Area,
}

impl Min {
    pub fn width(&mut self, width: f32) {
        self.area.width = width;
    }

    pub fn height(&mut self, height: f32) {
        self.area.height = height;
    }
}

impl Default for Min {
    fn default() -> Self {
        Min {
            area: Area::zero(),
        }
    }
}

impl Wrapper for Min {
    fn child_layout(&self, mut self_max: Area) -> Area {
        self_max.width = self_max.width.max(self.area.width);
        self_max.height = self_max.height.max(self.area.height);
        self_max
    }

    fn child_element(self: Box<Self>, ctx: &mut Context, mut child: ElementObj) {
        child.min_area.width = child.min_area.width.max(self.area.width);
        child.min_area.height = child.min_area.height.max(self.area.height);
        ctx.element(child);
    }

    fn close(self: Box<Self>, ctx: &mut Context) {
        ctx.new_element(self.area, Box::new(NullElement));
    }
}