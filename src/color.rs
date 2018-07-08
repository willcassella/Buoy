#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    pub const TRANSPARENT: Color = Color(0x00000000);
}