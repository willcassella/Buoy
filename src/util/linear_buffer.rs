use std::marker::{PhantomData, Unsize};
use std::ptr;
use std::ops::{Deref, DerefMut};
use std::cell::UnsafeCell;
use std::ops::CoerceUnsized;

const BUFFER_SIZE: usize = 1_usize << 16;

#[repr(align(16))]
struct Node {
    buf: [u8; BUFFER_SIZE],
    prev: Option<Box<Node>>,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            buf: unsafe { core::mem::MaybeUninit::uninit().assume_init() },
            prev: None,
        }
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

struct LinearBufferInner {
    head: Option<Box<Node>>,
    offset: usize,
}

impl LinearBufferInner {
    fn new() -> Self {
        LinearBufferInner {
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

    unsafe fn alloc<T>(&mut self, value: T) -> *mut T {
        match self.head {
            Some(ref mut node) => {
                // Get the destination offset
                let start = node.buf.as_mut_ptr();
                let dest_offset = self.offset + start.add(self.offset).align_offset(std::mem::align_of::<T>());

                // If we've exceeded the bounds of our buffer, allocate into a new node
                if dest_offset + std::mem::size_of::<T>() > BUFFER_SIZE {
                    let (mut node, offset, ptr) = LinearBufferInner::alloc_new_node(value);

                    // Update self
                    node.prev = self.head.take();
                    self.head = Some(node);
                    self.offset = offset;

                    return ptr;
                }

                let dest = start.add(dest_offset) as *mut T;
                dest.write(value);

                // Update self
                self.offset = dest_offset + std::mem::size_of::<T>();

                dest
            },
            None => {
                let (node, offset, ptr) = LinearBufferInner::alloc_new_node(value);

                // Update self
                self.head = Some(node);
                self.offset = offset;

                ptr
            }
        }
    }

    unsafe fn alloc_new_node<T>(value: T) -> (Box<Node>, usize, *mut T) {
        let mut node = Box::new(Node::default());
        let start = node.buf.as_mut_ptr();

        // Align the pointer for writing (assume it fits)
        let dest_offset = start.align_offset(std::mem::align_of::<T>());
        let dest = start.add(dest_offset) as *mut T;

        // Write the value into the pointer
        dest.write(value);

        (node, dest_offset + std::mem::size_of::<T>(), dest)
    }
}

impl Default for LinearBufferInner {
    fn default() -> Self {
        LinearBufferInner::new()
    }
}

#[derive(Default)]
pub struct LinearBuffer {
    inner: UnsafeCell<LinearBufferInner>,
}

impl LinearBuffer {
    pub fn new() -> Self {
        LinearBuffer {
            inner: UnsafeCell::new(LinearBufferInner::new()),
        }
    }

    pub fn clear(&mut self) {
        unsafe { (*self.inner.get()).clear() }
    }

    pub fn alloc<'a, T>(&'a self, value: T) -> LinearBufferBox<'a, T> {
        let ptr = unsafe { (*self.inner.get()).alloc(value) };

        LinearBufferBox {
            value: ptr,
            _phantom: PhantomData,
        }
    }
}

pub struct LinearBufferBox<'a, T: ?Sized> {
    value: *mut T,
    _phantom: PhantomData<&'a ()>,
}

impl<'a, T, U: ?Sized> CoerceUnsized<LinearBufferBox<'a, U>> for LinearBufferBox<'a, T>
where
    T: Unsize<U>
{
}

impl<'a, T: ?Sized> Drop for LinearBufferBox<'a, T> {
    fn drop(&mut self) {
        unsafe { ptr::drop_in_place(self.value); }
    }
}

impl<'a, T: ?Sized> Deref for LinearBufferBox<'a, T> {
    type Target = T;

    fn deref<'b>(&'b self) -> &'b T {
        unsafe { &*self.value }
    }
}

impl<'a, T: ?Sized> DerefMut for LinearBufferBox<'a, T> {
    fn deref_mut<'b>(&'b mut self) -> &'b mut T {
        unsafe { &mut *self.value }
    }
}

#[cfg(test)]
mod tests {
    use super::LinearBuffer;

    #[test]
    fn try_alloc() {
        let mut buf = LinearBuffer::new();

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