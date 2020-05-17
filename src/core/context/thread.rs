use crate::core::context::GuiContext;
use crate::core::device::{RendererWrapper, TypeId};
use crate::core::message::{Message, MessageMap, Outbox};
use crate::util::arena::Arena;
use std::cell::RefCell;
use std::collections::hash_map::{Entry, HashMap};

pub struct ThreadContext<'frm, C> {
    // TODO: Eventually replace these with UnsafeCell
    renderers: RefCell<HashMap<TypeId, Box<dyn RendererWrapper<'frm, C> + 'frm>>>,
    outgoing_messages: RefCell<MessageMap>,
    buffer: &'frm Arena,
}

impl<'frm, C: 'static> ThreadContext<'frm, C> {
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
        gui: &'frm GuiContext<C>,
        type_id: TypeId,
    ) -> &'thrd dyn RendererWrapper<'frm, C> {
        let mut renderers = self.renderers.borrow_mut();
        let value = match renderers.entry(type_id) {
            Entry::Occupied(entry) => &**entry.into_mut(),
            Entry::Vacant(entry) => {
                let renderer_factory = gui
                    .renderers
                    .get(&type_id)
                    .ok_or_else(|| format!("No renderer registered for {}", type_id))
                    .unwrap();
                &**entry.insert(renderer_factory.into_renderer())
            }
        };

        // Safe because ABox's can be moved around without invalidating references to their contents
        // (so reallocating the HashMap won't invalidate the returned reference)
        // and nothing will be removed from the HashMap until this ThreadContext is destroyed.
        unsafe { &*(value as *const dyn RendererWrapper<'frm, C>) }
    }

    pub fn write_message<T: Message>(&self, outbox: Outbox<T>, value: T) {
        self.outgoing_messages.borrow_mut().write(outbox, value);
    }

    pub fn take_outgoing_messages(&mut self) -> MessageMap {
        std::mem::take(&mut self.outgoing_messages.borrow_mut())
    }
}
