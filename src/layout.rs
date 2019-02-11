use std::f32;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Region {
    pub pos: Point,
    pub area: Area,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn zero() -> Self {
        Point {
            x: 0_f32,
            y: 0_f32,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Area {
    pub width: f32,
    pub height: f32,
}

impl Area {
    pub fn zero() -> Self {
        Area {
            width: 0_f32,
            height: 0_f32,
        }
    }

    pub fn infinite() -> Self {
        Area {
            width: f32::INFINITY,
            height: f32::INFINITY,
        }
    }
}

impl Default for Area {
    fn default() -> Self {
        Area::zero()
    }
}
