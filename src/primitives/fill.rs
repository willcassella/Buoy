use crate::prelude::*;
use crate::render::commands::ColoredQuad;
use crate::render::{color, CommandList};

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct Fill {
    pub color: color::RGBA8,
}

fn generate_quad(color: color::RGBA8, region: Region, cmds: &mut CommandList) {
    cmds.add_colored_quads(&[ColoredQuad::new(From::from(region), color)]);
}

impl Fill {
    pub fn new(color: color::RGBA8) -> Self {
        Fill { color }
    }

    pub fn build(id: Id) -> FillBuilder {
        FillBuilder {
            id,
            socket: SocketName::default(),
            element: Fill::default(),
        }
    }
}

impl Element for Fill {
    fn run<'ctx, 'frm>(self, ctx: Context<'ctx, 'frm>, _id: Id) -> LayoutNode<'frm> {
        let color = self.color;
        ctx.new_layout(
            Area::zero(),
            move |region: Region, cmds: &mut CommandList| {
                generate_quad(color, region, cmds);
            },
        )
    }
}

pub struct FillBuilder {
    id: Id,
    socket: SocketName,
    element: Fill,
}

impl FillBuilder {
    pub fn socket(mut self, socket: SocketName) -> Self {
        self.socket = socket;
        self
    }

    pub fn color(mut self, color: color::RGBA8) -> Self {
        self.element.color = color;
        self
    }
}

impl Builder for FillBuilder {
    type Element = Fill;

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
