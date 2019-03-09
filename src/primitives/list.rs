use std::f32;
use crate::layout::{Area, Region};
use crate::render::CommandList;
use crate::core::*;

use super::archetype;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum Dir {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct List {
    pub dir: Dir,
}

impl List {
    pub fn new(dir: Dir) -> Self {
        List{
            dir,
        }
    }

    pub fn left_to_right() -> Self {
        List::new(Dir::LeftToRight)
    }

    pub fn right_to_left() -> Self {
        List::new(Dir::RightToLeft)
    }

    pub fn top_to_bottom() -> Self {
        List::new(Dir::TopToBottom)
    }

    pub fn bottom_to_top() -> Self {
        List::new(Dir::BottomToTop)
    }
}

impl Element for List {
    type Resume = ();

    fn run(
        self,
        ctx: &mut Context,
        socket: &mut dyn Socket,
    ) -> Option<Self::Resume> {
        archetype::panel(self, ctx, socket);
        None
    }
}

impl archetype::Panel for List {
    fn open(
        &self,
        mut max_area: Area
    ) -> Area {
        match self.dir {
            Dir::LeftToRight |
            Dir::RightToLeft => max_area.width = f32::INFINITY,
            Dir::TopToBottom |
            Dir::BottomToTop => max_area.height = f32::INFINITY,
        };

        max_area
    }

    fn close(
        self,
        ctx: &mut Context,
        socket: &mut dyn Socket,
        children: Vec<Render>
    ) {
        let mut min_area = Area::zero();

        // Figure out height and width for the stack
        match self.dir {
            Dir::LeftToRight |
            Dir::RightToLeft => {
                for child in &children {
                    min_area.width += child.min_area.width;
                    min_area.height = min_area.height.max(child.min_area.height);
                }
            },
            Dir::TopToBottom |
            Dir::BottomToTop => {
                for child in &children {
                    min_area.height += child.min_area.height;
                    min_area.width = min_area.width.max(child.min_area.width);
                }
            }
        }

        let render_func: Box<dyn render::RenderImpl> = match self.dir {
            Dir::LeftToRight => Box::new(move |region: Region, cmds: &mut CommandList| {
                render_left_to_right(children.as_slice(), region, cmds);
            }),
            Dir::RightToLeft => Box::new(move |region: Region, cmds: &mut CommandList| {
                render_right_to_left(children.as_slice(), region, cmds);
            }),
            Dir::TopToBottom => Box::new(move |region: Region, cmds: &mut CommandList| {
                render_top_to_bottom(children.as_slice(), region, cmds);
            }),
            Dir::BottomToTop => Box::new(move |region: Region, cmds: &mut CommandList| {
                render_bottom_to_top(children.as_slice(), region, cmds);
            }),
        };

        ctx.render(socket, Render{ min_area, imp: render_func });
    }
}

fn render_left_to_right(children: &[Render], mut region: Region, cmds: &mut CommandList) {
    for child in children {
        let mut child_region = region;
        child_region.area.width = child.min_area.width;

        child.imp.render(child_region, cmds);

        region.pos.x += child.min_area.width;
        region.area.width -= child.min_area.width;
    }
}

fn render_right_to_left(children: &[Render], mut region: Region, out: &mut CommandList) {
    for child in children {
        let mut child_region = region;
        child_region.pos.x = child_region.pos.x + child_region.area.width - child.min_area.width;
        child_region.area.width = child.min_area.width;

        child.imp.render(child_region, out);

        region.area.width -= child.min_area.width;
    }
}

fn render_top_to_bottom(children: &[Render], mut region: Region, cmds: &mut CommandList) {
    for child in children {
        let mut child_region = region;
        child_region.area.height = child.min_area.height;

        child.imp.render(child_region, cmds);

        region.pos.y += child.min_area.height;
        region.area.height -= child.min_area.height;
    }
}

fn render_bottom_to_top(children: &[Render], mut region: Region, out: &mut CommandList) {
    for child in children {
        let mut child_region = region;
        child_region.pos.y = region.pos.y + region.area.height - child.min_area.height;
        child_region.area.height = child.min_area.height;

        child.imp.render(child_region, out);

        region.area.height -= child.min_area.height;
    }
}
