use crate::prelude::*;
use crate::render::{
    commands::{HoverQuad, HoverAction, Quad},
    CommandList,
};

#[derive(Clone)]
pub struct Hover {
    pub action: HoverAction,
}

impl Hover {
    pub fn new(action: HoverAction) -> Self {
        Hover {
            action,
        }
    }
}

impl Element for Hover {
    fn run(&self, ctx: Context, id: Id) -> LayoutObj {
        let action = self.action.clone();

        LayoutObj::new(
            Area::zero(),
            move |region: Region, cmds: &mut CommandList| {
                // Create the hover boundary
                let quad = HoverQuad {
                    quad: Quad::from(region),
                    action: action.clone(),
                };
                cmds.add_hover_quads(&[quad]);
            },
        ).upcast()
    }
}