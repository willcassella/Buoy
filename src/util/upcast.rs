use std::ops::Deref;
use std::rc::Rc;

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
