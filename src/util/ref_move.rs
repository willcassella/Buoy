use crate::util::upcast::Upcast;
use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

pub trait Ext: Sized {
    fn anchor<A: ?Sized>(self) -> StackAnchor<Self, A>
    where
        Self: Upcast<A>;
}

impl<T> Ext for T {
    fn anchor<A: ?Sized>(self) -> StackAnchor<Self, A>
    where
        Self: Upcast<A>,
    {
        StackAnchor {
            value: self,
            _p: PhantomData,
        }
    }
}

pub trait Anchor<T: ?Sized>: DerefMut<Target = T> {
    unsafe fn dealloc(self);
}

pub struct StackAnchor<T, A: ?Sized> {
    value: T,
    _p: PhantomData<A>,
}

impl<A: ?Sized, T: Upcast<A>> Deref for StackAnchor<T, A> {
    type Target = A;
    fn deref(&self) -> &A {
        self.value.upcast()
    }
}

impl<A: ?Sized, T: Upcast<A>> DerefMut for StackAnchor<T, A> {
    fn deref_mut(&mut self) -> &mut A {
        self.value.upcast_mut()
    }
}

impl<A: ?Sized, T: Upcast<A>> Anchor<A> for StackAnchor<T, A> {
    #[inline(always)]
    unsafe fn dealloc(self) {
        std::mem::forget(self);
    }
}

struct BoxAnchor<T: ?Sized, A: ?Sized> {
    b: Box<T>,
    _p: PhantomData<A>,
}

impl<A: ?Sized, T: ?Sized + Upcast<A>> Deref for BoxAnchor<T, A> {
    type Target = A;

    #[inline(always)]
    fn deref(&self) -> &A {
        self.b.deref().upcast()
    }
}

impl<A: ?Sized, T: ?Sized + Upcast<A>> DerefMut for BoxAnchor<T, A> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut A {
        self.b.deref_mut().upcast_mut()
    }
}

impl<A: ?Sized, T: ?Sized + Upcast<A>> Anchor<A> for BoxAnchor<T, A> {
    #[inline(always)]
    unsafe fn dealloc(self) {
        // Create a Box to ManuallyDrop<T> and let it fall out of scope.
        // The memory behind the Box will be deallocated, but T's destructor will not be called.
        Box::from_raw(Box::into_raw(self.b) as *mut ManuallyDrop<T>);
    }
}

pub struct RefMove<'a, T: ?Sized> {
    ptr: NonNull<T>,
    _p: PhantomData<&'a ()>,
}

#[inline]
pub fn ref_move<P, F, A, R, T>(mut anchor: A, f: F) -> R
where
    for<'a> F: FnOnce(RefMove<'a, P>) -> R,
    P: ?Sized,
    T: ?Sized + Upcast<P>,
    A: Anchor<T>,
{
    unsafe {
        let rm = RefMove {
            ptr: NonNull::new_unchecked(anchor.deref_mut().upcast_mut()),
            _p: PhantomData,
        };
        let result = (f)(rm);
        anchor.dealloc();
        result
    }
}

impl<'a, T> RefMove<'a, T> {
    pub fn take(self) -> T {
        let result = unsafe { std::ptr::read(self.ptr.as_ptr()) };
        std::mem::forget(self);
        result
    }
}

impl<'a, T: ?Sized> RefMove<'a, T> {
    pub fn upcast<A>(mut x: Self) -> RefMove<'a, A>
    where
        T: Upcast<A>,
    {
        unsafe {
            RefMove {
                ptr: NonNull::new_unchecked(x.ptr.as_mut().upcast_mut()),
                _p: PhantomData,
            }
        }
    }

    // TODO: Checked variant of this
    pub unsafe fn downcast_unchecked<A>(x: Self) -> RefMove<'a, A> {
        let result = RefMove {
            ptr: NonNull::new_unchecked(x.ptr.as_ptr() as *mut A),
            _p: PhantomData,
        };
        std::mem::forget(x);
        result
    }
}

impl<'a, T: ?Sized> Drop for RefMove<'a, T> {
    fn drop(&mut self) {
        unsafe {
            std::ptr::drop_in_place(self.ptr.as_ptr());
        }
    }
}

impl<'a, T: ?Sized> Deref for RefMove<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { self.ptr.as_ref() }
    }
}

impl<'a, T: ?Sized> DerefMut for RefMove<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.ptr.as_mut() }
    }
}

struct RefMoveAnchor<'a, T: ?Sized> {
    ref_move: RefMove<'a, T>,
}

impl<'a, T: ?Sized> Deref for RefMoveAnchor<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.ref_move.deref()
    }
}

impl<'a, T: ?Sized> DerefMut for RefMoveAnchor<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.ref_move.deref_mut()
    }
}

impl<'a, T: ?Sized> Anchor<T> for RefMoveAnchor<'a, T> {
    #[inline(always)]
    unsafe fn dealloc(self) {
        // This anchor doesn't actually own the underlying allocation, so nothing
        // to deallocate.
        std::mem::forget(self);
    }
}
