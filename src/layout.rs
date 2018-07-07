use std::f32;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Area {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Bounds {
    pub width: f32,
    pub height: f32,
}

impl Bounds {
    pub fn to_flex(&self) -> FlexBounds {
        FlexBounds {
            min_width: self.width,
            max_width: self.width,
            min_height: self.height,
            max_height: self.height,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct FlexBounds {
    pub min_width: f32,
    pub max_width: f32,
    pub min_height: f32,
    pub max_height: f32,
}

impl FlexBounds {
    pub fn fixed_width(width: f32) -> Self {
        FlexBounds {
            min_width: width,
            max_width: width,
            min_height: 0_f32,
            max_height: f32::INFINITY,
        }
    }

    pub fn fixed_height(height: f32) -> Self {
        FlexBounds {
            min_width: 0_f32,
            max_width: f32::INFINITY,
            min_height: height,
            max_height: height,
        }
    }

    pub fn is_fixed(&self) -> bool {
        self.min_width == self.max_width && self.min_height == self.max_height
    }

    pub fn add_width(&mut self, width: f32) {
        self.min_width += width;
        self.max_width += width;
    }

    pub fn add_height(&mut self, height: f32) {
        self.min_height += height;
        self.max_height += height;
    }

    pub fn subtract_width(&mut self, width: f32) {
        self.min_width -= width;
        self.max_width -= width;
    }

    pub fn subtract_height(&mut self, height: f32) {
        self.min_height -= height;
        self.max_height -= height;
    }
}

impl Default for FlexBounds {
    fn default() -> Self {
        FlexBounds {
            min_width: 0_f32,
            max_width: f32::INFINITY,
            min_height: 0_f32,
            max_height: f32::INFINITY,
        }
    }
}