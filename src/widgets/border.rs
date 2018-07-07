use context::Context;
use layout::{Area, Bounds};
use tree::{Socket, Element};
use super::super::command_list::{constants, CommandList, Color, Quad, ColoredQuad};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BlockBorder {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub border_color: Color,
}

fn generate_quads(border: BlockBorder, area: Area, commands: &mut CommandList) {
    let top_quad = ColoredQuad::new(Quad::new(area.x, area.y, area.width, border.top), border.border_color);
    let bottom_quad = ColoredQuad::new(Quad::new(area.x, area.y + area.height - border.bottom, area.width, border.bottom), border.border_color);
    let left_quad = ColoredQuad::new(Quad::new(area.x, area.y + border.top, border.left, area.height - border.top - border.bottom), border.border_color);
    let right_quad = ColoredQuad::new(Quad::new(area.x + area.width - border.right, area.y + border.top, border.right, area.height - border.bottom - border.top), border.border_color);
    commands.add_colored_quads(&[top_quad, bottom_quad, left_quad, right_quad]);
}

impl Default for BlockBorder {
    fn default() -> Self {
        BlockBorder {
            left: 0_f32,
            top: 0_f32,
            right: 0_f32,
            bottom: 0_f32,
            border_color: constants::BLACK,
        }
    }
}

impl Socket for BlockBorder {
    fn get_child_max(&self, mut self_max: Bounds) -> Bounds {
        self_max.width -= self.left + self.right;
        self_max.height -= self.top + self.bottom;
        return self_max;
    }

    fn child(self: Box<Self>, ctx: &mut Context, child_min: Bounds, child_element: Box<Element>) {
        let mut bounds = child_min;

        // Add to width/height to account for border
        bounds.width += self.left + self.right;
        bounds.height += self.top + self.bottom;

        ctx.element(bounds, Box::new(move |mut area: Area, commands: &mut CommandList| {
            generate_quads(*self, area, commands);

            // Reduce area to account for border
            area.x += self.left;
            area.width -= self.left + self.right;
            area.y += self.top;
            area.height -= self.top + self.bottom;

            // Render the child element
            child_element.render(area, commands);
        }))
    }

    fn close(self: Box<Self>, ctx: &mut Context) {
        // Since we don't have a child, bounds are just size of border
        let bounds = Bounds{ width: self.left + self.right, height: self.top + self.bottom };
        ctx.element(bounds, Box::new(move |area: Area, commands: &mut CommandList| {
            generate_quads(*self, area, commands)
        }));
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
