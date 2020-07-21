use std::fmt::{self, Display, Formatter};
use std::hash::{Hash, Hasher};
use fnv::FnvHasher;

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, Hash, Eq, PartialEq)]
pub struct Id(u64);

impl Id {
    pub fn append<T: Into<Id>>(self, id: T) -> Self {
        let mut hasher = FnvHasher::with_key(self.0);
        id.into().hash(&mut hasher);

        Id(hasher.finish())
    }
}

impl<'a> From<&'a str> for Id {
    fn from(id: &'a str) -> Self {
        let mut hasher = FnvHasher::default();
        id.hash(&mut hasher);

        Id(hasher.finish())
    }
}

impl From<u64> for Id {
    fn from(id: u64) -> Self {
        let mut hasher = FnvHasher::default();
        id.hash(&mut hasher);

        Id(hasher.finish())
    }
}

impl Display for Id {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.0, fmt)
    }
}
