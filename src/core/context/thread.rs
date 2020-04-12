use crate::core::component::{Renderer, TypeId};
use crate::core::context::Window;
use crate::core::message::{Message, MessageMap, Outbox};
use crate::util::arena::{ABox, Arena};
use std::cell::RefCell;
use std::collections::hash_map::{Entry, HashMap};

pub struct ThreadContext<'frm> {
    // TODO: Eventually replace these with UnsafeCell
    renderers: RefCell<HashMap<TypeId, ABox<'frm, dyn Renderer<'frm>>>>,
    outgoing_messages: RefCell<MessageMap>,
    buffer: &'frm Arena,
}

impl<'frm> ThreadContext<'frm> {
    pub fn new(buffer: &'frm Arena) -> Self {
        ThreadContext {
            renderers: Default::default(),
            outgoing_messages: Default::default(),
            buffer,
        }
    }

    pub fn buffer(&self) -> &'frm Arena {
        self.buffer
    }

    pub fn renderer_for<'thrd>(
        &'thrd self,
        window: &Window,
        type_id: TypeId,
    ) -> &'thrd dyn Renderer<'frm> {
        let mut renderers = self.renderers.borrow_mut();
        let value = match renderers.entry(type_id) {
            Entry::Occupied(entry) => &**entry.into_mut(),
            Entry::Vacant(entry) => {
                let renderer_factory = window
                    .renderers
                    .get(&type_id)
                    .expect("No such renderer registered");
                &**entry.insert(renderer_factory.create_renderer(type_id, &self.buffer))
            }
        };

        // Safe because ABox's inside the HashMap have 'frm (greater than 'thrd) lifetime
        // And nothing will be removed from the HashMap until this ThreadContext is destroyed.
        unsafe { std::mem::transmute(value) }
    }

    pub fn write_message<T: Message>(&self, outbox: Outbox<T>, value: T) {
        self.outgoing_messages.borrow_mut().write(outbox, value);
    }

    pub fn take_outgoing_messages(&mut self) -> MessageMap {
        std::mem::take(&mut self.outgoing_messages.borrow_mut())
    }
}
