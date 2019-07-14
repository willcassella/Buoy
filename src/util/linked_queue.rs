use super::linked_buffer::{
    LinkedBuffer,
    LinkedBufferBox
};

type Link<'buf, T> = LinkedBufferBox<'buf, Node<'buf, T>>;

pub struct Node<'buf, T> {
    next: Option<Link<'buf, T>>,
    value: T,
}

impl<'buf, T> Node<'buf, T> {
    pub fn new(value: T) -> Self {
        Node {
            next: None,
            value,
        }
    }
}

pub struct QueueBuilder<'buf, T> {
    head: Option<Link<'buf, T>>,
    tail_next: *mut Option<Link<'buf, T>>,
}

impl<'buf, T> Default for QueueBuilder<'buf, T> {
    fn default() -> Self {
        QueueBuilder {
            head: None,
            tail_next: std::ptr::null_mut(),
        }
    }
}

impl<'buf, T> QueueBuilder<'buf, T> {
    pub fn push_back(&mut self, buf: &'buf LinkedBuffer, value: T) {
        let node = buf.alloc(Node{ value, next: None });
        self.push_back_node(node);
    }

    pub fn push_back_node(&mut self, mut node: LinkedBufferBox<'buf, Node<'buf, T>>) {
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

    pub fn finish(self) -> Queue<'buf, T> {
        Queue{ next: self.head }
    }
}

pub struct Queue<'buf, T> {
    next: Option<Link<'buf, T>>,
}

impl<'buf, T> Queue<'buf, T> {
    pub fn pop_front(&mut self) -> Option<T> {
        let node = match self.next.take() {
            Some(node) => LinkedBufferBox::into_inner(node),
            None => return None,
        };

        self.next = node.next;
        Some(node.value)
    }
}