use crate::{Context, input::Input};
use crate::element::{UIWidgetImpl, UISocket, UIRender};
use crate::render::{CommandList, commands::{InputAction, Quad, HoverQuad}};
use crate::layout::Region;
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

impl UIWidgetImpl for Hover {
    type Next = ();

    fn run(
        mut self,
        ctx: &mut Context,
        socket: &mut dyn UISocket
    ) -> Option<Self::Next> {
        archetype::wrap(self, ctx, socket);
        None
    }
}

impl archetype::Wrap for Hover {
    fn close_some(
        self,
        ctx: &mut Context,
        socket: &mut dyn UISocket,
        child: UIRender,
    ) {
        ctx.render_new(socket, child.min_area, move |region: Region, cmds: &mut CommandList| {
            // Render the child
            child.imp.render(region, cmds);

            // Create the hover boundary
            let quad = HoverQuad {
                quad: Quad::from(region),
                active_state: self.hovered.clone(),
                action: self.action.clone(),
            };
            cmds.add_hover_quads(&[quad]);
        });
    }

    fn close_none(
        self,
        _ctx: &mut Context,
        _socket: &mut UISocket,
    ) {
        // Do nothing
    }
}
