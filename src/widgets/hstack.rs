use std::f32;
use context::Context;
use layout::{Area, Region, Point};
use tree::{Socket, Element};
use commands::CommandList;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum HDir {
    LeftToRight,
    RightToLeft,
}

pub struct HStack {
    pub dir: HDir,
    width: f32,
    height: f32,
    children: Vec<(Box<Element>, f32)>,
}

impl HStack {
    pub fn push(self, ctx: &mut Context) {
        ctx.push_socket(Box::new(self));
    }
}

impl Default for HStack {
    fn default() -> Self {
        HStack {
            dir: HDir::LeftToRight,
            width: 0_f32,
            height: 0_f32,
            children: Vec::new(),
        }
    }
}

impl Socket for HStack {
    fn get_child_max(&self, mut self_max: Area) -> Area {
        self_max.width = f32::INFINITY;
        return self_max;
    }

    fn child(
        mut self: Box<Self>,
        ctx: &mut Context,
        child_min: Area,
        child_element: Box<Element>
    ) {
        self.width += child_min.width;
        self.height = self.height.max(child_min.height);
        self.children.push((child_element, child_min.width));

        ctx.push_socket(self);
        ctx.children();
        ctx.pop(); // socket
    }

    fn close(self: Box<Self>, ctx: &mut Context) {
        // Optimization: If there are no children, nothing to render
        if self.children.is_empty() {
            return;
        }

        let bounds = Area {
            width: self.width,
            height: self.height,
        };
        ctx.element(bounds, self);
    }
}

fn render_left_to_right(children: &Vec<(Box<Element>, f32)>, mut region: Region, cmds: &mut CommandList) {
    for (child, width) in children {
        let mut child_region = region;
        child_region.area.width = *width;

        child.render(child_region, cmds);

        region.pos.x += width;
    }
}

fn render_right_to_left(children: &Vec<(Box<Element>, f32)>, region: Region, out: &mut CommandList) {
    let mut x = region.pos.x + region.area.width;

    for (child, width) in children {
        let child_region = Region {
            pos: Point {
                x: x - width,
                y: region.pos.y,
            },
            area: Area {
                width: *width,
                height: region.area.height,
            },
        };

        child.render(child_region, out);

        x -= width;
    }
}

impl Element for HStack {
    fn render(&self, region: Region, cmds: &mut CommandList) {
        match self.dir {
            HDir::LeftToRight => {
                render_left_to_right(&self.children, region, cmds);
            },
            HDir::RightToLeft => {
                render_right_to_left(&self.children, region, cmds);
            }
        }
    }
}