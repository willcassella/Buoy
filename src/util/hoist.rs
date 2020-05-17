use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

pub trait Hoistable<'a>: Sized + 'a {
    type Hoisted: ApplyHoist<'a, Target = Self>;
}

pub trait ApplyHoist<'a>: Sized + 'static {
    type Target: Hoistable<'a, Hoisted = Self>;
}

pub struct Hoist<'a, T> {
    _p: PhantomData<&'a T>,
}

impl<'a, T: ApplyHoist<'a>> Deref for Hoist<'a, T> {
    type Target = T::Target;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Self as *const Self::Target) }
    }
}

impl<'a, T: ApplyHoist<'a>> DerefMut for Hoist<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self as *mut Self as *mut Self::Target) }
    }
}

pub trait Ext<'a>: Hoistable<'a> {
    fn hoist(&self) -> &Hoist<'a, Self::Hoisted> {
        unsafe { &*(self as *const Self as *const _) }
    }

    fn hoist_mut(&mut self) -> &mut Hoist<'a, Self::Hoisted> {
        unsafe { &mut *(self as *mut Self as *mut _) }
    }
}

impl<'a, T: Hoistable<'a>> Ext<'a> for T {}
