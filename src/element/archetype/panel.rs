use std::any::Any;
use std::ops::{Deref, DerefMut};
use crate::Context;
use crate::layout::Area;
use crate::element::{UIElementImpl, UISocket};
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
    fn run<'ui, 'ctx>(
        self: Box<Self>,
        ctx: &mut Context<'ui, 'ctx>,
    ) {
        let child_max_area = self.0.open(ctx.max_area());
        let mut children = Vec::new();

        let mut local = Context::local(ctx);
        local.socket_begin(UISocket::new(child_max_area, &mut children));
            local.children_all();
        local.end();

        // Wait for socket to fill up
        local.await_sockets();
        self.0.close(ctx, children);
    }
}
