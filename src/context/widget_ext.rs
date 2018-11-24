use std::any::Any;
use std::ops::{Deref, DerefMut};
use std::convert::From;
use crate::util::fill::Fill;
use crate::{Context, Widget, ElementObj};
use crate::layout::Area;

pub trait Wrapper: Any {
    fn open(&self, self_max: Area) -> Area {
        self_max
    }

    fn close_some(self, ctx: &mut Context, child: ElementObj);

    fn close_none(self, ctx: &mut Context);
}

pub struct WrapperObj<T: Wrapper> {
    child: Option<ElementObj>,
    wrapper: T,
}

impl<T: Wrapper> From<T> for WrapperObj<T> {
    fn from(wrapper: T) -> Self {
        WrapperObj {
            child: None,
            wrapper,
        }
    }
}

impl<T: Wrapper> Widget for WrapperObj<T> {
    fn open<'a>(&'a mut self, self_bounds: Area) -> (&'a mut Fill<ElementObj>, Area) {
        (&mut self.child, self.wrapper.open(self_bounds))
    }

    fn close(self: Box<Self>, ctx: &mut Context) {
        // Moving out of box to work around issue with destructuring and boxes
        let this = *self;

        match this.child {
            Some(child) => this.wrapper.close_some(ctx, child),
            None => this.wrapper.close_none(ctx),
        }
    }
}

impl<T: Wrapper> Deref for WrapperObj<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.wrapper
    }
}

impl<T: Wrapper> DerefMut for WrapperObj<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.wrapper
    }
}

pub trait Generator: Any {
    fn generate(self, ctx: &mut Context);
}

pub struct GeneratorObj<T: Generator>(pub T, ());

impl<T: Generator> From<T> for GeneratorObj<T> {
    fn from(generator: T) -> Self {
        GeneratorObj(generator, ())
    }
}

impl<T: Generator> Widget for GeneratorObj<T> {
    fn open<'a>(&'a mut self, self_bounds: Area) -> (&'a mut Fill<ElementObj>, Area) {
        (&mut self.1, self_bounds)
    }

    fn close(self: Box<Self>, ctx: &mut Context) {
        self.0.generate(ctx);
    }
}

impl<T: Generator> Deref for GeneratorObj<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: Generator> DerefMut for GeneratorObj<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
