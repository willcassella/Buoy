use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::util::fill::Fill;
use crate::element::UIRender;

pub trait UISocket: Fill<UIRender> {
}

impl<T: Fill<UIRender>> UISocket for T {
}

#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct Id(pub u64);

impl<'a> From<&'a str> for Id {
    fn from(s: &'a str) -> Self {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        Id(hasher.finish())
    }
}
