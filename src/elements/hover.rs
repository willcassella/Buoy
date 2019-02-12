use crate::context::{State, Context};
use crate::element::{IntoUIElement, Widget, WidgetImpl};
use crate::render::{UIRender, CommandList, commands::{InputAction, Quad, HoverQuad}};
use crate::layout::Region;

#[derive(Clone)]
pub struct Hover {
    pub hovered: State<bool>,
    pub action: Option<InputAction>,
}

impl Hover {
    pub fn new(ctx: &mut Context, action: InputAction) -> Self {
        Hover {
            hovered: ctx.new_state_default(false),
            action: Some(action),
        }
    }

    pub fn new_no_action(ctx: &mut Context) -> Self {
        Hover {
            hovered: ctx.new_state_default(false),
            action: None,
        }
    }
}

impl IntoUIElement for Hover {
    type Target = Widget<Hover>;
}

impl WidgetImpl for Hover {
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
