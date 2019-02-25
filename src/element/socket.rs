use crate::util::fill::Fill;
use crate::render::UIRender;
use crate::layout::Area;

pub struct UISocket<'ctx> {
    imp: &'ctx mut UISocketImpl,
    max_area: Area,
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
}

pub trait UISocketImpl {
    fn render(
        &mut self,
        render: UIRender,
    );
}

impl<T: Fill<UIRender>> UISocketImpl for T {
    fn render(
        &mut self,
        render: UIRender,
    ) {
        if self.remaining_capacity() != 0 {
            self.push(render);
        }
    }
}
