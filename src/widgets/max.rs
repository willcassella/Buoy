use std::f32;
use crate::{Context, Wrapper, WrapperObj, WidgetType, ElementObj};
use crate::layout::{Area, Region};
use crate::commands::CommandList;

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
    pub area: Area,
}

impl Max {
    pub fn h_align(mut self, v: HAlign) -> Self {
        self.h_align = v;
        self
    }

    pub fn v_align(mut self, v: VAlign) -> Self {
        self.v_align = v;
        self
    }

    pub fn width(mut self, v: f32) -> Self {
        self.area.width = v;
        self
    }

    pub fn height(mut self, v: f32) -> Self {
        self.area.height = v;
        self
    }
}

impl Default for Max {
    fn default() -> Self {
        Max {
            h_align: HAlign::Left,
            v_align: VAlign::Top,
            area: Area::infinite(),
        }
    }
}

impl Wrapper for Max {
    fn open(&self, mut self_bounds: Area) -> Area {
        self_bounds.width = self_bounds.width.min(self.area.width);
        self_bounds.height = self_bounds.height.min(self.area.height);
        return self_bounds;
    }

    fn close_some(self, ctx: &mut Context, child: ElementObj) {
        ctx.new_element(child.min_area, Box::new(move |mut region: Region, cmds: &mut CommandList| {
            if self.area.width < region.area.width {
                region = align_horizontally(self.h_align, self.area, region);
            }
            if self.area.height < region.area.height {
                region = align_vertically(self.v_align, self.area, region);
            }

            child.element.render(region, cmds);
        }));
    }

    fn close_none(self, _ctx: &mut Context) {
        // Do nothing
    }
}

impl WidgetType for Max {
    type Target = WrapperObj<Max>;
}