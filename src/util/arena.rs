use std::any::Any;
use std::cell::UnsafeCell;
use std::convert::From;
use std::marker::PhantomData;
use std::mem::{align_of, size_of};
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull; // TODO: Switch to std::ptr::Unique when stabilized

use super::upcast::Upcast;

const BUFFER_SIZE: usize = 1_usize << 16;

#[repr(align(16))]
struct Node {
    buf: [u8; BUFFER_SIZE],
    prev: Option<Box<Node>>,
}

impl Node {
    fn alloc() -> Box<Self> {
        Box::new(Self {
            buf: unsafe { core::mem::MaybeUninit::uninit().assume_init() },
            prev: None,
        })
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        let mut prev = self.prev.take();
        while let Some(mut x) = prev.take() {
            prev = x.prev.take();
        }
    }
}

struct ArenaInner {
    head: Option<Box<Node>>,
    offset: usize,
}

impl ArenaInner {
    fn new() -> Self {
        ArenaInner {
            head: None,
            offset: 0,
        }
    }

    fn clear(&mut self) {
        if let Some(ref mut head) = self.head {
            head.prev = None;
        }
        self.offset = 0;
    }

    unsafe fn alloc_new_node(size: usize, align: usize) -> (Box<Node>, usize, *mut ()) {
        let mut node = Node::alloc();
        let start = node.buf.as_mut_ptr();

        // Align the pointer for writing
        let dest_offset = start.align_offset(align);
        let new_offset = dest_offset + size;
        debug_assert!(new_offset <= BUFFER_SIZE);

        let dest = start.add(dest_offset) as *mut ();
        (node, new_offset, dest)
    }

    unsafe fn alloc_raw(&mut self, size: usize, align: usize) -> NonNull<()> {
        match self.head {
            Some(ref mut node) => {
                // Get the destination offset
                let start = node.buf.as_mut_ptr();
                let dest_offset = self.offset + start.add(self.offset).align_offset(align);

                // If we've exceeded the bounds of our buffer, allocate into a new node
                if dest_offset + size > BUFFER_SIZE {
                    let (mut node, offset, ptr) = ArenaInner::alloc_new_node(size, align);

                    // Update self
                    node.prev = self.head.take();
                    self.head = Some(node);
                    self.offset = offset;

                    return NonNull::new(ptr).unwrap();
                }

                let dest = start.add(dest_offset) as *mut ();

                // Update self
                self.offset = dest_offset + size;

                NonNull::new(dest).unwrap()
            }
            None => {
                let (node, offset, dest) = ArenaInner::alloc_new_node(size, align);

                // Update self
                self.head = Some(node);
                self.offset = offset;

                NonNull::new(dest).unwrap()
            }
        }
    }

    unsafe fn alloc_typed<T>(&mut self) -> NonNull<T> {
        let ptr = self.alloc_raw(size_of::<T>(), align_of::<T>());
        NonNull::new_unchecked(ptr.as_ptr() as *mut T)
    }
}

impl Default for ArenaInner {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default)]
pub struct Arena {
    inner: UnsafeCell<ArenaInner>,
}

impl Arena {
    pub fn new() -> Self {
        Arena {
            inner: UnsafeCell::new(ArenaInner::new()),
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            let inner = &mut *self.inner.get();
            inner.clear();
        }
    }

    pub fn alloc<T>(&self, value: T) -> ABox<'_, T> {
        let ptr = unsafe {
            let inner = &mut *self.inner.get();
            let dest = inner.alloc_typed::<T>();
            std::ptr::write(dest.as_ptr(), value);
            dest
        };

        ABox {
            value: ptr,
            _phantom: PhantomData,
        }
    }

    // This allows you to initialize a composite structure of LBBox's with a single allocation
    pub fn alloc_composite1<'a, T1, T2, I2>(&'a self, t1: T1, i2: I2) -> ABox<'a, T2>
    where
        I2: FnOnce(ABox<'a, T1>) -> T2,
    {
        unsafe {
            let inner = &mut *self.inner.get();
            let mut dest = inner.alloc_typed::<(T2, T1)>();

            // Write t1 into memory
            std::ptr::write(&mut dest.as_mut().1, t1);
            let t1 = ABox {
                value: NonNull::from(&mut dest.as_mut().1),
                _phantom: PhantomData,
            };

            // Initialize t2 with t1
            std::ptr::write(&mut dest.as_mut().0, i2(t1));
            ABox {
                value: NonNull::from(&mut dest.as_mut().0),
                _phantom: PhantomData,
            }
        }
    }
}

pub struct ABox<'a, T: ?Sized> {
    value: NonNull<T>,
    _phantom: PhantomData<&'a ()>,
}

impl<'a, T> ABox<'a, T> {
    pub fn into_inner(x: Self) -> T {
        let value = unsafe { std::ptr::read(x.value.as_ptr()) };
        std::mem::forget(x);
        value
    }
}

impl<'a, T: ?Sized> ABox<'a, T> {
    pub fn forget_inner(x: Self) {
        // Currently no memory to deallocate
        std::mem::forget(x);
    }

    pub fn upcast<F: ?Sized>(mut x: Self) -> ABox<'a, F>
    where
        T: Upcast<F>,
    {
        let result = ABox {
            value: unsafe { NonNull::from(x.value.as_mut().upcast_mut()) },
            _phantom: PhantomData,
        };
        std::mem::forget(x);
        result
    }
}

impl<'a, T: ?Sized + Upcast<dyn Any>> ABox<'a, T> {
    pub fn downcast<U: Any>(mut x: Self) -> Result<ABox<'a, U>, Self> {
        match (*x).upcast_mut().downcast_mut::<U>() {
            Some(u) => {
                let result = ABox {
                    value: NonNull::from(u),
                    _phantom: PhantomData,
                };
                std::mem::forget(x);
                Ok(result)
            }
            None => Err(x),
        }
    }
}

impl<'a, T: ?Sized> Drop for ABox<'a, T> {
    fn drop(&mut self) {
        unsafe {
            std::ptr::drop_in_place(self.value.as_ptr());
        }
    }
}

impl<'a, T: ?Sized> Deref for ABox<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.value.as_ref() }
    }
}

impl<'a, T: ?Sized> DerefMut for ABox<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.value.as_mut() }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::arena::Arena;

    #[test]
    fn try_alloc() {
        let mut buf = Arena::new();

        {
            let one = buf.alloc(1);
            let two = buf.alloc("two");
            let three = buf.alloc(3.0);

            dbg!(*one);
            assert_eq!(*one, 1);
            dbg!(*two);
            assert_eq!(*two, "two");
            dbg!(*three);
            assert_eq!(*three, 3.0);
        }

        buf.clear();

        {
            let four = buf.alloc("four");
            let five = buf.alloc(Box::new(5_usize));
            let six = buf.alloc([6; 3]);
            let seven = buf.alloc(());

            dbg!(*four);
            assert_eq!(*four, "four");
            dbg!(&*five);
            assert_eq!(*five, Box::new(5_usize));
            dbg!(*six);
            assert_eq!(*six, [6; 3]);
            dbg!(*seven);
            assert_eq!(*seven, ());
        }
    }
}
