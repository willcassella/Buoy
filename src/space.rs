use std::f32;
use std::ops::{Add, Sub};

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

    pub fn contains(&self, mut point: Point) -> bool {
        if self.pos.x > point.x || self.pos.y > point.y {
            return false;
        }

        point = point - self.pos;
        if point.x > self.area.width || point.y > self.area.height {
            return false;
        }

        true
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

impl Add<Point> for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Self {
        Vector { x, y }
    }

    pub fn zero() -> Self {
        Vector { x: 0_f32, y: 0_f32 }
    }
}

impl Add<Vector> for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Vector> for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
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

    pub fn min(self, other: Self) -> Self {
        Area {
            width: self.width.min(other.width),
            height: self.height.min(other.height),
        }
    }

    pub fn max(self, other: Self) -> Self {
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

#[derive(Copy, Clone)]
enum Align {
    Start,
    StartOffsetAbs(f32),
    StartOffsetPct(f32),
    End,
    EndOffsetAbs(f32),
    EndOffsetPct(f32),
    Center,
}

fn align<T: Into<Align>>(align: T, area_dim: f32, region_dim: f32, start: f32) -> f32 {
    match align.into() {
        Align::Start => start,
        Align::StartOffsetAbs(x) => start + x, // TODO: Clamp to (region_dim - area_dim)?
        Align::StartOffsetPct(x) => start + (region_dim - area_dim) * x,
        Align::End => start + region_dim - area_dim,
        Align::EndOffsetAbs(x) => start + region_dim - area_dim - x,
        Align::EndOffsetPct(x) => start + (region_dim - area_dim) * (1_f32 - x),
        Align::Center => (start + region_dim / 2_f32) - area_dim / 2_f32,
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HAlign {
    Center,
    Left,
    Right,
    LeftOffsetAbs(f32),
    LeftOffsetPct(f32),
    RightOffsetAbs(f32),
    RightOffsetPct(f32),
}

impl HAlign {
    pub fn align_horizontally(self, area: Area, mut region: Region) -> Region {
        region.pos.x = align(self, area.width, region.area.width, region.pos.x);
        region.area.width = area.width;
        region
    }
}

impl Default for HAlign {
    fn default() -> Self {
        HAlign::Left
    }
}

impl Into<Align> for HAlign {
    fn into(self) -> Align {
        match self {
            HAlign::Left => Align::Start,
            HAlign::LeftOffsetAbs(x) => Align::StartOffsetAbs(x),
            HAlign::LeftOffsetPct(x) => Align::StartOffsetPct(x),
            HAlign::Right => Align::End,
            HAlign::RightOffsetAbs(x) => Align::EndOffsetAbs(x),
            HAlign::RightOffsetPct(x) => Align::EndOffsetPct(x),
            HAlign::Center => Align::Center,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VAlign {
    Top,
    TopOffsetAbs(f32),
    TopOffsetPct(f32),
    Bottom,
    BottomOffsetAbs(f32),
    BottomOffsetPct(f32),
    Center,
}

impl VAlign {
    pub fn align_vertically(self, area: Area, mut region: Region) -> Region {
        region.pos.y = align(self, area.height, region.area.height, region.pos.y);
        region.area.height = area.height;
        region
    }
}

impl Default for VAlign {
    fn default() -> Self {
        VAlign::Top
    }
}

impl Into<Align> for VAlign {
    fn into(self) -> Align {
        match self {
            VAlign::Top => Align::Start,
            VAlign::TopOffsetAbs(x) => Align::StartOffsetAbs(x),
            VAlign::TopOffsetPct(x) => Align::StartOffsetPct(x),
            VAlign::Bottom => Align::End,
            VAlign::BottomOffsetAbs(x) => Align::EndOffsetAbs(x),
            VAlign::BottomOffsetPct(x) => Align::EndOffsetPct(x),
            VAlign::Center => Align::Center,
        }
    }
}
