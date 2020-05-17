use std::f32;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Region {
    pub pos: Point,
    pub area: Area,
}

impl Region {
    pub fn new(pos: Point, area: Area) -> Self {
        Region { pos, area }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }

    pub fn zero() -> Self {
        Point { x: 0_f32, y: 0_f32 }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Area {
    pub width: f32,
    pub height: f32,
}

impl Area {
    pub fn new(width: f32, height: f32) -> Self {
        Area { width, height }
    }

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

    pub fn stretch(self, other: Self) -> Self {
        Area {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
}

impl Default for Area {
    fn default() -> Self {
        Area::zero()
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum HAlign {
    Center,
    Left,
    Right,
}

impl HAlign {
    pub fn align(self, area: Area, mut region: Region) -> Region {
        match self {
            HAlign::Left => {
                region.area.width = area.width;
            }
            HAlign::Right => {
                region.pos.x = region.pos.x + region.area.width - area.width;
                region.area.width = area.width;
            }
            HAlign::Center => {
                region.pos.x = (region.pos.x + region.area.width / 2_f32) - area.width / 2_f32;
                region.area.width = area.width;
            }
        }

        region
    }
}

impl Default for HAlign {
    fn default() -> Self {
        HAlign::Left
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum VAlign {
    Center,
    Top,
    Bottom,
}

impl VAlign {
    pub fn align(self, area: Area, mut region: Region) -> Region {
        match self {
            VAlign::Top => {
                region.area.height = area.height;
            }
            VAlign::Bottom => {
                region.pos.y = region.pos.y + region.area.height - area.height;
                region.area.height = area.height;
            }
            VAlign::Center => {
                region.pos.y = (region.pos.y + region.area.height / 2_f32) - area.height / 2_f32;
                region.area.height = area.height;
            }
        }

        region
    }
}

impl Default for VAlign {
    fn default() -> Self {
        VAlign::Top
    }
}
