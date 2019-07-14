use std::marker::{PhantomData, Unsize};
use std::ptr;
use std::ops::{Deref, DerefMut};
use std::cell::UnsafeCell;
use std::ops::CoerceUnsized;
use std::mem::{size_of, align_of};

use super::dst;

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

struct LinkedBufferInner {
    head: Option<Box<Node>>,
    offset: usize,
}

impl LinkedBufferInner {
    fn new() -> Self {
        LinkedBufferInner {
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

    unsafe fn alloc_raw(&mut self, size: usize, align: usize) -> *mut () {
        match self.head {
            Some(ref mut node) => {
                // Get the destination offset
                let start = node.buf.as_mut_ptr();
                let dest_offset = self.offset + start.add(self.offset).align_offset(align);

                // If we've exceeded the bounds of our buffer, allocate into a new node
                if dest_offset + size > BUFFER_SIZE {
                    let (mut node, offset, ptr) = LinkedBufferInner::alloc_new_node(size, align);

                    // Update self
                    node.prev = self.head.take();
                    self.head = Some(node);
                    self.offset = offset;

                    return ptr;
                }

                let dest = start.add(dest_offset) as *mut ();

                // Update self
                self.offset = dest_offset + size;

                dest
            },
            None => {
                let (node, offset, dest) = LinkedBufferInner::alloc_new_node(size, align);

                // Update self
                self.head = Some(node);
                self.offset = offset;

                dest
            }
        }
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
}

impl Default for LinkedBufferInner {
    fn default() -> Self {
        LinkedBufferInner::new()
    }
}

#[derive(Default)]
pub struct LinkedBuffer {
    inner: UnsafeCell<LinkedBufferInner>,
}

impl LinkedBuffer {
    pub fn new() -> Self {
        LinkedBuffer {
            inner: UnsafeCell::new(LinkedBufferInner::new()),
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            let inner = &mut *self.inner.get();
            inner.clear();
        }
    }

    pub fn alloc<'a, T>(&'a self, value: T) -> LBBox<'a, T> {
        let ptr = unsafe {
            let inner = &mut *self.inner.get();
            let dest = inner.alloc_raw(size_of::<T>(), align_of::<T>()) as *mut T;
            std::ptr::write(dest, value);
            dest
        };

        LBBox {
            value: ptr,
            _phantom: PhantomData,
        }
    }

    // TODO: Figure out how Pin fits into this
    pub fn alloc_dst<'a, F: ?Sized, T: dst::Dst<F>, U: Unsize<F>>(
        &'a self,
        args: T::InitArgs,
        value: U,
    ) -> LBBox<'a, T> {
        let ptr = unsafe {
            let inner = &mut *self.inner.get();

            // Allocate memory for the dst
            let dest = inner.alloc_raw(dst::alloc_size::<F, T, U>(), std::mem::align_of::<T>()) as *mut T;

            // Construct the dst
            std::ptr::write(dest, T::init(args));

            // Initialize the dst field
            let dst_field = (*dest).get_dst_field();
            dst_field.init(value);

            dest
        };

        LBBox {
            value: ptr,
            _phantom: PhantomData,
        }
    }
}

pub struct LBBox<'a, T: ?Sized> {
    value: *mut T,
    _phantom: PhantomData<&'a ()>,
}

impl<'a, T> LBBox<'a, T> {
    pub fn into_inner(b: Self) -> T {
        let value = unsafe { std::ptr::read(b.value) };
        std::mem::forget(b);
        value
    }
}

impl<'a, T, U: ?Sized> CoerceUnsized<LBBox<'a, U>> for LBBox<'a, T>
where
    T: Unsize<U>
{
}

impl<'a, T: ?Sized> Drop for LBBox<'a, T> {
    fn drop(&mut self) {
        unsafe { ptr::drop_in_place(self.value); }
    }
}

impl<'a, T: ?Sized> Deref for LBBox<'a, T> {
    type Target = T;

    fn deref<'b>(&'b self) -> &'b T {
        unsafe { &*self.value }
    }
}

impl<'a, T: ?Sized> DerefMut for LBBox<'a, T> {
    fn deref_mut<'b>(&'b mut self) -> &'b mut T {
        unsafe { &mut *self.value }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedBuffer;

    #[test]
    fn try_alloc() {
        let mut buf = LinkedBuffer::new();

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

            dbg!(*four);
            assert_eq!(*four, "four");
            dbg!(&*five);
            assert_eq!(*five, Box::new(5_usize));
            dbg!(*six);
            assert_eq!(*six, [6; 3]);
        }
    }
}