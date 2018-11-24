use std::f32;
use context::{Context, WidgetInfo, WidgetId};
use layout::{Area, Region};
use tree::{Socket, Element};
use commands::CommandList;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum VDir {
    TopToBottom,
    BottomToTop,
}

pub struct VStack {
    pub dir: VDir,
    width: f32,
    height: f32,
    children: Vec<(Box<Element>, f32)>,
}

impl VStack {
    pub fn new() -> Box<Self> {
        Box::new(Self::default())
    }

    pub fn push(self: Box<Self>, ctx: &mut Context, info: WidgetInfo) {
        ctx.push_socket(info, self);
    }

    pub fn dir(mut self: Box<Self>, dir: VDir) -> Box<Self> {
        self.dir = dir;
        self
    }
}

impl Default for VStack {
    fn default() -> Self {
        VStack {
            dir: VDir::TopToBottom,
            width: 0_f32,
            height: 0_f32,
            children: Vec::new(),
        }
    }
}

impl Socket for VStack {
    fn get_child_max(&self, mut self_max: Area) -> Area {
        self_max.height = f32::INFINITY;
        return self_max;
    }

    fn child(
        mut self: Box<Self>,
        ctx: &mut Context,
        child_min: Area,
        child_element: Box<Element>
    ) {
        self.height += child_min.height;
        self.width = self.width.max(child_min.width);
        self.children.push((child_element, child_min.height));

        let id = WidgetId::prefix(ctx.self_id(), WidgetId::num(self.children.len() as u64));
        ctx.push_socket(WidgetInfo::new(id), self);
            ctx.children();
        ctx.pop(); // self
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

fn render_top_to_bottom(children: &Vec<(Box<Element>, f32)>, mut region: Region, cmds: &mut CommandList) {
    region.pos.y += region.area.height;

    for (child, height) in children {
        region.pos.y -= *height;

        let mut child_region = region;
        child_region.area.height = *height;

        child.render(child_region, cmds);
    }
}

fn render_bottom_top_top(children: &Vec<(Box<Element>, f32)>, mut region: Region, out: &mut CommandList) {
    for (child, height) in children {
        let mut child_region = region;
        child_region.area.height = *height;

        child.render(child_region, out);

        region.pos.y += *height;
    }
}

impl Element for VStack {
    fn render(&self, region: Region, cmds: &mut CommandList) {
        match self.dir {
            VDir::TopToBottom => {
                render_top_to_bottom(&self.children, region, cmds);
            },
            VDir::BottomToTop => {
                render_bottom_top_top(&self.children, region, cmds);
            }
        }
    }
}