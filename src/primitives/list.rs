use std::f32;

use crate::prelude::*;
use crate::render::CommandList;

use super::archetype;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum ListDir {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct List {
    pub dir: ListDir,
}

impl List {
    pub fn new(dir: ListDir) -> Self {
        List { dir }
    }

    pub fn left_to_right() -> Self {
        List::new(ListDir::LeftToRight)
    }

    pub fn right_to_left() -> Self {
        List::new(ListDir::RightToLeft)
    }

    pub fn top_to_bottom() -> Self {
        List::new(ListDir::TopToBottom)
    }

    pub fn bottom_to_top() -> Self {
        List::new(ListDir::BottomToTop)
    }
}

impl Element for List {
    fn run<'ctx, 'win>(&self, ctx: Context<'ctx, 'win>, id: Id) -> LayoutNode<'win> {
        archetype::panel(ctx, id, self)
    }
}

impl archetype::Panel for List {
    fn open(&self, mut max_area: Area) -> Area {
        match self.dir {
            ListDir::LeftToRight | ListDir::RightToLeft => max_area.width = f32::INFINITY,
            ListDir::TopToBottom | ListDir::BottomToTop => max_area.height = f32::INFINITY,
        };

        max_area
    }

    fn close<'ctx, 'win>(&self, ctx: Context<'ctx, 'win>, _id: Id, children: Vec<LayoutNode<'win>>) -> LayoutNode<'win> {
        let mut min_area = Area::zero();

        // Figure out height and width for the stack
        match self.dir {
            ListDir::LeftToRight | ListDir::RightToLeft => {
                for child in &children {
                    min_area.width += child.min_area.width;
                    min_area.height = min_area.height.max(child.min_area.height);
                }
            }
            ListDir::TopToBottom | ListDir::BottomToTop => {
                for child in &children {
                    min_area.height += child.min_area.height;
                    min_area.width = min_area.width.max(child.min_area.width);
                }
            }
        }

        match self.dir {
            ListDir::LeftToRight => ctx.new_layout(min_area, move |region: Region, cmds: &mut CommandList| {
                render_left_to_right(children.as_slice(), region, cmds);
            }),
            ListDir::RightToLeft => ctx.new_layout(min_area, move |region: Region, cmds: &mut CommandList| {
                render_right_to_left(children.as_slice(), region, cmds);
            }),
            ListDir::TopToBottom => ctx.new_layout(min_area, move |region: Region, cmds: &mut CommandList| {
                render_top_to_bottom(children.as_slice(), region, cmds);
            }),
            ListDir::BottomToTop => ctx.new_layout(min_area, move |region: Region, cmds: &mut CommandList| {
                render_bottom_to_top(children.as_slice(), region, cmds);
            }),
        }
    }
}

fn render_left_to_right(children: &[LayoutNode], mut region: Region, cmds: &mut CommandList) {
    for child in children {
        let mut child_region = region;
        child_region.area.width = child.min_area.width;

        child.layout.render(child_region, cmds);

        region.pos.x += child.min_area.width;
        region.area.width -= child.min_area.width;
    }
}

fn render_right_to_left(children: &[LayoutNode], mut region: Region, out: &mut CommandList) {
    for child in children {
        let mut child_region = region;
        child_region.pos.x = child_region.pos.x + child_region.area.width - child.min_area.width;
        child_region.area.width = child.min_area.width;

        child.layout.render(child_region, out);

        region.area.width -= child.min_area.width;
    }
}

fn render_top_to_bottom(children: &[LayoutNode], mut region: Region, cmds: &mut CommandList) {
    for child in children {
        let mut child_region = region;
        child_region.area.height = child.min_area.height;

        child.layout.render(child_region, cmds);

        region.pos.y += child.min_area.height;
        region.area.height -= child.min_area.height;
    }
}

fn render_bottom_to_top(children: &[LayoutNode], mut region: Region, out: &mut CommandList) {
    for child in children {
        let mut child_region = region;
        child_region.pos.y = region.pos.y + region.area.height - child.min_area.height;
        child_region.area.height = child.min_area.height;

        child.layout.render(child_region, out);

        region.area.height -= child.min_area.height;
    }
}
