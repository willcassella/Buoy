use crate::prelude::*;
use crate::render::CommandList;
use super::archetype;

pub struct Overlap;

impl Element for Overlap {
    fn run<'ctx, 'win>(&self, ctx: Context<'ctx, 'win>, id: Id) -> LayoutNode<'win> {
        archetype::panel(ctx, id, self)
    }
}

impl archetype::Panel for Overlap {
    fn open(&self, child_max_area: Area) -> Area {
        child_max_area
    }

    fn close<'ctx, 'win>(&self, ctx: Context<'ctx, 'win>, _id: Id, children: Vec<LayoutNode<'win>>) -> LayoutNode<'win> {
        // Get the max size required among all children
        let max_area = children.iter().fold(Area::zero(), |max, child| max.stretch(&child.min_area));

        // Use that as the min required space for this element
        ctx.new_layout(
            max_area,
            move |region: Region, cmds: &mut CommandList| {
                // Render every child in the same region
                for child in &children {
                    child.layout.render(region, cmds);
                }
            },
        )
    }
}