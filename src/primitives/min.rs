use std::f32;

use crate::prelude::*;
use crate::render::CommandList;

use super::archetype;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Space {
    pub h_align: HAlign,
    pub v_align: VAlign,
    pub max: Area,
    pub min: Area,
}

impl Space {
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

impl Default for Space {
    fn default() -> Self {
        Space {
            h_align: HAlign::Left,
            v_align: VAlign::Top,
            min: Area::zero(),
            max: Area::infinite(),
        }
    }
}

impl Element for Space {
    fn run<'a, C: Context<'a>>(
        self,
        ctx: C,
    ) {
        archetype::wrap(self, ctx)
    }
}

impl archetype::Wrap for Space {
    fn open(
        &self,
        mut max_area: Area
    ) -> Area {
        max_area.width = max_area.width.min(self.max.width).max(self.min.width);
        max_area.height = max_area.height.min(self.max.height).max(self.min.height);
        max_area
    }

    fn close_some<'a, C: Context<'a>, L: Layout>(
        self,
        ctx: C,
        child: LayoutObj<L>,
    ) {
        ctx.layout_new(child.min_area, move |mut region: Region, cmds: &mut CommandList| {
            if self.max.width < region.area.width {
                region = self.h_align.align(self.max, region);
            }
            if self.max.height < region.area.height {
                region = self.v_align.align(self.max, region);
            }

            child.imp.render(region, cmds);
        });
    }

    fn close_none<'a, C: Context<'a>>(
        self,
        ctx: C,
    ) {
        // Just take up space
        if self.min != Area::zero() {
            ctx.layout_new(self.min, ());
        }
    }
}
