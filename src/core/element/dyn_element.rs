use std::any::Any;

use crate::core::*;

pub trait DynElement: Any {
    fn box_clone(
        &self
    ) -> Box<dyn DynElement>;

    fn box_run(
        self: Box<Self>,
        ctx: &mut Context,
        socket: &mut dyn socket::Socket,
    ) -> Option<Box<dyn DynElement>>;

    fn into_any_mut(
        &mut self,
    ) -> &mut Any;
}

impl<T: Element> DynElement for T {
    fn box_clone(
        &self
    ) -> Box<dyn DynElement> {
        Box::new(self.clone())
    }

    fn box_run(
        self: Box<Self>,
        ctx: &mut Context,
        socket: &mut dyn socket::Socket,
    ) -> Option<Box<dyn DynElement>> {
        let next = self.run(ctx, socket);
        next.map(|x| Box::new(x) as Box<dyn DynElement>)
    }

    fn into_any_mut(
        &mut self,
    ) -> &mut Any {
        self
    }
}

impl Clone for Box<dyn DynElement> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

impl Element for Box<dyn DynElement> {
    type Next = Self;

    fn run(
        self,
        ctx: &mut Context,
        socket: &mut dyn Socket,
    ) -> Option<Self> {
        self.box_run(ctx, socket)
    }

    fn upcast(
        self,
    ) -> Box<dyn DynElement> {
        self
    }

    fn downcast<D: Element>(
        self,
    ) -> Result<D, Self> {
        let raw = Box::into_raw(self);

        unsafe {
            match (*raw).into_any_mut().downcast_mut::<D>() {
                Some(v) => Ok(*Box::from_raw(v)),
                None => Err(Box::from_raw(raw))
            }
        }
    }
}
