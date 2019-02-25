use crate::context::{State, Context};
use crate::element::{IntoUIElement, Wrap, WrapImpl};
use crate::render::{UIRender, CommandList, commands::{InputAction, Quad, HoverQuad}};
use crate::layout::Region;

pub type HoverState = State<bool>;

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

impl IntoUIElement for Hover {
    type Target = Wrap<Hover>;
}

impl WrapImpl for Hover {
    fn close_some(
        self,
        ctx: &mut Context,
        child: UIRender,
    ) {
        ctx.render_new(child.min_area, Box::new(move |region: Region, cmds: &mut CommandList| {
            // Render the child
            child.imp.render(region, cmds);

            // Create the hover boundary
            let quad = HoverQuad {
                quad: Quad::from(region),
                active_state: self.hovered.clone(),
                action: self.action.clone(),
            };
            cmds.add_hover_quads(&[quad]);
        }));
    }

    fn close_none(
        self,
        _ctx: &mut Context,
    ) {
        // Do nothing
    }
}
