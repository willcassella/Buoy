use arrayvec::ArrayString;

#[derive(Clone, Copy, Default)]
pub struct Anchor(ArrayString<[u8; 32]>);

impl Anchor {
    pub fn from(name: &str) -> Self {
        Anchor(ArrayString::from(name).expect("Name too long"))
    }
}
