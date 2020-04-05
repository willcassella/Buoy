use crate::prelude::*;
use crate::render::{
    commands::{ClickQuad, Quad},
    CommandList,
};

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
            element: Click::new(message),
        }
    }
}

impl Element for Click {
    fn run<'ctx, 'frm>(self, ctx: Context<'ctx, 'frm>, _id: Id) -> LayoutNode<'frm> {
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
    element: Click,
}

impl Builder for ClickBuilder {
    type Element = Click;

    fn get_id(&self) -> Id {
        self.id
    }

    fn get_socket(&self) -> SocketName {
        self.socket
    }

    fn get_element(self) -> Self::Element {
        self.element
    }
}
