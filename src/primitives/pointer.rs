use std::rc::Rc;
use crate::State;
use crate::layout::Region;
use crate::element::{IntoUIElement, Input, InputObj};
use crate::render::CommandList;
use crate::render::commands::{HoverQuad, Quad};

pub struct PointerHover {
    pub action: Rc<Fn()>,
    pub active: State<bool>,
}

impl PointerHover {
    pub fn new(action: Rc<Fn()>, active: State<bool>) -> Self {
        PointerHover {
            action,
            active,
        }
    }
}

impl Input for PointerHover {
    fn render(&self, region: Region, cmds: &mut CommandList) {
        let hover_quad = HoverQuad {
            quad: Quad::from(region),
            action: self.action.clone(),
            active_state: self.active,
        };
        cmds.hover_quads.push(hover_quad);
    }
}

impl IntoUIElement for PointerHover {
    type Target = InputObj<Self>;
}
