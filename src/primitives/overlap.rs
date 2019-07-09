use crate::prelude::*;
use crate::render::CommandList;
use super::archetype;

pub struct Overlap;

impl Element for Overlap {
    fn run(&self, ctx: Context, id: Id) -> LayoutObj {
        archetype::panel(ctx, id, self)
    }
}

impl archetype::Panel for Overlap {
    fn open(&self, child_max_area: Area) -> Area {
        child_max_area
    }

    fn close(&self, _ctx: Context, _id: Id, children: Vec<LayoutObj>) -> LayoutObj {
        // Get the max size required among all children
        let max_area = children.iter().fold(Area::zero(), |max, child| max.stretch(&child.min_area));

        // Use that as the min required space for this element
        LayoutObj::new(
            max_area,
            move |region: Region, cmds: &mut CommandList| {
                // Render every child in the same region
                for child in &children {
                    child.imp.render(region, cmds);
                }
            },
        ).upcast()
    }
}