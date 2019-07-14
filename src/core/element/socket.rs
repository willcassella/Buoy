use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::core::element::*;
use crate::util::fill::Fill;

pub trait Socket: Fill<LayoutObj> {
    fn upcast(&self) -> &dyn Socket;

    fn upcast_mut(&mut self) -> &mut dyn Socket;
}

impl<T: Fill<LayoutObj>> Socket for T {
    fn upcast(&self) -> &dyn Socket {
        self
    }

    fn upcast_mut(&mut self) -> &mut dyn Socket {
        self
    }
}

// TODO: Should this be an arrayvec::ArrayString instead?
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct SocketName(pub u64);

impl SocketName {
    pub fn is_default(self) -> bool {
        self.0 == 0
    }
}

impl<'a> From<&'a str> for SocketName {
    fn from(s: &'a str) -> Self {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        SocketName(hasher.finish())
    }
}
