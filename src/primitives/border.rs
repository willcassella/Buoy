use super::archetype;
use crate::basic_renderer::*;
use crate::prelude::*;
use crate::render::commands::{ColoredQuad, Quad};
use crate::render::{color, CommandList};
use crate::util::arena::{ABox, Arena};
use std::rc::Rc;

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
    pub fn uniform(size: f32) -> Self {
        Border {
            left: size,
            top: size,
            right: size,
            bottom: size,
            color: color::constants::TRANSPARENT,
        }
    }

    pub fn build(id: Id) -> BorderBuilder {
        BorderBuilder {
            id,
            socket: SocketName::default(),
            border: Default::default(),
        }
    }

    fn generate_commands(&self, region: Region, cmds: &mut CommandList) {
        let top_quad = ColoredQuad::new(
            Quad::new(region.pos.x, region.pos.y, region.area.width, self.top),
            self.color,
        );
        let bottom_quad = ColoredQuad::new(
            Quad::new(
                region.pos.x,
                region.pos.y + region.area.height - self.bottom,
                region.area.width,
                self.bottom,
            ),
            self.color,
        );
        let left_quad = ColoredQuad::new(
            Quad::new(
                region.pos.x,
                region.pos.y + self.top,
                self.left,
                region.area.height - self.top - self.bottom,
            ),
            self.color,
        );
        let right_quad = ColoredQuad::new(
            Quad::new(
                region.pos.x + region.area.width - self.right,
                region.pos.y + self.top,
                self.right,
                region.area.height - self.top - self.bottom,
            ),
            self.color,
        );
        cmds.add_colored_quads(
            [top_quad, bottom_quad, left_quad, right_quad]
                .iter()
                .cloned(),
        );
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

impl Component for Border {
    fn type_id() -> TypeId {
        TypeId::new("buoy", "border")
    }
}

impl Render for Border {
    fn render<'frm, 'thrd, 'ctx>(self, ctx: Context<'frm, 'thrd, 'ctx>) -> LayoutNode<'frm> {
        archetype::wrap(self, ctx)
    }
}

struct BorderRendererFactory;
impl RendererFactory for BorderRendererFactory {
    fn create_renderer<'frm, 'thrd>(
        &self,
        type_id: TypeId,
        buffer: &'thrd Arena,
    ) -> ABox<'thrd, dyn Renderer<'frm>> {
        assert_eq!(type_id, Border::type_id());
        ABox::upcast(buffer.alloc(BasicRenderer::<Border>::default()))
    }
}

pub fn register(window: &mut Window) {
    window.register_component(Border::type_id(), Rc::new(BorderRendererFactory));
}

impl archetype::Wrap for Border {
    fn open(&self, mut max_area: Area) -> Area {
        max_area.width -= self.left + self.right;
        max_area.height -= self.top + self.bottom;

        max_area
    }

    fn close_some<'frm, 'thrd, 'ctx>(
        self,
        ctx: Context<'frm, 'thrd, 'ctx>,
        child: LayoutNode<'frm>,
    ) -> LayoutNode<'frm> {
        let border = self;
        let mut min_area = child.min_area;

        // Add to width/height to account for border
        min_area.width += border.left + border.right;
        min_area.height += border.top + border.bottom;

        ctx.new_layout(
            min_area,
            move |mut region: Region, cmds: &mut CommandList| {
                // Unless we're fully transparent, render the border
                if border.color != color::constants::TRANSPARENT {
                    border.generate_commands(region, cmds);
                }

                // Reduce area to account for border
                region.pos.x += border.left;
                region.area.width -= border.left + border.right;
                region.pos.y += border.top;
                region.area.height -= border.top + border.bottom;

                // Render the child
                child.render(region, cmds);
            },
        )
    }

    fn close_none<'frm, 'thrd, 'ctx>(self, ctx: Context<'frm, 'thrd, 'ctx>) -> LayoutNode<'frm> {
        // Since we don't have a child, min area is just size of border
        let min_area = Area {
            width: self.left + self.right,
            height: self.top + self.bottom,
        };

        if self.color == color::constants::TRANSPARENT {
            ctx.new_layout(min_area, ())
        } else {
            ctx.new_layout(min_area, move |region: Region, cmds: &mut CommandList| {
                self.generate_commands(region, cmds);
            })
        }
    }
}

pub struct BorderBuilder {
    id: Id,
    socket: SocketName,
    border: Border,
}

impl BorderBuilder {
    pub fn uniform(mut self, size: f32) -> Self {
        self.border = Border::uniform(size);
        self
    }

    pub fn socket(mut self, socket: SocketName) -> Self {
        self.socket = socket;
        self
    }

    pub fn top(mut self, top: f32) -> Self {
        self.border.top = top;
        self
    }

    pub fn bottom(mut self, bottom: f32) -> Self {
        self.border.bottom = bottom;
        self
    }

    pub fn left(mut self, left: f32) -> Self {
        self.border.left = left;
        self
    }

    pub fn right(mut self, right: f32) -> Self {
        self.border.right = right;
        self
    }

    pub fn color(mut self, color: color::RGBA8) -> Self {
        self.border.color = color;
        self
    }
}

impl Builder<'_> for BorderBuilder {
    type Component = Border;

    fn get_id(&self) -> Id {
        self.id
    }

    fn get_socket(&self) -> SocketName {
        self.socket
    }

    fn get_component(self) -> Self::Component {
        self.border
    }
}
