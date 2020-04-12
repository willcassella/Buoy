use crate::basic_renderer::*;
use crate::prelude::*;
use crate::render::commands::ColoredQuad;
use crate::render::{color, CommandList};
use crate::util::arena::{ABox, Arena};
use std::rc::Rc;

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct Fill {
    pub color: color::RGBA8,
}

fn generate_quad(color: color::RGBA8, region: Region, cmds: &mut CommandList) {
    cmds.add_colored_quads(std::iter::once(ColoredQuad::new(From::from(region), color)));
}

impl Fill {
    pub fn new(color: color::RGBA8) -> Self {
        Fill { color }
    }

    pub fn build(id: Id) -> FillBuilder {
        FillBuilder {
            id,
            socket: SocketName::default(),
            fill: Fill::default(),
        }
    }
}

impl Component for Fill {
    fn type_id() -> TypeId {
        TypeId::new("buoy", "fill")
    }
}

impl Render for Fill {
    fn render<'frm, 'thrd, 'ctx>(self, ctx: Context<'frm, 'thrd, 'ctx>) -> LayoutNode<'frm> {
        let color = self.color;
        ctx.new_layout(
            Area::zero(),
            move |region: Region, cmds: &mut CommandList| {
                generate_quad(color, region, cmds);
            },
        )
    }
}

struct FillRendererFactory;
impl RendererFactory for FillRendererFactory {
    fn create_renderer<'frm, 'thrd>(
        &self,
        type_id: TypeId,
        buffer: &'thrd Arena,
    ) -> ABox<'thrd, dyn Renderer<'frm>> {
        assert_eq!(Fill::type_id(), type_id);
        ABox::upcast(buffer.alloc(BasicRenderer::<Fill>::default()))
    }
}

pub fn register(window: &mut Window) {
    window.register_component(Fill::type_id(), Rc::new(FillRendererFactory));
}

pub struct FillBuilder {
    id: Id,
    socket: SocketName,
    fill: Fill,
}

impl FillBuilder {
    pub fn socket(mut self, socket: SocketName) -> Self {
        self.socket = socket;
        self
    }

    pub fn color(mut self, color: color::RGBA8) -> Self {
        self.fill.color = color;
        self
    }
}

impl Builder<'_> for FillBuilder {
    type Component = Fill;

    fn get_id(&self) -> Id {
        self.id
    }

    fn get_socket(&self) -> SocketName {
        self.socket
    }

    fn get_component(self) -> Self::Component {
        self.fill
    }
}
