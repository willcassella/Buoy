use std::f32;
use context::Context;
use layout::{Area, Bounds};
use tree::{Socket, Element};
use command_list::CommandList;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum HAlign {
    Stretch,
    Left,
    Right,
    Center,
}

fn align_horizontally(align: HAlign, bounds: Bounds, mut area: Area) -> Area {
    match align {
        HAlign::Stretch => {
            // Stretch is a no-op
        }
        HAlign::Left => {
            area.width = bounds.width;
        },
        HAlign::Right => {
            area.x = area.x + area.width - bounds.width;
            area.width = bounds.width;
        },
        HAlign::Center => {
            area.x = (area.x + area.width / 2_f32) - bounds.width / 2_f32;
            area.width = bounds.width;
        },
    }

    return area;
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum VAlign {
    Stretch,
    Top,
    Bottom,
    Center,
}

fn align_vertically(align: VAlign, bounds: Bounds, mut area: Area) -> Area {
    match align {
        VAlign::Stretch => {
            // Stretch is a no-op
        },
        VAlign::Top => {
            area.height = bounds.height;
        },
        VAlign::Bottom => {
            area.y = area.y + area.height - bounds.height;
            area.height = bounds.height;
        },
        VAlign::Center => {
            area.y = (area.y + area.height / 2_f32) - bounds.height / 2_f32;
        }
    }

    return area;
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Max {
    pub h_align: HAlign,
    pub v_align: VAlign,
    pub max: Bounds,
}

impl Max {
    pub fn push(self, ctx: &mut Context) {
        ctx.push_socket(Box::new(self));
    }

    pub fn h_align(mut self, v: HAlign) -> Self {
        self.h_align = v;
        self
    }

    pub fn v_align(mut self, v: VAlign) -> Self {
        self.v_align = v;
        self
    }

    pub fn max_width(mut self, v: f32) -> Self {
        self.max.width = v;
        self
    }

    pub fn max_height(mut self, v: f32) -> Self {
        self.max.height = v;
        self
    }
}

impl Default for Max {
    fn default() -> Self {
        Max {
            h_align: HAlign::Stretch,
            v_align: VAlign::Stretch,
            max: Bounds::infinite(),
        }
    }
}

impl Socket for Max {
    fn get_child_max(&self, mut self_max: Bounds) -> Bounds {
        self_max.width = self_max.width.min(self.max.width);
        self_max.height = self_max.height.min(self.max.height);
        return self_max;
    }

    fn child(self: Box<Self>, ctx: &mut Context, child_min: Bounds, child_element: Box<Element>) {
        ctx.element(child_min, Box::new(move |mut area: Area, command_list: &mut CommandList| {
            area = align_horizontally(self.h_align, child_min, area);
            area = align_vertically(self.v_align, child_min, area);
            child_element.render(area, command_list);
        }));
    }
}
