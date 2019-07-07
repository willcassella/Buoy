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
pub struct InputId {
    pub(super) frame_id: FrameId,
    pub(super) context_id: ContextId,
    pub(super) cookie: Cookie,
}

impl InputId {
    pub fn new(frame_id: FrameId, context_id: ContextId) -> Self {
        InputId {
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

pub trait InputState: Clone + Send + Any + Default {}

impl<T: Clone + Send + Any + Default> InputState for T {}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Input<T: InputState> {
    pub(super) id: InputId,
    _phantom: PhantomData<T>,
}

impl<T: InputState> Input<T> {
    pub fn new(id: InputId) -> Self {
        Input {
            id,
            _phantom: PhantomData,
        }
    }
}

pub type InputCache = HashMap<InputId, Box<Any>>;
