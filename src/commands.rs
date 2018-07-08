use color::Color;
use layout::Region;

#[repr(C)]
#[derive(Default, Clone, Copy, Debug)]
pub struct Quad {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Quad {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Quad {
            x,
            y,
            width,
            height,
        }
    }
}

impl From<Region> for Quad {
    fn from(v: Region) -> Self {
        Quad {
            x: v.pos.x,
            y: v.pos.y,
            width: v.area.width,
            height: v.area.height,
        }
    }
}

#[repr(C)]
#[derive(Default, Clone, Copy, Debug)]
pub struct ColoredQuad {
    pub quad: Quad,
    pub color: Color,
}

impl ColoredQuad {
    pub fn new(quad: Quad, color: Color) -> Self {
        ColoredQuad {
            quad,
            color,
        }
    }
}

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
