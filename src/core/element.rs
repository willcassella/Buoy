use std::any::Any;

use crate::core::tree::*;
use crate::core::common::*;

mod context;
pub use self::context::{Context, DynContext, ContextImpl};

mod id;
pub use self::id::Id;

mod socket;
pub use self::socket::{Socket, SocketName};

mod layout;
pub use self::layout::{Layout, LayoutObj};

pub trait Element: Sized + Clone + Any {
    fn run<'a, C: Context<'a>>(
        self,
        ctx: C,
    );

    fn upcast_box(
        self,
    ) -> Box<dyn DynElement> {
        Box::new(self)
    }

    fn downcast<D: Element>(
        self,
    ) -> Result<D, Self> {
        Err(self) // TODO: This should handle when Self == D
    }
}

impl Element for () {
    fn run<'a, C: Context<'a>>(
        self,
        _ctx: C,
    ) {
        // Do nothing
    }
}

pub trait DynElement: Any {
    fn box_clone(
        &self
    ) -> Box<dyn DynElement>;

    fn box_run<'a>(
        self: Box<Self>,
        ctx: DynContext<'a>,
    );

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

    fn box_run<'a>(
        self: Box<Self>,
        ctx: DynContext<'a>,
    ) {
        unimplemented!()
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
    fn run<'a, C: Context<'a>>(
        self,
        ctx: C,
    ) {
        self.box_run(ctx.upcast())
    }

    fn upcast_box(
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
