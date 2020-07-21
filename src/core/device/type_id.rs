use std::fmt;

// Generated as a v4 uuid. Usually precomputed.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct TypeId {
    id: u128,
}

impl TypeId {
    pub const fn new(id: u128) -> Self {
        TypeId { id }
    }
}

impl fmt::Display for TypeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl fmt::Debug for TypeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}
