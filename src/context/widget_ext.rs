use crate::{Context, Widget, WidgetObj, ElementObj};
use crate::layout::Area;

pub trait Wrapper {
    fn child_layout(&self, self_max: Area) -> Area {
        self_max
    }

    fn child_element(self: Box<Self>, ctx: &mut Context, child: ElementObj);

    fn close(self: Box<Self>, ctx: &mut Context);
}

pub struct WrapperObj<T: Wrapper>(T);

impl<T: Wrapper> Widget for WrapperObj<T> {
    fn child_layout(&self, self_max: Area) -> (usize, Area) {
        (1, Wrapper::child_layout(self, self_max))
    }

    fn child_elements(self: Box<Self>, ctx: &mut Context, children: Vec<ElementObj>) {
        match children.into_iter().next() {
            Some(child) => Wrapper::child_element(self, ctx, child),
            None => Wrapper::close(self, ctx),
        }
    }
}