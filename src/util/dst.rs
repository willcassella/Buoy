use std::marker::{PhantomPinned, Unsize};
use std::ops::{Deref, DerefMut};
use std::mem::{size_of, align_of};

#[repr(C)]
pub struct DstField<T: ?Sized> {
    value: *mut T,
    _pinned: PhantomPinned,
}

impl<T: ?Sized> DstField<T> {
    pub unsafe fn uninitialized() -> Self {
        DstField {
            value: std::mem::zeroed(),
            _pinned: PhantomPinned,
        }
    }

    pub unsafe fn init<U: Unsize<T>>(&mut self, value: U) {
        // TODO: Is there a better way to enforce this?
        debug_assert!(align_of::<U>() <= align_of::<Self>());

        // Get the object's new location in memory (this + 1)
        let dest = (self as *mut Self).add(1) as *mut U;
        self.value = dest as *mut T;

        // Store the object in memory
        std::ptr::write(dest, value);
    }
}

impl<T: ?Sized> Deref for DstField<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.value }
    }
}

impl<T: ?Sized> DerefMut for DstField<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.value }
    }
}

impl<T: ?Sized> Drop for DstField<T> {
    fn drop(&mut self) {
        unsafe { std::ptr::drop_in_place(self.value); }
    }
}

// This trait is unsafe for a few reasons
// 1) The 'DstField' field MUST BE THE LAST FIELD IN THE STRUCTURE
// 2) The type must use #[repr(C)]
pub unsafe trait Dst<T: ?Sized> {
    type InitArgs;

    unsafe fn init(args: Self::InitArgs) -> Self;

    fn get_dst_field(&mut self) -> &mut DstField<T>;
}

pub fn alloc_size<F: ?Sized, T: Dst<F>, U: Unsize<F>>() -> usize {
    size_of::<T>() + size_of::<U>()
}

#[cfg(test)]
mod tests {
    use super::{Dst, DstField};
    use super::super::linked_buffer::{LinkedBuffer, LBBox};

    trait MyTrait {
        fn get_type_name(&self) -> &'static str;
    }

    impl MyTrait for usize {
        fn get_type_name(&self) -> &'static str {
            "usize"
        }
    }

    #[repr(C)]
    struct MyDst {
        x: i32,
        dst: DstField<dyn MyTrait>,
    }

    unsafe impl Dst<dyn MyTrait> for MyDst {
        type InitArgs = (i32,);

        unsafe fn init(args: Self::InitArgs) -> Self {
            MyDst {
                x: args.0,
                dst: DstField::uninitialized(),
            }
        }

        fn get_dst_field(&mut self) -> &mut DstField<dyn MyTrait> {
            &mut self.dst
        }
    }

    #[test]
    fn test_alloc() {
        let buf = LinkedBuffer::default();
        let thing: LBBox<'_, MyDst> = buf.alloc_dst((1,), 5_usize);

        assert_eq!(thing.x, 1);
        assert_eq!(thing.dst.get_type_name(), "usize");
    }
}