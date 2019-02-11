use crate::Context;
use crate::layout::Area;
use crate::element::Filter;
use crate::render::UIRender;
use crate::util::fill::Fill;

pub struct UISocket {
    pub child_max_area: Area,
    pub imp: Box<dyn UISocketImpl>,
}

impl UISocket {
    pub fn new(child_max_area: Area, imp: Box<dyn UISocketImpl>) -> Self {
        UISocket {
            child_max_area,
            imp,
        }
    }
}

pub trait UISocketImpl {
    fn init(
        &mut self
    ) -> (Option<&dyn Filter>, &mut dyn Fill<UIRender>);

    fn close(
        self: Box<Self>,
        ctx: &mut Context,
    ) -> Option<UISocket>;
}
