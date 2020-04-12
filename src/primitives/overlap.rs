use super::archetype;
use crate::basic_renderer::*;
use crate::prelude::*;
use crate::render::CommandList;
use crate::util::queue::Queue;
use std::rc::Rc;

pub struct Overlap;

impl Overlap {
    pub fn build(id: Id) -> OverlapBuilder {
        OverlapBuilder {
            id,
            socket: SocketName::default(),
            overlap: Overlap,
        }
    }
}

impl Component for Overlap {
    fn type_id() -> TypeId {
        TypeId::new("buoy", "overlap")
    }
}

impl Render for Overlap {
    fn render<'frm, 'thrd, 'ctx>(self, ctx: Context<'frm, 'thrd, 'ctx>) -> LayoutNode<'frm> {
        archetype::panel(self, ctx)
    }
}

impl archetype::Panel for Overlap {
    fn open(&self, child_max_area: Area) -> Area {
        child_max_area
    }

    fn close<'frm, 'thrd, 'ctx>(
        self,
        ctx: Context<'frm, 'thrd, 'ctx>,
        children: Queue<'frm, LayoutNode<'frm>>,
    ) -> LayoutNode<'frm> {
        // Get the max size required among all children
        let max_area = (&children)
            .into_iter()
            .fold(Area::zero(), |max, child| max.stretch(child.min_area));

        // Use that as the min required space for this element
        ctx.new_layout(max_area, move |region: Region, cmds: &mut CommandList| {
            // Render every child in the same region
            for child in children {
                child.render(region, cmds);
            }
        })
    }
}

pub struct OverlapBuilder {
    id: Id,
    socket: SocketName,
    overlap: Overlap,
}

impl OverlapBuilder {
    pub fn socket(mut self, socket: SocketName) -> Self {
        self.socket = socket;
        self
    }
}

impl Builder<'_> for OverlapBuilder {
    type Component = Overlap;

    fn get_id(&self) -> Id {
        self.id
    }

    fn get_socket(&self) -> SocketName {
        self.socket
    }

    fn get_component(self) -> Self::Component {
        self.overlap
    }
}

struct RendererFactory;
impl_basic_renderer_factory!(RendererFactory, Overlap);

pub fn register(window: &mut Window) {
    window.register_component(Overlap::type_id(), Rc::new(RendererFactory));
}
