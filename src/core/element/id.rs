use std::hash::{Hasher, Hash};
use std::fmt::{self, Display, Formatter};
use std::collections::hash_map::DefaultHasher;

#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Id(u64);

impl Id {
    pub fn append(self, id: Id) -> Self {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        id.hash(&mut hasher);

        Id(hasher.finish())
    }

    pub fn append_str(self, id: &str) -> Self {
        self.append(Id::from(id))
    }

    pub fn append_num(self, id: u64) -> Self {
        self.append(Id::from(id))
    }
}

impl<'a> From<&'a str> for Id {
    fn from(id: &'a str) -> Self {
        let mut hasher = DefaultHasher::new();
        id.hash(&mut hasher);

        Id(hasher.finish())
    }
}

impl From<u64> for Id {
    fn from(id: u64) -> Self {
        let mut hasher = DefaultHasher::new();
        id.hash(&mut hasher);

        Id(hasher.finish())
    }
}

impl Display for Id {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.0, fmt)
    }
}
