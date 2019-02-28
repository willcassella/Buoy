use std::marker::PhantomData;
use crate::util::fill::Fill;
use crate::element::UIRender;
use crate::layout::Area;

pub struct UISocket {
    pub(crate) imp: Box<dyn UISocketImpl>,
    pub(crate) max_area: Area,
}

impl UISocket {
    pub fn new(
        max_area: Area,
        imp: Box<dyn UISocketImpl>,
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

impl<T: Fill<UIRender> + 'static> UISocketImpl for T {
}

pub struct SocketRef<'ctx, T> {
    _phantom: PhantomData<&'ctx mut T>,
}

impl<'ctx, T> SocketRef<'ctx, T> {
    pub(crate) fn new() -> Self {
        SocketRef {
            _phantom: PhantomData,
        }
    }
}
