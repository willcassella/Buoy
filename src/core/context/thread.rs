use crate::core::context::GuiContext;
use crate::core::message::{Message, MessageMap, Outbox};
use crate::util::arena::Arena;
use std::cell::RefCell;
use std::collections::hash_map::{Entry, HashMap};

pub struct ThreadContext<'frm> {
    outgoing_messages: RefCell<MessageMap>,
    buffer: &'frm Arena,
}

impl<'frm> ThreadContext<'frm> {
    pub fn new(buffer: &'frm Arena) -> Self {
        ThreadContext {
            outgoing_messages: Default::default(),
            buffer,
        }
    }

    pub fn buffer(&self) -> &'frm Arena {
        self.buffer
    }

    pub fn write_message<T: Message>(&self, outbox: Outbox<T>, value: T) {
        self.outgoing_messages.borrow_mut().write(outbox, value);
    }

    pub fn take_outgoing_messages(&mut self) -> MessageMap {
        std::mem::take(&mut self.outgoing_messages.borrow_mut())
    }
}
