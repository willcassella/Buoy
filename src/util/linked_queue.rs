use std::ops::{Deref, DerefMut};

use super::linked_buffer::{LinkedBuffer, LBBox};
use super::dst;

type Link<'buf, T> = LBBox<'buf, QNode<'buf, T>>;

#[repr(C)]
pub struct QNode<'buf, T> {
    next: Option<Link<'buf, T>>,
    value: T,
}

impl<'buf, T> QNode<'buf, T> {
    pub fn new(value: T) -> Self {
        QNode {
            next: None,
            value,
        }
    }

    pub fn into_inner(x: Self) -> T {
        x.value
    }
}

unsafe impl<'buf, F: ?Sized, T: dst::Dst<F>> dst::Dst<F> for QNode<'buf, T> {
    type InitArgs = T::InitArgs;

    unsafe fn init(args: Self::InitArgs) -> Self {
        QNode {
            next: None,
            value: T::init(args),
        }
    }

    fn get_dst_field(&mut self) -> &mut dst::DstField<F> {
        self.value.get_dst_field()
    }
}

impl<'buf, T> Deref for QNode<'buf, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

impl<'buf, T> DerefMut for QNode<'buf, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

pub struct Queue<'buf, T> {
    head: Option<Link<'buf, T>>,
    tail_next: *mut Option<Link<'buf, T>>,
}

impl<'buf, T> Default for Queue<'buf, T> {
    fn default() -> Self {
        Queue {
            head: None,
            tail_next: std::ptr::null_mut(),
        }
    }
}

impl<'buf, T> Queue<'buf, T> {
    pub fn push_back_node(&mut self, mut node: LBBox<'buf, QNode<'buf, T>>) {
        // If self.tail_next is null, then the tail_next pointer should be the head link
        let tail_next = if self.tail_next.is_null() {
            &mut self.head
        } else {
            self.tail_next
        };

        // There should be no way to get an orphaned node that has a next element
        // The point of this function is to enable intrusive linked lists, not O(n) insertion
        debug_assert!(node.next.is_none());
        self.tail_next = &mut node.next;

        // Write the value into the link (using std::ptr::write because we know *tail_next == None)
        unsafe { std::ptr::write(tail_next, Some(node)); }
    }

    pub fn append(&mut self, other: Self) {
        // Don't do anything if the list is empty
        if other.head.is_none() {
            return;
        }

        let tail_next = if self.tail_next.is_null() {
            &mut self.head
        } else {
            self.tail_next
        };

        unsafe { std::ptr::write(tail_next, other.head); }
        self.tail_next = other.tail_next;
    }

    pub fn pop_front_node(&mut self) -> Option<LBBox<QNode<'buf, T>>> {
        let mut node = match self.head.take() {
            Some(node) => node,
            None => return None,
        };

        // Update head and tail pointer
        self.head = node.next.take();
        if self.head.is_none() {
            self.tail_next = std::ptr::null_mut();
        }

        Some(node)
    }

    pub fn take(&mut self) -> Queue<'buf, T> {
        let result = Queue {
            head: self.head.take(),
            tail_next: self.tail_next,
        };
        self.tail_next = std::ptr::null_mut();
        result
    }

    pub fn push_back(&mut self, buf: &'buf LinkedBuffer, value: T) {
        let node = buf.alloc(QNode{ value, next: None });
        self.push_back_node(node);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.pop_front_node().map(|node| QNode::into_inner(LBBox::into_inner(node)))
    }
}

#[cfg(test)]
mod tests {
    use super::Queue;
    use super::super::linked_buffer::LinkedBuffer;

    #[test]
    fn test_order() {
        let buf = LinkedBuffer::default();
        let mut queue = Queue::default();

        queue.push_back(&buf, 1);
        queue.push_back(&buf, 2);
        queue.push_back(&buf, 3);

        assert_eq!(queue.pop_front(), Some(1));
        assert_eq!(queue.pop_front(), Some(2));
        assert_eq!(queue.pop_front(), Some(3));
        assert_eq!(queue.pop_front(), None);
    }
}