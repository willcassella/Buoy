use std::any::Any;
use std::collections::hash_map::HashMap;
use std::marker::PhantomData;

use crate::core::id::Id;

pub trait Message: Clone + Send + Any {}

impl<T: Clone + Send + Any> Message for T {}

pub type MessageMap = HashMap<Id, Box<dyn Any + Send>>;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Inbox<T: Message> {
    id: Id,
    _phantom: PhantomData<T>,
}

impl<T: Message> Inbox<T> {
    pub(crate) fn new(id: Id) -> Self {
        Inbox {
            id,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn id(&self) -> Id {
        self.id
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Outbox<T: Message> {
    id: Id,
    _phantom: PhantomData<T>,
}

impl<T: Message> Outbox<T> {
    pub(crate) fn new(id: Id) -> Self {
        Outbox {
            id,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn id(&self) -> Id {
        self.id
    }
}
