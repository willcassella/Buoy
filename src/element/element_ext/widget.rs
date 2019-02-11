use std::any::Any;
use std::ops::{Deref, DerefMut};
use crate::Context;
use crate::layout::Area;
use crate::util::fill::Fill;
use crate::element::{UIElementImpl, UISocket, UISocketImpl, Filter};
use crate::render::UIRender;

pub trait WidgetImpl: Any + Clone {
    fn open(&self, max_area: Area) -> Area {
        max_area
    }

    fn close_some(
        self,
        ctx: &mut Context,
        child: UIRender,
    );

    fn close_none(
        self,
        ctx: &mut Context
    );
}

#[derive(Clone)]
pub struct Widget<T: WidgetImpl>(pub T);

impl<T: WidgetImpl> From<T> for Widget<T> {
    fn from(imp: T) -> Self {
        Widget(imp)
    }
}

impl<T: WidgetImpl> Deref for Widget<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: WidgetImpl> DerefMut for Widget<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: WidgetImpl> UIElementImpl for Widget<T> {
    fn open(
        self: Box<Self>,
        max_area: Area
    ) -> UISocket {
        let child_max_area = self.0.open(max_area);
        let socket = WidgetSocket(self.0, None);

        UISocket::new(child_max_area, Box::new(socket))
    }
}

struct WidgetSocket<T: WidgetImpl>(T, Option<UIRender>);

impl<T: WidgetImpl> UISocketImpl for WidgetSocket<T> {
    fn init(
        &mut self
    ) -> (Option<&dyn Filter>, &mut dyn Fill<UIRender>) {
        (None, &mut self.1)
    }

    fn close(
        self: Box<Self>,
        ctx: &mut Context,
    ) -> Option<UISocket> {
        match self.1 {
            Some(child) => self.0.close_some(ctx, child),
            None => self.0.close_none(ctx),
        };
        None
    }
}
