use std::any::Any;
use std::collections::hash_map::HashMap;
use std::marker::PhantomData;

use crate::core::id::Id;

pub trait Message: Clone + Send + Any {}

impl<T: Clone + Send + Any> Message for T {}

#[derive(Default)]
pub struct MessageMap {
    map: HashMap<Id, Box<dyn Any + Send>>,
}

impl MessageMap {
    pub fn read<T: Message>(&self, inbox: Inbox<T>) -> Option<T> {
        let value = match self.map.get(&inbox.id()) {
            Some(value) => &**value,
            None => return None,
        };

        value.downcast_ref::<T>().cloned()
    }

    pub fn write<T: Message>(&mut self, outbox: Outbox<T>, value: T) {
        match outbox.mapping {
            Some(mapping) => mapping(value, MessageWriter { message_map: self }),
            None => {
                self.map.insert(outbox.id(), Box::new(value));
            }
        };
    }

    pub fn extend(&mut self, other: &mut MessageMap) {
        self.map.extend(other.map.drain());
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }
}

pub struct MessageWriter<'a> {
    message_map: &'a mut MessageMap,
}

impl<'a> MessageWriter<'a> {
    pub fn write<T: Message>(&mut self, outbox: Outbox<T>, value: T) {
        self.message_map.write(outbox, value);
    }

    pub fn reborrow<'b>(&'b mut self) -> MessageWriter<'b> {
        MessageWriter {
            message_map: self.message_map,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Inbox<T: Message> {
    id: Id,
    _phantom: PhantomData<T>,
}

impl<T: Message> Inbox<T> {
    pub(in crate::core) fn new(id: Id) -> Self {
        Inbox {
            id,
            _phantom: PhantomData,
        }
    }

    pub(in crate::core) fn id(&self) -> Id {
        self.id
    }
}

#[repr(C)]
pub struct Outbox<T: Message> {
    id: Id,
    mapping: Option<Box<dyn FnOnce(T, MessageWriter)>>,
}

impl<T: Message> Outbox<T> {
    pub(in crate::core) fn new(id: Id) -> Self {
        Outbox { id, mapping: None }
    }

    pub(in crate::core) fn id(&self) -> Id {
        self.id
    }

    pub fn map_from<I: Message, F: FnOnce(I, MessageWriter) -> T + 'static>(
        self,
        mapping: F,
    ) -> Outbox<I> {
        Outbox {
            id: self.id,
            mapping: Some(Box::new(move |v, mut writer| {
                let v = mapping(v, writer.reborrow());
                writer.write(self, v);
            })),
        }
    }
}
