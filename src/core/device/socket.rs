use fnv::FnvHasher;
use std::hash::{Hash, Hasher};

use crate::core::context::LayoutNode;
use crate::util::fill::Fill;

pub trait Socket: Fill<LayoutNode> {}

impl<T: Fill<LayoutNode>> Socket for T {}

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
        let mut hasher = FnvHasher::default();
        s.hash(&mut hasher);
        SocketName(hasher.finish())
    }
}

pub enum SocketCapacity {
    One,
    Infinite,
    Finite(usize),
}
