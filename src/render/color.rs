#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RGBA8(pub u32);

impl Default for RGBA8 {
    fn default() -> Self {
        constants::WHITE
    }
}

impl RGBA8 {
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        let v = u32::from(red) << 24 | u32::from(green) << 16 | u32::from(blue) << 8 | u32::from(alpha);
        RGBA8(v)
    }

    pub fn red(self) -> u8 {
        (self.0 >> 24) as u8
    }

    pub fn set_red(mut self, red: u8) -> Self {
        let red = u32::from(red) << 24;
        self.0 &= red | 0x00_FF_FF_FF;
        self
    }

    pub fn green(self) -> u8 {
        (self.0 >> 16) as u8
    }

    pub fn set_green(mut self, green: u8) -> Self {
        let green = u32::from(green) << 16;
        self.0 &= green | 0xFF_00_FF_FF;
        self
    }

    pub fn blue(self) -> u8 {
        (self.0 >> 8) as u8
    }

    pub fn set_blue(mut self, blue: u8) -> Self {
        let blue = u32::from(blue) << 8;
        self.0 &= blue | 0xFF_FF_00_FF;
        self
    }

    pub fn alpha(self) -> u8 {
        self.0 as u8
    }

    pub fn set_alpha(mut self, alpha: u8) -> Self {
        let blue = u32::from(alpha);
        self.0 &= blue | 0xFF_FF_FF_00;
        self
    }
}

pub mod constants {
    use super::RGBA8;

    pub const RED: RGBA8 = RGBA8(0xFF_00_00_FF);
    pub const GREEN: RGBA8 = RGBA8(0x00_FF_00_FF);
    pub const BLUE: RGBA8 = RGBA8(0x00_00_FF_FF);
    pub const BLACK: RGBA8 = RGBA8(0x00_00_00_FF);
    pub const WHITE: RGBA8 = RGBA8(0xFF_FF_FF_FF);
    pub const TRANSPARENT: RGBA8 = RGBA8(0x00_00_00_00);
}