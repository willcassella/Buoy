use context::Context;
use layout::{Area, Region};
use tree::{Socket, Element, NullElement};
use commands::{CommandList, ColoredQuad, Quad};
use color::{self, Color};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BlockBorder {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub color: Color,
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

    pub fn push(self, ctx: &mut Context) {
        ctx.push_socket(Box::new(self));
    }

    pub fn top(&mut self, top: f32) -> &mut Self {
        self.top = top;
        self
    }

    pub fn bottom(&mut self, bottom: f32) -> &mut Self {
        self.bottom = bottom;
        self
    }

    pub fn left(&mut self, left: f32) -> &mut Self {
        self.left = left;
        self
    }

    pub fn right(&mut self, right: f32) -> &mut Self {
        self.right = right;
        self
    }

    pub fn color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }
}

impl Socket for BlockBorder {
    fn get_child_max(&self, mut self_max: Area) -> Area {
        self_max.width -= self.left + self.right;
        self_max.height -= self.top + self.bottom;
        return self_max;
    }

    fn child(self: Box<Self>, ctx: &mut Context, child_min: Area, child_element: Box<Element>) {
        let mut bounds = child_min;

        // Add to width/height to account for border
        bounds.width += self.left + self.right;
        bounds.height += self.top + self.bottom;

        ctx.element(bounds, Box::new(move |mut region: Region, cmds: &mut CommandList| {
            // Optimization: If we're fully transparent (ie, for padding), don't render anything
            if self.color != color::constants::TRANSPARENT {
                self.generate_commands(region, cmds);
            }

            // Reduce area to account for border
            region.pos.x += self.left;
            region.area.width -= self.left + self.right;
            region.pos.y += self.top;
            region.area.height -= self.top + self.bottom;

            // Render the child element
            child_element.render(region, cmds);
        }))
    }

    fn close(self: Box<Self>, ctx: &mut Context) {
        // Since we don't have a child, bounds are just size of border
        let bounds = Area{ width: self.left + self.right, height: self.top + self.bottom };

        if self.color == color::constants::TRANSPARENT {
            ctx.element(bounds, Box::new(NullElement));
        } else {
            ctx.element(bounds, Box::new(move |region: Region, cmds: &mut CommandList| {
                self.generate_commands(region, cmds);
            }));
        }
    }
}

// So how does BlockBorder work with respect to multiple children?
// One thing that might be useful to consider is how elements like BlockBorder work with respect to duplication
// I think it could be misleading if some elements "duplicate" themselves when wrapped around elements when other's don't...
// Though how else does BlockBorder implement layout for its children?
// I don't think it's possible to implement some sort of max function without having Horizontal/Vertical alignment
// So honest question: Is there value in having elements that by themselves represent potentially more than one element?
//  - I think it could get very confusing

// So elements like BlockBorder generally only make sense with a single child. How do you handle cases where it's given multiple elements?
// 1 - Truncation/error
//      + Easiest solution
//          + Also the best long-term solution, since there may be more complex scenarios where options 2 and 3 aren't feasible
//      - How do you communicate this to the user?
//      - Also might not play nice with some of the filtering stuff I want to support
//      - Also, does this happen at the widget level, or the layout level?
//          - Definately the layout level
// 2 - Just use max width/height
//      + Works
//      - Putting more overhead (and burden!) on implementation of BlockBorder than there needs to be
// 3 - Self-duplication
//      + Similar to solution #1, now you have multiple elements with single children!
//      + Could have some neat implications if explored (worth thinking about)
//      - Confusing for the user, how do you remember which elements duplicate themselves and which elements actually represent "one" element?

// Also, here's how child and parent communicate with eachother:
// - Child receives min/max offered width and height
//      - max width means you can't be wider than this space, useful for doing any sort of wrapping/scaling
//      - min width means you don't need to be smaller than this, useful for shortcutting measurement
//          - Example: Box has fied max width, parent's min is smaller?

// <min v=5>
//  <max v=10>
//      ...
//  </max>
// </min>

// So how should children be communicated?
// Currently my thinking is that children are always communicated to their parents via the stack
// I think that's nice because it facilitates generic communication between widgets in the hierarchy, and hinders potential for making assumptions about the structure of the UI
// (though it doesn't eliminate it entirely)
