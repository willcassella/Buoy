use crate::prelude::*;
use crate::render::CommandList;
use super::archetype;

use crate::util::queue::Queue;

pub struct Overlap;

impl Overlap {
    pub fn build(id: Id) -> OverlapBuilder {
        OverlapBuilder {
            id,
            socket: SocketName::default(),
            element: Overlap,
        }
    }
}

impl Element for Overlap {
    fn run<'ctx, 'frm>(self, ctx: Context<'ctx, 'frm>, id: Id) -> LayoutNode<'frm> {
        archetype::panel(self, id, ctx)
    }
}

impl archetype::Panel for Overlap {
    fn open(&self, child_max_area: Area) -> Area {
        child_max_area
    }

    fn close<'ctx, 'frm>(self, _id: Id, ctx: Context<'ctx, 'frm>, children: Queue<'frm, LayoutNode<'frm>>) -> LayoutNode<'frm> {
        // Get the max size required among all children
        let max_area = (&children).into_iter().fold(Area::zero(), |max, child| max.stretch(child.min_area));

        // Use that as the min required space for this element
        ctx.new_layout(
            max_area,
            move |region: Region, cmds: &mut CommandList| {
                // Render every child in the same region
                for child in children {
                    child.render(region, cmds);
                }
            },
        )
    }
}

pub struct OverlapBuilder {
    id: Id,
    socket: SocketName,
    element: Overlap,
}

impl OverlapBuilder {
    pub fn socket(mut self, socket: SocketName) -> Self {
        self.socket = socket;
        self
    }
}

impl Builder for OverlapBuilder {
    type Element = Overlap;

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
