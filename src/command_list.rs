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

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Color(pub u32);

impl Default for Color {
    fn default() -> Self {
        constants::WHITE
    }
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        let v = (red as u32) << 24 | (green as u32) << 16 | (blue as u32) << 8 | (alpha as u32);
        return Color(v);
    }

    pub fn red(self) -> u8 {
        (self.0 >> 24) as u8
    }

    pub fn set_red(&mut self, red: u8) {
        let red = (red as u32) << 24 | 0xFFFFFF;
        self.0 &= red;
    }

    pub fn green(self) -> u8 {
        (self.0 >> 16) as u8
    }

    pub fn set_green(&mut self, green: u8) {
        let green = (green as u32) << 16 | 0xFF00FFFF;
        self.0 &= green;
    }

    pub fn blue(self) -> u8 {
        (self.0 >> 8) as u8
    }

    pub fn set_blue(&mut self, blue: u8) {
        let blue = (blue as u32) << 8 | 0xFFFF00FF;
        self.0 &= blue;
    }

    pub fn alpha(self) -> u8 {
        self.0 as u8
    }

    pub fn set_alpha(&mut self, alpha: u8) {
        let blue = (alpha as u32) | 0xFFFFFF00;
        self.0 &= blue;
    }
}

pub mod constants {
    use super::Color;

    pub const RED: Color = Color(0xFF0000FF);
    pub const GREEN: Color = Color(0x00FF00FF);
    pub const BLUE: Color = Color(0x0000FFFF);
    pub const BLACK: Color = Color(0x000000FF);
    pub const WHITE: Color = Color(0xFFFFFFFF);
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
    colored_quads: Vec<ColoredQuad>,
}

impl CommandList {
    pub fn add_colored_quads(&mut self, colored_quads: &[ColoredQuad]) {
        self.colored_quads.extend_from_slice(colored_quads);
    }

    pub fn get_colored_quads(&self) -> &Vec<ColoredQuad> {
        &self.colored_quads
    }
}
