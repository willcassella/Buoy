use std::any::Any;
use std::ops::{Deref, DerefMut};
use crate::Context;
use crate::layout::{Area, Region};
use crate::element::{UIElement, UIElementOpen};
use crate::render::{UIRenderObj, CommandList};

pub trait Input: Any {
    fn render(&self, region: Region, cmds: &mut CommandList);
}

pub struct InputObj<T: Input> {
    child: Option<UIRenderObj>,
    input: T,
}

impl<T: Input> From<T> for InputObj<T> {
    fn from(input: T) -> Self {
        InputObj {
            child: None,
            input,
        }
    }
}

impl<T: Input> UIElement for InputObj<T> {
    fn open<'a>(&'a mut self, max_area: Area) -> UIElementOpen<'a> {
        UIElementOpen {
            child_max_area: max_area,
            child_fill: &mut self.child,
        }
    }

    fn close(self: Box<Self>, ctx: &mut Context) {
        let this = *self;
        let InputObj{ child, input } = this;

        if let Some(child) = child {
            ctx.render_new(child.min_area, Box::new(move |region: Region, cmds: &mut CommandList| {
                input.render(region, cmds);
                child.render.render(region, cmds);
            }));
        }
    }
}

impl<T: Input> Deref for InputObj<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.input
    }
}

impl<T: Input> DerefMut for InputObj<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.input
    }
}
