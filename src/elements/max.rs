use std::f32;
use crate::Context;
use crate::layout::{Area, Region};
use crate::element::{IntoUIElement, Widget, WidgetObj};
use crate::render::{UIRenderObj, CommandList};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum HAlign {
    Left,
    Right,
    Center,
}

fn align_horizontally(align: HAlign, area: Area, mut region: Region) -> Region {
    match align {
        HAlign::Left => {
            region.area.width = area.width;
        },
        HAlign::Right => {
            region.pos.x = region.pos.x + region.area.width - area.width;
            region.area.width = area.width;
        },
        HAlign::Center => {
            region.pos.x = (region.pos.x + region.area.width / 2_f32) - area.width / 2_f32;
            region.area.width = area.width;
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

fn align_vertically(align: VAlign, area: Area, mut region: Region) -> Region {
    match align {
        VAlign::Top => {
            region.area.height = area.height;
        },
        VAlign::Bottom => {
            region.pos.y = region.pos.y + region.area.height - area.height;
            region.area.height = area.height;
        },
        VAlign::Center => {
            region.pos.y = (region.pos.y + region.area.height / 2_f32) - area.height / 2_f32;
            region.area.height = area.height;
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

impl Widget for Max {
    fn open(&self, mut max_area: Area) -> Area {
        max_area.width = max_area.width.min(self.area.width);
        max_area.height = max_area.height.min(self.area.height);
        return max_area;
    }

    fn close_some(self, ctx: &mut Context, child: UIRenderObj) {
        ctx.render_new(child.min_area, Box::new(move |mut region: Region, cmds: &mut CommandList| {
            if self.area.width < region.area.width {
                region = align_horizontally(self.h_align, self.area, region);
            }
            if self.area.height < region.area.height {
                region = align_vertically(self.v_align, self.area, region);
            }

            child.render.render(region, cmds);
        }));
    }

    fn close_none(self, _ctx: &mut Context) {
        // Do nothing
    }
}

impl IntoUIElement for Max {
    type Target = WidgetObj<Max>;
}