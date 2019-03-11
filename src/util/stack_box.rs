use std::mem::ManuallyDrop;

pub struct StackBox<'a, T: ?Sized> {
    value: &'a mut T,
}

pub struct StackBoxStorage<T> {
    value: ManuallyDrop<T>,
}

impl<T> StackBoxStorage<T> {
    pub unsafe fn take<'a>(&'a mut self) -> StackBox<'a, T> {
        StackBox{ value: &mut self.value }
    }
}

impl<'a, T> StackBox<'a, T> {
    pub fn take(self) -> T {
        let value = unsafe{ std::ptr::read(self.value) };
        std::mem::forget(self);
        value
    }
}

impl<'a, T: ?Sized> Drop for StackBox<'a, T> {
    fn drop(&mut self) {
        unsafe { std::ptr::drop_in_place(self.value) };
    }
}
