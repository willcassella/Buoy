use super::archetype;
use crate::basic_renderer::*;
use crate::prelude::*;
use crate::render::CommandList;
use std::f32;
use std::rc::Rc;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Size {
    pub h_align: HAlign,
    pub v_align: VAlign,
    pub max: Area,
    pub min: Area,
}

impl Size {
    pub fn build(id: Id) -> SizeBuilder {
        SizeBuilder {
            id,
            socket: SocketName::default(),
            size: Size::default(),
        }
    }
}

impl Default for Size {
    fn default() -> Self {
        Size {
            h_align: HAlign::Center,
            v_align: VAlign::Center,
            min: Area::zero(),
            max: Area::infinite(),
        }
    }
}

impl Component for Size {
    fn type_id() -> TypeId {
        TypeId::new("buoy", "size")
    }
}

impl Render for Size {
    fn render<'frm, 'thrd, 'ctx>(self, ctx: Context<'frm, 'thrd, 'ctx>) -> LayoutNode<'frm> {
        archetype::wrap(self, ctx)
    }
}

impl archetype::Wrap for Size {
    fn open(&self, mut max_area: Area) -> Area {
        max_area.width = max_area.width.min(self.max.width).max(self.min.width);
        max_area.height = max_area.height.min(self.max.height).max(self.min.height);
        max_area
    }

    fn close_some<'frm, 'thrd, 'ctx>(
        self,
        ctx: Context<'frm, 'thrd, 'ctx>,
        child: LayoutNode<'frm>,
    ) -> LayoutNode<'frm> {
        ctx.new_layout(
            child.min_area.stretch(self.min), // TODO: Handle if child is too big (will require clipping/scrolling)
            move |mut region: Region, cmds: &mut CommandList| {
                if self.max.width < region.area.width {
                    region = self.h_align.align(self.max, region);
                } else if self.min.width > region.area.width {
                    region.area.width = self.min.width;
                }
                if self.max.height < region.area.height {
                    region = self.v_align.align(self.max, region);
                } else if self.min.height > region.area.height {
                    region.area.height = self.min.height;
                }

                child.render(region, cmds);
            },
        )
    }

    fn close_none<'frm, 'thrd, 'ctx>(self, ctx: Context<'frm, 'thrd, 'ctx>) -> LayoutNode<'frm> {
        // Just take up space
        ctx.new_layout(self.min, ())
    }
}

pub struct SizeBuilder {
    id: Id,
    socket: SocketName,
    size: Size,
}

impl SizeBuilder {
    pub fn h_align(mut self, h_align: HAlign) -> Self {
        self.size.h_align = h_align;
        self
    }

    pub fn v_align(mut self, v_align: VAlign) -> Self {
        self.size.v_align = v_align;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.size.min.width = width;
        self.size.max.width = width;
        self
    }

    pub fn min_width(mut self, width: f32) -> Self {
        self.size.min.width = width;
        self
    }

    pub fn max_width(mut self, width: f32) -> Self {
        self.size.max.width = width;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.size.min.height = height;
        self.size.max.height = height;
        self
    }

    pub fn min_height(mut self, height: f32) -> Self {
        self.size.min.height = height;
        self
    }

    pub fn max_height(mut self, height: f32) -> Self {
        self.size.max.height = height;
        self
    }
}

impl Builder<'_> for SizeBuilder {
    type Component = Size;

    fn get_id(&self) -> Id {
        self.id
    }

    fn get_socket(&self) -> SocketName {
        self.socket
    }

    fn get_component(self) -> Self::Component {
        self.size
    }
}

struct RendererFactory;
impl_basic_renderer_factory!(RendererFactory, Size);

pub fn register(window: &mut Window) {
    window.register_component(Size::type_id(), Rc::new(RendererFactory));
}
