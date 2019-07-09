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
    fn run(&self, ctx: Context, id: Id) -> LayoutObj {
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

    fn close(&self, _ctx: Context, _id: Id, children: Vec<LayoutObj>) -> LayoutObj {
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

        let layout_func: Box<dyn Layout> = match self.dir {
            ListDir::LeftToRight => Box::new(move |region: Region, cmds: &mut CommandList| {
                render_left_to_right(children.as_slice(), region, cmds);
            }),
            ListDir::RightToLeft => Box::new(move |region: Region, cmds: &mut CommandList| {
                render_right_to_left(children.as_slice(), region, cmds);
            }),
            ListDir::TopToBottom => Box::new(move |region: Region, cmds: &mut CommandList| {
                render_top_to_bottom(children.as_slice(), region, cmds);
            }),
            ListDir::BottomToTop => Box::new(move |region: Region, cmds: &mut CommandList| {
                render_bottom_to_top(children.as_slice(), region, cmds);
            }),
        };

        LayoutObj::new(min_area, layout_func)
    }
}

fn render_left_to_right(children: &[LayoutObj], mut region: Region, cmds: &mut CommandList) {
    for child in children {
        let mut child_region = region;
        child_region.area.width = child.min_area.width;

        child.imp.render(child_region, cmds);

        region.pos.x += child.min_area.width;
        region.area.width -= child.min_area.width;
    }
}

fn render_right_to_left(children: &[LayoutObj], mut region: Region, out: &mut CommandList) {
    for child in children {
        let mut child_region = region;
        child_region.pos.x = child_region.pos.x + child_region.area.width - child.min_area.width;
        child_region.area.width = child.min_area.width;

        child.imp.render(child_region, out);

        region.area.width -= child.min_area.width;
    }
}

fn render_top_to_bottom(children: &[LayoutObj], mut region: Region, cmds: &mut CommandList) {
    for child in children {
        let mut child_region = region;
        child_region.area.height = child.min_area.height;

        child.imp.render(child_region, cmds);

        region.pos.y += child.min_area.height;
        region.area.height -= child.min_area.height;
    }
}

fn render_bottom_to_top(children: &[LayoutObj], mut region: Region, out: &mut CommandList) {
    for child in children {
        let mut child_region = region;
        child_region.pos.y = region.pos.y + region.area.height - child.min_area.height;
        child_region.area.height = child.min_area.height;

        child.imp.render(child_region, out);

        region.area.height -= child.min_area.height;
    }
}
