use context::{Context, WidgetId};
use layout::Area;
use Socket;
use Element;
use NullElement;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Min {
    pub min: Area,
}

impl Min {
    pub fn width(&mut self, width: f32) -> &mut Self {
        self.min.width = width;
        self
    }

    pub fn height(&mut self, height: f32) -> &mut Self {
        self.min.height = height;
        self
    }

    pub fn push(self, ctx: &mut Context, id: WidgetId) {
        ctx.push_socket(Box::new(self), id);
    }
}

impl Default for Min {
    fn default() -> Self {
        Min {
            min: Area::zero(),
        }
    }
}

impl Socket for Min {
    fn get_child_max(&self, mut self_max: Area) -> Area {
        self_max.width = self_max.width.max(self.min.width);
        self_max.height = self_max.height.max(self.min.height);
        self_max
    }

    fn child(self: Box<Self>, ctx: &mut Context, mut child_min: Area, child_element: Box<Element>) {
        child_min.width = child_min.width.max(self.min.width);
        child_min.height = child_min.width.max(self.min.height);
        ctx.element(child_min, child_element);
    }

    fn close(self: Box<Self>, ctx: &mut Context) {
        ctx.element(self.min, Box::new(NullElement));
    }
}