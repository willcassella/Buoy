use crate::basic_renderer::*;
use crate::prelude::*;
use crate::render::{
    commands::{ClickQuad, Quad},
    CommandList,
};
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

struct RendererFactory;
impl_basic_renderer_factory!(RendererFactory, Click);

pub fn register(window: &mut Window) {
    window.register_component(Click::type_id(), Rc::new(RendererFactory));
}
