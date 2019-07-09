use std::f32;

use crate::prelude::*;
use crate::render::CommandList;

use super::archetype;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Size {
    pub h_align: HAlign,
    pub v_align: VAlign,
    pub max: Area,
    pub min: Area,
}

impl Size {
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

impl Default for Size {
    fn default() -> Self {
        Size {
            h_align: HAlign::Left,
            v_align: VAlign::Top,
            min: Area::zero(),
            max: Area::infinite(),
        }
    }
}

impl Element for Size {
    fn run(&self, ctx: Context, id: Id) -> LayoutObj {
        archetype::wrap(ctx, id, self)
    }
}

impl archetype::Wrap for Size {
    fn open(&self, mut max_area: Area) -> Area {
        max_area.width = max_area.width.min(self.max.width).max(self.min.width);
        max_area.height = max_area.height.min(self.max.height).max(self.min.height);
        max_area
    }

    fn close_some<L: Layout>(&self, _ctx: Context, _id: Id, child: LayoutObj<L>) -> LayoutObj {
        let this = *self;

        LayoutObj::new(
            child.min_area.stretch(&this.min), // TODO: Handle if child is too big (will require clipping/scrolling)
            move |mut region: Region, cmds: &mut CommandList| {
                if this.max.width < region.area.width {
                    region = this.h_align.align(this.max, region);
                } else if this.min.width > region.area.width {
                    region.area.width = this.min.width;
                }
                if this.max.height < region.area.height {
                    region = this.v_align.align(this.max, region);
                } else if this.min.height > region.area.height {
                    region.area.height = this.min.height;
                }

                child.imp.render(region, cmds);
            },
        )
        .upcast()
    }

    fn close_none(&self, _ctx: Context, _id: Id) -> LayoutObj {
        // Just take up space
        LayoutObj::new(self.min, ()).upcast()
    }
}
