pub use super::commands::{ColoredQuad};

#[derive(Default)]
pub struct CommandList {
    pub colored_quads: Vec<ColoredQuad>,
}

impl CommandList {
    pub fn add_colored_quads(&mut self, colored_quads: &[ColoredQuad]) {
        self.colored_quads.extend_from_slice(colored_quads);
    }

    pub fn get_colored_quads(&self) -> &Vec<ColoredQuad> {
        &self.colored_quads
    }
}