use std::any::Any;
use std::ops::{Deref, DerefMut};
use crate::Context;
use crate::layout::Area;
use crate::util::fill::Fill;
use crate::element::{UIElementImpl, UISocket, UISocketImpl, Filter};
use crate::render::UIRender;

pub trait PanelImpl: Any + Clone {
    fn open(
        &self,
        max_area: Area
    ) -> Area;

    fn close(
        self,
        ctx: &mut Context,
        children: Vec<UIRender>
    );
}

#[derive(Clone)]
pub struct Panel<T: PanelImpl>(pub T);

impl<T: PanelImpl> From<T> for Panel<T> {
    fn from(panel: T) -> Self {
        Panel(panel)
    }
}

impl<T: PanelImpl> Deref for Panel<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: PanelImpl> DerefMut for Panel<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: PanelImpl> UIElementImpl for Panel<T> {
    fn open(
        self: Box<Self>,
        max_area: Area
    ) -> UISocket {
        let child_max_area = self.0.open(max_area);
        let socket = PanelSocket(self.0, Vec::new());

        UISocket::new(child_max_area, Box::new(socket))
    }
}

struct PanelSocket<T: PanelImpl>(T, Vec<UIRender>);

impl<T: PanelImpl> UISocketImpl for PanelSocket<T> {
    fn init(
        &mut self,
    ) -> (Option<&dyn Filter>, &mut dyn Fill<UIRender>) {
        (None, &mut self.1)
    }

    fn close(
        self: Box<Self>,
        ctx: &mut Context,
    ) -> Option<UISocket> {
        self.0.close(ctx, self.1);
        None
    }
}
