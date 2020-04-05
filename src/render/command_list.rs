pub use super::commands::{ClickQuad, ColoredQuad, HoverQuad};

#[derive(Default)]
pub struct CommandList {
    pub colored_quads: Vec<ColoredQuad>,
    pub hover_quads: Vec<HoverQuad>,
    pub click_quads: Vec<ClickQuad>,
}

impl CommandList {
    pub fn clear(&mut self) {
        self.colored_quads.clear();
        self.hover_quads.clear();
        self.click_quads.clear();
    }

    pub fn add_colored_quads<I: IntoIterator<Item = ColoredQuad>>(&mut self, colored_quads: I) {
        self.colored_quads.extend(colored_quads);
    }

    pub fn add_hover_quads<I: IntoIterator<Item = HoverQuad>>(&mut self, hover_quads: I) {
        self.hover_quads.extend(hover_quads);
    }

    pub fn add_click_quads<I: IntoIterator<Item = ClickQuad>>(&mut self, click_quads: I) {
        self.click_quads.extend(click_quads);
    }
}
