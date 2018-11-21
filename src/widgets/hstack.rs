use std::f32;
use std::usize;
use crate::{Context, Widget, WidgetObj, Element, ElementObj};
use crate::layout::{Area, Region};
use crate::commands::CommandList;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum HDir {
    LeftToRight,
    RightToLeft,
}

impl Default for HDir {
    fn default() -> Self {
        HDir::LeftToRight
    }
}

#[derive(Default)]
pub struct HStack {
    pub dir: HDir,
}

impl WidgetObj<HStack> {
    pub fn dir(mut self, dir: HDir) -> Self {
        self.widget.dir = dir;
        self
    }
}

impl Widget for HStack {
    fn child_layout(&self, mut self_bounds: Area) -> (usize, Area) {
        self_bounds.width = f32::INFINITY;
        (usize::MAX, self_bounds)
    }

    fn child_elements(self: Box<Self>, ctx: &mut Context, children: Vec<ElementObj>) {
        let mut min_area = Area::zero();

        for child in &children {
            min_area.width += child.min_area.width;
            min_area.height = min_area.height.max(child.min_area.height);
        }

        let render_func: Box<Element> = match self.dir {
            HDir::LeftToRight => Box::new(move |region: Region, cmds: &mut CommandList| {
                render_left_to_right(children.as_slice(), region, cmds);
            }),
            HDir::RightToLeft => Box::new(move |region: Region, cmds: &mut CommandList| {
                render_right_to_left(children.as_slice(), region, cmds);
            }),
        };

        ctx.new_element(min_area, render_func);
    }
}

fn render_left_to_right(children: &[ElementObj], mut region: Region, cmds: &mut CommandList) {
    for child in children {
        let mut child_region = region;
        child_region.area.width = child.min_area.width;

        child.element.render(child_region, cmds);

        region.pos.x += child.min_area.width;
        region.area.width -= child.min_area.width;
    }
}

fn render_right_to_left(children: &[ElementObj], mut region: Region, out: &mut CommandList) {
    for child in children {
        let mut child_region = region;
        child_region.pos.x = child_region.pos.x + child_region.area.width - child.min_area.width;
        child_region.area.width = child.min_area.width;

        child.element.render(child_region, out);

        region.area.width -= child.min_area.width;
    }
}