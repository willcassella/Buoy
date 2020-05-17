use arrayvec::ArrayString;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct TypeId {
    pub package: ArrayString<[u8; 4]>,
    pub name: ArrayString<[u8; 12]>,
}

impl TypeId {
    pub fn new(package: &str, name: &str) -> Self {
        // TODO: Have better error handling here
        TypeId {
            package: ArrayString::from(package).unwrap(),
            name: ArrayString::from(name).unwrap(),
        }
    }

    pub fn null() -> Self {
        TypeId {
            package: Default::default(),
            name: Default::default(),
        }
    }
}

impl fmt::Display for TypeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.package, self.name)
    }
}

impl fmt::Debug for TypeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}
