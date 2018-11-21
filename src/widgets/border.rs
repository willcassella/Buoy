use crate::{Context, Wrapper, ElementObj, NullElement};
use crate::layout::{Area, Region};
use crate::commands::{CommandList, ColoredQuad, Quad};
use crate::color::{self, Color};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BlockBorder {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub color: Color,
}

impl BlockBorder {
    fn generate_commands(&self, region: Region, cmds: &mut CommandList) {
        let top_quad = ColoredQuad::new(Quad::new(region.pos.x, region.pos.y, region.area.width, self.top), self.color);
        let bottom_quad = ColoredQuad::new(Quad::new(region.pos.x, region.pos.y + region.area.height - self.bottom, region.area.width, self.bottom), self.color);
        let left_quad = ColoredQuad::new(Quad::new(region.pos.x, region.pos.y + self.top, self.left, region.area.height - self.top - self.bottom), self.color);
        let right_quad = ColoredQuad::new(Quad::new(region.pos.x + region.area.width - self.right, region.pos.y + self.top, self.right, region.area.height - self.top - self.bottom), self.color);
        cmds.add_colored_quads(&[top_quad, bottom_quad, left_quad, right_quad]);
    }

    pub fn uniform(size: f32) -> Self {
        BlockBorder {
            left: size,
            top: size,
            right: size,
            bottom: size,
            color: color::constants::BLACK,
        }
    }

    pub fn top(&mut self, top: f32) {
        self.top = top;
    }

    pub fn bottom(&mut self, bottom: f32) {
        self.bottom = bottom;
    }

    pub fn left(&mut self, left: f32) {
        self.left = left;
    }

    pub fn right(&mut self, right: f32) {
        self.right = right;
    }

    pub fn color(&mut self, color: Color) {
        self.color = color;
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

impl Wrapper for BlockBorder {
    fn child_layout(&self, mut self_max: Area) -> Area {
        self_max.width -= self.left + self.right;
        self_max.height -= self.top + self.bottom;
        return self_max;
    }

    fn child_element(self: Box<Self>, ctx: &mut Context, child: ElementObj) {
        let mut bounds = child.min_area;

        // Add to width/height to account for border
        bounds.width += self.left + self.right;
        bounds.height += self.top + self.bottom;

        ctx.new_element(bounds, Box::new(move |mut region: Region, cmds: &mut CommandList| {
            // Unless we're fully transparent, render the border
            if self.color != color::constants::TRANSPARENT {
                self.generate_commands(region, cmds);
            }

            // Reduce area to account for border
            region.pos.x += self.left;
            region.area.width -= self.left + self.right;
            region.pos.y += self.top;
            region.area.height -= self.top + self.bottom;

            // Render the child element
            child.element.render(region, cmds);
        }))
    }

    fn close(self: Box<Self>, ctx: &mut Context) {
        // Since we don't have a child, bounds are just size of border
        let bounds = Area{ width: self.left + self.right, height: self.top + self.bottom };

        if self.color == color::constants::TRANSPARENT {
            ctx.new_element(bounds, Box::new(NullElement));
        } else {
            ctx.new_element(bounds, Box::new(move |region: Region, cmds: &mut CommandList| {
                self.generate_commands(region, cmds);
            }));
        }
    }
}
