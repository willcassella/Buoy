pub use super::commands::{ColoredQuad, HoverQuad};

#[derive(Default)]
pub struct CommandList {
    pub colored_quads: Vec<ColoredQuad>,
    pub hover_quads: Vec<HoverQuad>,
}

impl CommandList {
    pub fn clear(&mut self) {
        self.colored_quads.clear();
        self.hover_quads.clear();
    }

    pub fn add_colored_quads(&mut self, colored_quads: &[ColoredQuad]) {
        self.colored_quads.extend_from_slice(colored_quads);
    }

    pub fn get_colored_quads(&self) -> &Vec<ColoredQuad> {
        &self.colored_quads
    }

    pub fn add_hover_quads<I: Iterator<Item = HoverQuad>>(&mut self, hover_quads: I) {
        self.hover_quads.extend(hover_quads)
    }

    pub fn get_hover_quads(&self) -> &Vec<HoverQuad> {
        &self.hover_quads
    }
}
