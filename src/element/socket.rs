use crate::util::fill::Fill;
use crate::element::UIRender;
use crate::layout::Area;

pub struct UISocket<'ctx> {
    pub(crate) imp: &'ctx mut UISocketImpl,
    pub(crate) max_area: Area,
}

impl<'ctx> UISocket<'ctx> {
    pub fn new(
        max_area: Area,
        imp: &'ctx mut UISocketImpl
    ) -> Self {
        UISocket {
            imp,
            max_area,
        }
    }

    pub fn max_area(&self) -> Area {
        self.max_area
    }
}

pub trait UISocketImpl: Fill<UIRender> {
}

impl<T: Fill<UIRender>> UISocketImpl for T {
}
