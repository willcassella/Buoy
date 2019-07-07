use crate::prelude::*;
use crate::render::{CommandList, color};
use crate::render::commands::{Quad, ColoredQuad};

use super::archetype;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Border {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub color: color::RGBA8,
}

impl Border {
    fn generate_commands(&self, region: Region, cmds: &mut CommandList) {
        let top_quad = ColoredQuad::new(Quad::new(region.pos.x, region.pos.y, region.area.width, self.top), self.color);
        let bottom_quad = ColoredQuad::new(Quad::new(region.pos.x, region.pos.y + region.area.height - self.bottom, region.area.width, self.bottom), self.color);
        let left_quad = ColoredQuad::new(Quad::new(region.pos.x, region.pos.y + self.top, self.left, region.area.height - self.top - self.bottom), self.color);
        let right_quad = ColoredQuad::new(Quad::new(region.pos.x + region.area.width - self.right, region.pos.y + self.top, self.right, region.area.height - self.top - self.bottom), self.color);
        cmds.add_colored_quads(&[top_quad, bottom_quad, left_quad, right_quad]);
    }
}

impl Border {
    pub fn uniform(size: f32) -> Self {
        Border {
            left: size,
            top: size,
            right: size,
            bottom: size,
            color: color::constants::BLACK,
        }
    }
}

impl Border {
    pub fn top(mut self, top: f32) -> Self {
        self.top = top;
        self
    }

    pub fn bottom(mut self, bottom: f32) -> Self {
        self.bottom = bottom;
        self
    }

    pub fn left(mut self, left: f32) -> Self {
        self.left = left;
        self
    }

    pub fn right(mut self, right: f32) -> Self {
        self.right = right;
        self
    }

    pub fn color(mut self, color: color::RGBA8) -> Self {
        self.color = color;
        self
    }
}

impl Default for Border {
    fn default() -> Self {
        Border {
            left: 0_f32,
            top: 0_f32,
            right: 0_f32,
            bottom: 0_f32,
            color: color::constants::TRANSPARENT,
        }
    }
}

impl Element for Border {
    fn run(
        &self,
        ctx: Context,
        id: Id,
    ) -> LayoutObj {
        archetype::wrap(ctx, id, self)
    }
}

impl archetype::Wrap for Border {
    fn open(&self, mut max_area: Area) -> Area {
        max_area.width -= self.left + self.right;
        max_area.height -= self.top + self.bottom;

        max_area
    }

    fn close_some<L: Layout>(
        &self,
        _ctx: Context,
        _id: Id,
        child: LayoutObj<L>,
    ) -> LayoutObj {
        let mut min_area = child.min_area;

        // Add to width/height to account for border
        min_area.width += self.left + self.right;
        min_area.height += self.top + self.bottom;

        let this = self.clone();
        return LayoutObj::new(min_area, move |mut region: Region, cmds: &mut CommandList| {
            // Unless we're fully transparent, render the border
            if this.color != color::constants::TRANSPARENT {
                this.generate_commands(region, cmds);
            }

            // Reduce area to account for border
            region.pos.x += this.left;
            region.area.width -= this.left + this.right;
            region.pos.y += this.top;
            region.area.height -= this.top + this.bottom;

            // Render the child
            child.imp.render(region, cmds);
        }).upcast();
    }

    fn close_none(
        &self,
        _ctx: Context,
        _id: Id,
    ) -> LayoutObj {
        // Since we don't have a child, min area is just size of border
        let min_area = Area{ width: self.left + self.right, height: self.top + self.bottom };

        if self.color == color::constants::TRANSPARENT {
            return LayoutObj::new(min_area, ()).upcast().upcast();
        } else {
            let this = self.clone();
            return LayoutObj::new(min_area, move |region: Region, cmds: &mut CommandList| {
                this.generate_commands(region, cmds);
            }).upcast();
        }
    }
}
