use context::Context;
use layout::{Area, Bounds};
use tree::{Socket, Element};
use command_list::CommandList;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum HAlign {
    Left,
    Right,
    Center,
    Stretch,
}

fn align_horizontally(align: HAlign, bounds: Bounds, mut area: Area) -> Area {
    match align {
        HAlign::Left => {
            area.width = bounds.width;
        },
        HAlign::Right => {
        },
        _ => {
        },
    }

    return area;
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum VAlign {
    Top,
    Bottom,
    Center,
    Stretch,
}

fn align_vertically(align: VAlign, bounds: Bounds, mut area: Area) -> Area {
    return area;
}

pub struct Align {
    h_align: HAlign,
    v_align: VAlign,
}

impl Socket for HAlign {
    fn child(self: Box<Self>, ctx: &mut Context, child_min: Bounds, child_element: Box<Element>) {
        ctx.element(child_min, Box::new(move |mut area: Area, command_list: &mut CommandList| {
            area = align_horizontally(*self, child_min, area);
            child_element.render(area, command_list);
        }));
    }
}

impl Socket for VAlign {
    fn child(self: Box<Self>, ctx: &mut Context, child_min: Bounds, child_element: Box<Element>) {
        ctx.element(child_min, Box::new(move |mut area: Area, command_list: &mut CommandList| {
            area = align_vertically(*self, child_min, area);
            child_element.render(area, command_list);
        }));
    }
}

impl Socket for Align {
    fn child(self: Box<Self>, ctx: &mut Context, child_min: Bounds, child_element: Box<Element>) {
        ctx.element(child_min, Box::new(move |mut area: Area, command_list: &mut CommandList| {
            area = align_horizontally(self.h_align, child_min, area);
            area = align_vertically(self.v_align, child_min, area);
            child_element.render(area, command_list);
        }))
    }
}