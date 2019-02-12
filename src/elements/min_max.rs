use std::f32;
use crate::Context;
use crate::layout::{Area, Region};
use crate::element::{IntoUIElement, Widget, WidgetImpl};
use crate::render::{UIRender, NullUIRender, CommandList};

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

    region
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

    region
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MinMax {
    pub h_align: HAlign,
    pub v_align: VAlign,
    pub max: Area,
    pub min: Area,
}

impl MinMax {
    pub fn h_align(mut self, h_align: HAlign) -> Self {
        self.h_align = h_align;
        self
    }

    pub fn v_align(mut self, v_align: VAlign) -> Self {
        self.v_align = v_align;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.min.width = width;
        self.max.width = width;
        self
    }

    pub fn min_width(mut self, width: f32) -> Self {
        self.min.width = width;
        self
    }

    pub fn max_width(mut self, width: f32) -> Self {
        self.max.width = width;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.min.height = height;
        self.max.height = height;
        self
    }

    pub fn min_height(mut self, height: f32) -> Self {
        self.min.height = height;
        self
    }

    pub fn max_height(mut self, height: f32) -> Self {
        self.max.height = height;
        self
    }
}

impl Default for MinMax {
    fn default() -> Self {
        MinMax {
            h_align: HAlign::Left,
            v_align: VAlign::Top,
            min: Area::zero(),
            max: Area::infinite(),
        }
    }
}

impl WidgetImpl for MinMax {
    fn open(
        &self,
        mut max_area: Area
    ) -> Area {
        max_area.width = max_area.width.min(self.max.width).max(self.min.width);
        max_area.height = max_area.height.min(self.max.height).max(self.min.height);
        max_area
    }

    fn close_some(
        self,
        ctx: &mut Context,
        child: UIRender
    ) {
        ctx.render_new(child.min_area, Box::new(move |mut region: Region, cmds: &mut CommandList| {
            if self.max.width < region.area.width {
                region = align_horizontally(self.h_align, self.max, region);
            }
            if self.max.height < region.area.height {
                region = align_vertically(self.v_align, self.max, region);
            }

            child.imp.render(region, cmds);
        }));
    }

    fn close_none(
        self,
        ctx: &mut Context
    ) {
        // Just take up space
        if self.min != Area::zero() {
            ctx.render_new(self.min, Box::new(NullUIRender));
        }
    }
}

impl IntoUIElement for MinMax {
    type Target = Widget<MinMax>;
}
