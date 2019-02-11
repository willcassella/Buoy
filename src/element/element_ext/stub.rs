use std::any::Any;
use std::ops::{Deref, DerefMut};
use crate::Context;
use crate::layout::Area;
use crate::util::fill::Fill;
use crate::element::{UIElementImpl, UISocket, UISocketImpl, Filter};
use crate::render::UIRender;

pub trait StubImpl: Any + Clone {
    fn generate(
        self,
        ctx: &mut Context
    );
}

#[derive(Clone)]
pub struct Stub<T: StubImpl>(pub T);

impl<T: StubImpl> From<T> for Stub<T> {
    fn from(imp: T) -> Self {
        Stub(imp)
    }
}

impl<T: StubImpl> Deref for Stub<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: StubImpl> DerefMut for Stub<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: StubImpl> UIElementImpl for Stub<T> {
    fn open(
        self: Box<Self>,
        _max_area: Area,
    ) -> UISocket {
        let socket = StubSocket(self.0, ());
        UISocket::new(Area::zero(), Box::new(socket))
    }
}

pub struct StubSocket<T: StubImpl>(T, ());

impl<T: StubImpl> UISocketImpl for StubSocket<T> {
    fn init(
        &mut self,
    ) -> (Option<&dyn Filter>, &mut Fill<UIRender>) {
        (None, &mut self.1)
    }

    fn close(
        self: Box<Self>,
        ctx: &mut Context,
    ) -> Option<UISocket> {
        self.0.generate(ctx);
        None
  }
}
