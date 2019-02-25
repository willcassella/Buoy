use arrayvec::ArrayString;

pub type Name = arrayvec::ArrayString<[u8; 32]>;

#[derive(Clone, Copy, Debug)]
pub enum Anchor {
    Any,
    Named(Name),
}

impl Anchor {
    pub fn any() -> Self {
        Anchor::Any
    }

    pub fn named(name: &str) -> Self {
        Anchor::Named(Name::from(name).expect("Name too long"))
    }
}

impl Default for Anchor {
    fn default() -> Self {
        Anchor::Named(ArrayString::default())
    }
}
