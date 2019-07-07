use crate::prelude::*;
use crate::render::{
    commands::{HoverQuad, InputAction, Quad},
    CommandList,
};

use super::archetype;

pub type HoverState = Input<bool>;

#[derive(Clone)]
pub struct Hover {
    pub hovered: HoverState,
    pub action: Option<InputAction>,
}

impl Hover {
    pub fn new(state: HoverState, action: InputAction) -> Self {
        Hover {
            hovered: state,
            action: Some(action),
        }
    }

    pub fn new_no_action(state: HoverState) -> Self {
        Hover {
            hovered: state,
            action: None,
        }
    }
}

impl Element for Hover {
    fn run(&self, ctx: Context, id: Id) -> LayoutObj {
        archetype::wrap(ctx, id, self)
    }
}

impl archetype::Wrap for Hover {
    fn close_some<L: Layout>(&self, _ctx: Context, _id: Id, child: LayoutObj<L>) -> LayoutObj {
        let this = self.clone();
        LayoutObj::new(
            child.min_area,
            move |region: Region, cmds: &mut CommandList| {
                // Render the child
                child.imp.render(region, cmds);

                // Create the hover boundary
                let quad = HoverQuad {
                    quad: Quad::from(region),
                    active_state: this.hovered,
                    action: this.action.clone(),
                };
                cmds.add_hover_quads(&[quad]);
            },
        )
        .upcast()
    }

    fn close_none(&self, _ctx: Context, _id: Id) -> LayoutObj {
        LayoutObj::new(Area::zero(), ()).upcast()
    }
}
