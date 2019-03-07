use crate::layout::{Area, Region};
use crate::render::{CommandList, color};
use crate::render::commands::{Quad, ColoredQuad};
use crate::core::*;

use super::{archetype, null_render::NullUIRender};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct BlockBorder {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub color: color::RGBA8,
}

impl BlockBorder {
    fn generate_commands(&self, region: Region, cmds: &mut CommandList) {
        let top_quad = ColoredQuad::new(Quad::new(region.pos.x, region.pos.y, region.area.width, self.top), self.color);
        let bottom_quad = ColoredQuad::new(Quad::new(region.pos.x, region.pos.y + region.area.height - self.bottom, region.area.width, self.bottom), self.color);
        let left_quad = ColoredQuad::new(Quad::new(region.pos.x, region.pos.y + self.top, self.left, region.area.height - self.top - self.bottom), self.color);
        let right_quad = ColoredQuad::new(Quad::new(region.pos.x + region.area.width - self.right, region.pos.y + self.top, self.right, region.area.height - self.top - self.bottom), self.color);
        cmds.add_colored_quads(&[top_quad, bottom_quad, left_quad, right_quad]);
    }
}

impl BlockBorder {
    pub fn uniform(size: f32) -> Self {
        BlockBorder {
            left: size,
            top: size,
            right: size,
            bottom: size,
            color: color::constants::BLACK,
        }
    }
}

impl BlockBorder {
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

impl Default for BlockBorder {
    fn default() -> Self {
        BlockBorder {
            left: 0_f32,
            top: 0_f32,
            right: 0_f32,
            bottom: 0_f32,
            color: color::constants::TRANSPARENT,
        }
    }
}

impl Element for BlockBorder {
    type Next = ();

    fn run(
        mut self,
        ctx: &mut Context,
        socket: &mut dyn Socket
    ) -> Option<Self::Next> {
        archetype::wrap(self, ctx, socket);
        None
    }
}

impl archetype::Wrap for BlockBorder {
    fn open(&self, mut max_area: Area) -> Area {
        max_area.width -= self.left + self.right;
        max_area.height -= self.top + self.bottom;

        max_area
    }

    fn close_some(
        self,
        ctx: &mut Context,
        socket: &mut dyn Socket,
        child: Render,
    ) {
        let mut min_area = child.min_area;

        // Add to width/height to account for border
        min_area.width += self.left + self.right;
        min_area.height += self.top + self.bottom;

        ctx.render_new(socket, min_area, move |mut region: Region, cmds: &mut CommandList| {
            // Unless we're fully transparent, render the border
            if self.color != color::constants::TRANSPARENT {
                self.generate_commands(region, cmds);
            }

            // Reduce area to account for border
            region.pos.x += self.left;
            region.area.width -= self.left + self.right;
            region.pos.y += self.top;
            region.area.height -= self.top + self.bottom;

            // Render the child
            child.imp.render(region, cmds);
        })
    }

    fn close_none(
        self,
        ctx: &mut Context,
        socket: &mut dyn Socket,
    ) {
        // Since we don't have a child, min area is just size of border
        let min_area = Area{ width: self.left + self.right, height: self.top + self.bottom };

        if self.color == color::constants::TRANSPARENT {
            ctx.render_new(socket, min_area, NullUIRender);
        } else {
            ctx.render_new(socket, min_area, move |region: Region, cmds: &mut CommandList| {
                self.generate_commands(region, cmds);
            });
        }
    }
}
