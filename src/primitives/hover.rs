use crate::render::{CommandList, commands::{InputAction, Quad, HoverQuad}};
use crate::prelude::*;

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
    fn run<'window, 'ctx>(
        &self,
        ctx: Context<'window, 'ctx>,
    ) -> LayoutObj {
        archetype::wrap(self, ctx)
    }
}

impl archetype::Wrap for Hover {
    fn close_some<'window, 'ctx, L: Layout>(
        &self,
        _ctx: Context<'window, 'ctx>,
        child: LayoutObj<L>,
    ) -> LayoutObj {
        let this = self.clone();
        return LayoutObj::new(child.min_area, move |region: Region, cmds: &mut CommandList| {
            // Render the child
            child.imp.render(region, cmds);

            // Create the hover boundary
            let quad = HoverQuad {
                quad: Quad::from(region),
                active_state: this.hovered.clone(),
                action: this.action.clone(),
            };
            cmds.add_hover_quads(&[quad]);
        }).upcast();
    }

    fn close_none<'window, 'ctx>(
        &self,
        _ctx: Context<'window, 'ctx>,
    ) -> LayoutObj {
        LayoutObj::new(Area::zero(), ()).upcast()
    }
}
