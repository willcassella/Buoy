use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::util::fill::Fill;
use crate::core::element::*;

pub trait Socket: Fill<LayoutObj> {
}

impl<T: Fill<LayoutObj>> Socket for T {
}

// TODO: Should this be an arrayvec::ArrayString instead?
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct SocketName(pub u64);

impl<'a> From<&'a str> for SocketName {
    fn from(s: &'a str) -> Self {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        SocketName(hasher.finish())
    }
}
