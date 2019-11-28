use std::f32;

use crate::prelude::*;
use crate::render::CommandList;

use super::archetype;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Size {
    pub h_align: HAlign,
    pub v_align: VAlign,
    pub max: Area,
    pub min: Area,
}

impl Size {
    pub fn build(id: Id) -> SizeBuilder {
        SizeBuilder {
            id,
            socket: SocketName::default(),
            element: Size::default(),
        }
    }
}

impl Default for Size {
    fn default() -> Self {
        Size {
            h_align: HAlign::Center,
            v_align: VAlign::Center,
            min: Area::zero(),
            max: Area::infinite(),
        }
    }
}

impl Element for Size {
    fn run<'ctx, 'frm>(self, ctx: Context<'ctx, 'frm>, id: Id) -> LayoutNode<'frm> {
        archetype::wrap(self, id, ctx)
    }
}

impl archetype::Wrap for Size {
    fn open(&self, mut max_area: Area) -> Area {
        max_area.width = max_area.width.min(self.max.width).max(self.min.width);
        max_area.height = max_area.height.min(self.max.height).max(self.min.height);
        max_area
    }

    fn close_some<'ctx, 'frm>(self, _id: Id, ctx: Context<'ctx, 'frm>, child: LayoutNode<'frm>) -> LayoutNode<'frm> {
        ctx.new_layout(
            child.min_area.stretch(&self.min), // TODO: Handle if child is too big (will require clipping/scrolling)
            move |mut region: Region, cmds: &mut CommandList| {
                if self.max.width < region.area.width {
                    region = self.h_align.align(self.max, region);
                } else if self.min.width > region.area.width {
                    region.area.width = self.min.width;
                }
                if self.max.height < region.area.height {
                    region = self.v_align.align(self.max, region);
                } else if self.min.height > region.area.height {
                    region.area.height = self.min.height;
                }

                child.render(region, cmds);
            },
        )
    }

    fn close_none<'ctx, 'frm>(self, _id: Id, ctx: Context<'ctx, 'frm>) -> LayoutNode<'frm> {
        // Just take up space
        ctx.new_layout(self.min, ())
    }
}

pub struct SizeBuilder {
    id: Id,
    socket: SocketName,
    element: Size,
}

impl SizeBuilder {
    pub fn h_align(mut self, h_align: HAlign) -> Self {
        self.element.h_align = h_align;
        self
    }

    pub fn v_align(mut self, v_align: VAlign) -> Self {
        self.element.v_align = v_align;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.element.min.width = width;
        self.element.max.width = width;
        self
    }

    pub fn min_width(mut self, width: f32) -> Self {
        self.element.min.width = width;
        self
    }

    pub fn max_width(mut self, width: f32) -> Self {
        self.element.max.width = width;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.element.min.height = height;
        self.element.max.height = height;
        self
    }

    pub fn min_height(mut self, height: f32) -> Self {
        self.element.min.height = height;
        self
    }

    pub fn max_height(mut self, height: f32) -> Self {
        self.element.max.height = height;
        self
    }
}

impl Builder for SizeBuilder {
    type Element = Size;

    fn get_id(&self) -> Id {
        self.id
    }

    fn get_socket(&self) -> SocketName {
        self.socket
    }

    fn get_element(self) -> Self::Element {
        self.element
    }
}
