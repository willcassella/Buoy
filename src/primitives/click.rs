use crate::basic_renderer::*;
use crate::prelude::*;
use crate::render::{
    commands::{ClickQuad, Quad},
    CommandList,
};
use crate::util::arena::{ABox, Arena};
use std::rc::Rc;

pub struct Click {
    pub message: Outbox<()>,
}

impl Click {
    pub fn new(message: Outbox<()>) -> Self {
        Click { message }
    }

    pub fn build(id: Id, message: Outbox<()>) -> ClickBuilder {
        ClickBuilder {
            id,
            socket: SocketName::default(),
            click: Click::new(message),
        }
    }
}

impl Component for Click {
    fn type_id() -> TypeId {
        TypeId::new("buoy", "click")
    }
}

impl Render for Click {
    fn render<'frm, 'thrd, 'ctx>(self, ctx: Context<'frm, 'thrd, 'ctx>) -> LayoutNode<'frm> {
        ctx.new_layout(
            Area::zero(),
            move |region: Region, cmds: &mut CommandList| {
                let quad = ClickQuad {
                    quad: Quad::from(region),
                    message: self.message,
                };
                cmds.add_click_quads(std::iter::once(quad));
            },
        )
    }
}

struct ClickRendererFactory;
impl RendererFactory for ClickRendererFactory {
    fn create_renderer<'frm, 'thrd>(
        &self,
        type_id: TypeId,
        buffer: &'thrd Arena,
    ) -> ABox<'thrd, dyn Renderer<'frm>> {
        assert_eq!(Click::type_id(), type_id);
        ABox::upcast(buffer.alloc(BasicRenderer::<Click>::default()))
    }
}

pub fn register(window: &mut Window) {
    window.register_component(Click::type_id(), Rc::new(ClickRendererFactory));
}

pub struct ClickBuilder {
    id: Id,
    socket: SocketName,
    click: Click,
}

impl Builder<'_> for ClickBuilder {
    type Component = Click;

    fn get_id(&self) -> Id {
        self.id
    }

    fn get_socket(&self) -> SocketName {
        self.socket
    }

    fn get_component(self) -> Self::Component {
        self.click
    }
}
