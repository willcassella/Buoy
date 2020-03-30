use crate::prelude::*;
use crate::render::{
    commands::{HoverQuad, Quad},
    CommandList,
};

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
            element: Hover::new(message),
        }
    }
}

impl Element for Hover {
    fn run<'ctx, 'frm>(self, ctx: Context<'ctx, 'frm>, _id: Id) -> LayoutNode<'frm> {
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

pub struct HoverBuilder {
    id: Id,
    socket: SocketName,
    element: Hover,
}

impl Builder for HoverBuilder {
    type Element = Hover;

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
