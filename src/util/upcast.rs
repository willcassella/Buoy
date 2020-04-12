use std::any::Any;

// TODO: Replace this with 'unsize' once that's stabilized
pub trait Upcast<T: ?Sized> {
    fn upcast(&self) -> &T;
    fn upcast_mut(&mut self) -> &mut T;
}

impl<T: ?Sized> Upcast<T> for T {
    #[inline(always)]
    fn upcast(&self) -> &T {
        self
    }

    #[inline(always)]
    fn upcast_mut(&mut self) -> &mut T {
        self
    }
}

// Macro to automatically implement Upcast<X> for any type that implements X
#[macro_export]
macro_rules! auto_impl_upcast {
    (dyn $t:ident) => {
        impl<'a, T: $t + 'a> $crate::util::upcast::Upcast<dyn $t + 'a> for T {
            #[inline(always)]
            fn upcast(&self) -> &(dyn $t + 'a) {
                self
            }

            #[inline(always)]
            fn upcast_mut(&mut self) -> &mut (dyn $t + 'a) {
                self
            }
        }
    };
}

auto_impl_upcast!(dyn Any);
