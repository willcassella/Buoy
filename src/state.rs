use std::any::Any;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::num::Wrapping;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Hash)]
pub struct FrameId(pub Wrapping<u32>);

impl FrameId {
    pub fn next(self) -> Self {
        FrameId(self.0 + Wrapping(1_u32))
    }

    pub fn prev(self) -> Self {
        FrameId(self.0 - Wrapping(1_u32))
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Hash)]
pub struct ContextId(pub u16);

impl ContextId {
    pub fn increment(&mut self) -> Self {
        let temp = *self;
        self.0 += 1;
        temp
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Hash)]
pub struct Cookie(pub u16);

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Hash)]
pub struct StateId {
    pub(super) frame_id: FrameId,
    pub(super) context_id: ContextId,
    pub(super) cookie: Cookie,
}

impl StateId {
    pub fn new(frame_id: FrameId, context_id: ContextId) -> Self {
        StateId {
            frame_id,
            context_id,
            cookie: Cookie(1),
        }
    }

    pub fn increment(&mut self) -> Self {
        let temp = *self;
        self.cookie.0 += 1;
        temp
    }
}

pub trait StateT: Clone + Send + Any + Default {}

impl<T: Clone + Send + Any + Default> StateT for T {}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct State<T: StateT> {
    pub(super) id: StateId,
    _phantom: PhantomData<T>,
}

impl<T: StateT> State<T> {
    pub fn new(id: StateId) -> Self {
        State {
            id,
            _phantom: PhantomData,
        }
    }
}

pub type StateCache = HashMap<StateId, Box<Any>>;
