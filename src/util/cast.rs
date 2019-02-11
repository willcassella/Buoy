use std::any::Any;
use std::result::Result;
use std::ops::Deref;
use std::rc::Rc;

pub trait IntoAny: Any {
    fn into_any(&self) -> &dyn Any;

    fn into_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any> IntoAny for T {
    fn into_any(&self) -> &dyn Any {
        self
    }

    fn into_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub trait Upcast<T: ?Sized> {
    type Out: Deref<Target = T>;

    fn upcast(self) -> Self::Out;
}

impl<T> Upcast<T> for Box<T> {
    type Out = Self;

    fn upcast(self) -> Self::Out {
        self
    }
}

impl<T> Upcast<T> for Rc<T> {
    type Out = Self;

    fn upcast(self) -> Self::Out {
        self
    }
}

pub trait Downcast<T: Any>: Sized {
    type Out: Any + Sized;

    fn downcast(self) -> Result<Self::Out, Self>;
}

impl<T: Any + Sized, F: IntoAny + ?Sized> Downcast<T> for Box<F> {
    type Out = Box<T>;

    fn downcast(self) -> Result<Box<T>, Self> {
        let raw = Box::into_raw(self);

        unsafe {
            match (*raw).into_any_mut().downcast_mut::<T>() {
                Some(t) => Ok(Box::from_raw(t)),
                None => Err(Box::from_raw(raw))
            }
        }
    }
}
