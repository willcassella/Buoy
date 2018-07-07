use std::f32;
use context::Context;
use command_list::CommandList;
use tree::{Socket, Element};
use layout::{Area, Bounds};

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
    fn get_child_max(&self, mut self_max: Bounds) -> Bounds {
        self_max.width = f32::INFINITY;
        return self_max;
    }

    fn child(
        mut self: Box<Self>,
        ctx: &mut Context,
        child_min: Bounds,
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

        let bounds = Bounds{
            width: self.width,
            height: self.height,
        };
        ctx.element(bounds, self);
    }
}

fn render_left_to_right(children: Vec<(Box<Element>, f32)>, mut area: Area, out: &mut CommandList) {
    for (child, width) in children {
        let child_area = Area {
            x: area.x,
            y: area.y,
            width,
            height: area.height,
        };

        child.render(child_area, out);

        area.x += width;
    }
}

fn render_right_to_left(children: Vec<(Box<Element>, f32)>, area: Area, out: &mut CommandList) {
    let mut x = area.x + area.width;

    for (child, width) in children {
        let child_area = Area {
            x: x - width,
            y: area.y,
            width,
            height: area.height,
        };

        child.render(child_area, out);

        x -= width;
    }
}

impl Element for HStack {
    fn render(&self, area: Area, out: &mut CommandList) {
        match self.dir {
            HDir::LeftToRight => {
                render_left_to_right(self.children, area, out);
            },
            HDir::RightToLeft => {
                render_right_to_left(self.children, area, out);
            }
        }
    }
}