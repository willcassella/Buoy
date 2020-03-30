use std::any::Any;

// TODO: Replace this with 'unsize' once that's stabilized
pub trait Upcast<T: ?Sized> {
    fn upcast(&self) -> &T;
    fn upcast_mut(&mut self) -> &mut T;
}

impl<T: ?Sized> Upcast<T> for T {
    fn upcast(&self) -> &T { self }
    fn upcast_mut(&mut self) -> &mut T { self }
}

#[macro_export]
macro_rules! impl_upcast {
    (dyn $t:ident) => {
        impl<'a, T: $t + 'a> $crate::util::upcast::Upcast<dyn $t + 'a> for T {
            fn upcast(&self) -> &(dyn $t + 'a) { self }
            fn upcast_mut(&mut self) -> &mut (dyn $t + 'a) { self }
        }
    };
}

impl_upcast!(dyn Any);
