use std::f32;
use context::{Context, WidgetInfo};
use layout::{Area, Region};
use tree::{Socket, Element};
use commands::CommandList;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum HAlign {
    Left,
    Right,
    Center,
}

fn align_horizontally(align: HAlign, bounds: Area, mut region: Region) -> Region {
    match align {
        HAlign::Left => {
            region.area.width = bounds.width;
        },
        HAlign::Right => {
            region.pos.x = region.pos.x + region.area.width - bounds.width;
            region.area.width = bounds.width;
        },
        HAlign::Center => {
            region.pos.x = (region.pos.x + region.area.width / 2_f32) - bounds.width / 2_f32;
            region.area.width = bounds.width;
        },
    }

    return region;
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum VAlign {
    Top,
    Bottom,
    Center,
}

fn align_vertically(align: VAlign, bounds: Area, mut region: Region) -> Region {
    match align {
        VAlign::Top => {
            region.area.height = bounds.height;
        },
        VAlign::Bottom => {
            region.pos.y = region.pos.y + region.area.height - bounds.height;
            region.area.height = bounds.height;
        },
        VAlign::Center => {
            region.pos.y = (region.pos.y + region.area.height / 2_f32) - bounds.height / 2_f32;
            region.area.height = bounds.height;
        }
    }

    return region;
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Max {
    pub h_align: HAlign,
    pub v_align: VAlign,
    pub max: Area,
}

impl Max {
    pub fn new() -> Box<Self> {
        Box::new(Self::default())
    }

    pub fn push(self: Box<Self>, ctx: &mut Context, info: WidgetInfo) {
        ctx.push_socket(info, self);
    }

    pub fn h_align(mut self: Box<Self>, v: HAlign) -> Box<Self> {
        self.h_align = v;
        self
    }

    pub fn v_align(mut self: Box<Self>, v: VAlign) -> Box<Self> {
        self.v_align = v;
        self
    }

    pub fn width(mut self: Box<Self>, v: f32) -> Box<Self> {
        self.max.width = v;
        self
    }

    pub fn height(mut self: Box<Self>, v: f32) -> Box<Self> {
        self.max.height = v;
        self
    }
}

impl Default for Max {
    fn default() -> Self {
        Max {
            h_align: HAlign::Left,
            v_align: VAlign::Top,
            max: Area::infinite(),
        }
    }
}

impl Socket for Max {
    fn get_child_max(&self, mut self_max: Area) -> Area {
        self_max.width = self_max.width.min(self.max.width);
        self_max.height = self_max.height.min(self.max.height);
        return self_max;
    }

    fn child(self: Box<Self>, ctx: &mut Context, child_min: Area, child_element: Box<Element>) {
        ctx.element(child_min, Box::new(move |mut region: Region, cmds: &mut CommandList| {
            if self.max.width < region.area.width {
                region = align_horizontally(self.h_align, self.max, region);
            }
            if self.max.height < region.area.height {
                region = align_vertically(self.v_align, self.max, region);
            }

            child_element.render(region, cmds);
        }));
    }
}
