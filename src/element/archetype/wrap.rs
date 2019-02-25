use std::any::Any;
use std::ops::{Deref, DerefMut};
use crate::Context;
use crate::layout::Area;
use crate::element::{UIElementImpl, UISocket};
use crate::render::UIRender;

pub trait WrapImpl: Any + Clone {
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
pub struct Wrap<T: WrapImpl>(pub T);

impl<T: WrapImpl> From<T> for Wrap<T> {
    fn from(imp: T) -> Self {
        Wrap(imp)
    }
}

impl<T: WrapImpl> Deref for Wrap<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: WrapImpl> DerefMut for Wrap<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: WrapImpl> UIElementImpl for Wrap<T> {
    fn run<'ui, 'ctx>(
        self: Box<Self>,
        ctx: &mut Context<'ui, 'ctx>,
    ) {
        let child_max_area = self.0.open(ctx.max_area());
        let mut socket = None;

        let mut local = Context::local(ctx);
        local.socket_begin(UISocket::new(child_max_area, &mut socket));
            local.children_all();
        local.end();

        // Wait for sockets to fill
        local.await_sockets();

        match socket {
            Some(child) => self.0.close_some(ctx, child),
            None => self.0.close_none(ctx),
        }
    }
}
