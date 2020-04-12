use crate::basic_renderer::*;
use crate::prelude::*;
use crate::render::{
    commands::{HoverQuad, Quad},
    CommandList,
};
use crate::util::arena::{ABox, Arena};
use std::rc::Rc;

pub struct Hover {
    pub message: Outbox<()>,
}

impl Hover {
    pub fn new(message: Outbox<()>) -> Self {
        Hover { message }
    }

    pub fn build(id: Id, message: Outbox<()>) -> HoverBuilder {
        HoverBuilder {
            id,
            socket: SocketName::default(),
            hover: Hover::new(message),
        }
    }
}

impl Component for Hover {
    fn type_id() -> TypeId {
        TypeId::new("buoy", "hover")
    }
}

impl Render for Hover {
    fn render<'frm, 'thrd, 'ctx>(self, ctx: Context<'frm, 'thrd, 'ctx>) -> LayoutNode<'frm> {
        ctx.new_layout(
            Area::zero(),
            move |region: Region, cmds: &mut CommandList| {
                // Create the hover boundary
                let quad = HoverQuad {
                    quad: Quad::from(region),
                    message: self.message,
                };
                cmds.add_hover_quads(std::iter::once(quad));
            },
        )
    }
}

struct HoverRendererFactory;
impl RendererFactory for HoverRendererFactory {
    fn create_renderer<'frm, 'thrd>(
        &self,
        type_id: TypeId,
        buffer: &'thrd Arena,
    ) -> ABox<'thrd, dyn Renderer<'frm>> {
        assert_eq!(Hover::type_id(), type_id);
        ABox::upcast(buffer.alloc(BasicRenderer::<Hover>::default()))
    }
}

pub fn register(window: &mut Window) {
    window.register_component(Hover::type_id(), Rc::new(HoverRendererFactory));
}

pub struct HoverBuilder {
    id: Id,
    socket: SocketName,
    hover: Hover,
}

impl Builder<'_> for HoverBuilder {
    type Component = Hover;

    fn get_id(&self) -> Id {
        self.id
    }

    fn get_socket(&self) -> SocketName {
        self.socket
    }

    fn get_component(self) -> Self::Component {
        self.hover
    }
}
