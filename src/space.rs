use std::f32;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

pub trait Point:
    Sized
    + Copy
    + Clone
    + Default
    + PartialEq
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
{
    fn zero() -> Self;
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Point1D {
    pub x: f32,
}

impl Point1D {
    pub fn new(x: f32) -> Self {
        Point1D { x }
    }
}

impl Point for Point1D {
    fn zero() -> Self {
        Point1D { x: 0_f32 }
    }
}

impl Add<Point1D> for Point1D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Point1D { x: self.x + rhs.x }
    }
}

impl Add<Size1D> for Point1D {
    type Output = Self;

    fn add(self, rhs: Size1D) -> Self {
        Point1D {
            x: self.x + rhs.length,
        }
    }
}

impl AddAssign<Point1D> for Point1D {
    fn add_assign(&mut self, rhs: Point1D) {
        self.x += rhs.x;
    }
}

impl AddAssign<Size1D> for Point1D {
    fn add_assign(&mut self, rhs: Size1D) {
        self.x += rhs.length;
    }
}

impl Sub<Point1D> for Point1D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Point1D { x: self.x - rhs.x }
    }
}

impl Sub<Size1D> for Point1D {
    type Output = Self;

    fn sub(self, rhs: Size1D) -> Self {
        Point1D {
            x: self.x - rhs.length,
        }
    }
}

impl SubAssign<Point1D> for Point1D {
    fn sub_assign(&mut self, rhs: Point1D) {
        self.x -= rhs.x;
    }
}

impl SubAssign<Size1D> for Point1D {
    fn sub_assign(&mut self, rhs: Size1D) {
        self.x -= rhs.length;
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

impl Point2D {
    pub fn new(x: f32, y: f32) -> Self {
        Point2D { x, y }
    }
}

impl Point for Point2D {
    fn zero() -> Self {
        Point2D { x: 0_f32, y: 0_f32 }
    }
}

impl Add<Point2D> for Point2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Point2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Size2D> for Point2D {
    type Output = Self;

    fn add(self, rhs: Size2D) -> Self {
        Point2D {
            x: self.x + rhs.width,
            y: self.y + rhs.height,
        }
    }
}

impl AddAssign<Point2D> for Point2D {
    fn add_assign(&mut self, rhs: Point2D) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl AddAssign<Size2D> for Point2D {
    fn add_assign(&mut self, rhs: Size2D) {
        self.x += rhs.width;
        self.y += rhs.height;
    }
}

impl Sub<Point2D> for Point2D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Point2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<Size2D> for Point2D {
    type Output = Self;

    fn sub(self, rhs: Size2D) -> Self {
        Point2D {
            x: self.x - rhs.width,
            y: self.y - rhs.height,
        }
    }
}

impl SubAssign<Point2D> for Point2D {
    fn sub_assign(&mut self, rhs: Point2D) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl SubAssign<Size2D> for Point2D {
    fn sub_assign(&mut self, rhs: Size2D) {
        self.x -= rhs.width;
        self.y -= rhs.height;
    }
}

pub trait Size:
    Sized
    + Copy
    + Clone
    + Default
    + PartialEq
    + Add
    + AddAssign
    + Sub
    + SubAssign
    + Mul<f32>
    + MulAssign<f32>
    + Div<f32>
    + DivAssign<f32>
{
    fn zero() -> Self;

    fn infinite() -> Self;

    fn min(self, other: Self) -> Self;

    fn max(self, other: Self) -> Self;
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Size1D {
    pub length: f32,
}

impl Size1D {
    pub fn new(length: f32) -> Self {
        Size1D { length }
    }
}

impl Size for Size1D {
    fn zero() -> Self {
        Size1D { length: 0_f32 }
    }

    fn infinite() -> Self {
        Size1D {
            length: f32::INFINITY,
        }
    }

    fn min(self, other: Self) -> Self {
        Size1D {
            length: self.length.min(other.length),
        }
    }

    fn max(self, other: Self) -> Self {
        Size1D {
            length: self.length.max(other.length),
        }
    }
}

impl Add<Size1D> for Size1D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Size1D {
            length: self.length + rhs.length,
        }
    }
}

impl Add<Point1D> for Size1D {
    type Output = Point1D;

    fn add(self, rhs: Point1D) -> Point1D {
        Point1D {
            x: self.length + rhs.x,
        }
    }
}

impl AddAssign<Size1D> for Size1D {
    fn add_assign(&mut self, rhs: Size1D) {
        self.length += rhs.length;
    }
}

impl Sub<Size1D> for Size1D {
    type Output = Self;

    fn sub(self, rhs: Size1D) -> Self {
        Size1D {
            length: self.length - rhs.length,
        }
    }
}

impl SubAssign<Size1D> for Size1D {
    fn sub_assign(&mut self, rhs: Size1D) {
        self.length -= rhs.length;
    }
}

impl Mul<f32> for Size1D {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Size1D {
            length: self.length * rhs,
        }
    }
}

impl MulAssign<f32> for Size1D {
    fn mul_assign(&mut self, rhs: f32) {
        self.length *= rhs;
    }
}

impl Div<f32> for Size1D {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Size1D {
            length: self.length / rhs,
        }
    }
}

impl DivAssign<f32> for Size1D {
    fn div_assign(&mut self, rhs: f32) {
        self.length /= rhs;
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Size2D {
    pub width: f32,
    pub height: f32,
}

impl Size2D {
    pub fn new(width: f32, height: f32) -> Self {
        Size2D { width, height }
    }
}

impl Size for Size2D {
    fn zero() -> Self {
        Size2D {
            width: 0_f32,
            height: 0_f32,
        }
    }

    fn infinite() -> Self {
        Size2D {
            width: f32::INFINITY,
            height: f32::INFINITY,
        }
    }

    fn min(self, other: Self) -> Self {
        Size2D {
            width: self.width.min(other.width),
            height: self.height.min(other.height),
        }
    }

    fn max(self, other: Self) -> Self {
        Size2D {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
}

impl Add<Size2D> for Size2D {
    type Output = Self;

    fn add(self, rhs: Size2D) -> Self {
        Size2D {
            width: self.width + rhs.width,
            height: self.height + rhs.height,
        }
    }
}

impl Add<Point2D> for Size2D {
    type Output = Point2D;

    fn add(self, rhs: Point2D) -> Point2D {
        Point2D {
            x: self.width + rhs.x,
            y: self.height + rhs.y,
        }
    }
}

impl AddAssign<Size2D> for Size2D {
    fn add_assign(&mut self, rhs: Size2D) {
        self.width += rhs.width;
        self.height += rhs.height;
    }
}

impl Sub<Size2D> for Size2D {
    type Output = Self;

    fn sub(self, rhs: Size2D) -> Self {
        Size2D {
            width: self.width - rhs.width,
            height: self.height - rhs.height,
        }
    }
}

impl SubAssign<Size2D> for Size2D {
    fn sub_assign(&mut self, rhs: Size2D) {
        self.width -= rhs.width;
        self.height -= rhs.height;
    }
}

impl Mul<f32> for Size2D {
    type Output = Size2D;

    fn mul(self, rhs: f32) -> Self {
        Size2D {
            width: self.width * rhs,
            height: self.height * rhs,
        }
    }
}

impl MulAssign<f32> for Size2D {
    fn mul_assign(&mut self, rhs: f32) {
        self.width *= rhs;
        self.height *= rhs;
    }
}

impl Div<f32> for Size2D {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Size2D {
            width: self.width / rhs,
            height: self.height / rhs,
        }
    }
}

impl DivAssign<f32> for Size2D {
    fn div_assign(&mut self, rhs: f32) {
        self.width /= rhs;
        self.height /= rhs;
    }
}

#[repr(C)]
pub struct Region<S: Space> {
    pub pos: S::Point,
    pub size: S::Size,
}

impl<S: Space> Region<S> {
    pub fn new(pos: S::Point, size: S::Size) -> Self {
        Region { pos, size }
    }

    // pub fn contains(&self, mut point: S::Point) -> bool {
    //     if self.pos.x > point.x || self.pos.y > point.y {
    //         return false;
    //     }

    //     point = point - self.pos;
    //     if point.x > self.size.width || point.y > self.size.height {
    //         return false;
    //     }

    //     true
    // }
}

impl<S: Space> Copy for Region<S> {}

impl<S: Space> Clone for Region<S> {
    fn clone(&self) -> Self {
        Region {
            pos: self.pos.clone(),
            size: self.size.clone(),
        }
    }
}

pub type Region1D = Region<Space1D>;
pub type Region2D = Region<Space2D>;

pub trait Space {
    type Size: Size + Add<Self::Point, Output = Self::Point>;
    type Point: Point
        + Add<Self::Size, Output = Self::Point>
        + AddAssign<Self::Size>
        + Sub<Self::Size, Output = Self::Point>
        + SubAssign<Self::Size>;
}

pub struct Space1D;
impl Space for Space1D {
    type Size = Size1D;
    type Point = Point1D;
}

pub struct Space2D;
impl Space for Space2D {
    type Size = Size2D;
    type Point = Point2D;
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

fn align<T: Into<Align>>(align: T, size_dim: f32, region_dim: f32, start: f32) -> f32 {
    match align.into() {
        Align::Start => start,
        Align::StartOffsetAbs(x) => start + x, // TODO: Clamp to (region_dim - size_dim)?
        Align::StartOffsetPct(x) => start + (region_dim - size_dim) * x,
        Align::End => start + region_dim - size_dim,
        Align::EndOffsetAbs(x) => start + region_dim - size_dim - x,
        Align::EndOffsetPct(x) => start + (region_dim - size_dim) * (1_f32 - x),
        Align::Center => (start + region_dim / 2_f32) - size_dim / 2_f32,
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
    pub fn align_horizontally(self, size: Size2D, mut region: Region2D) -> Region2D {
        region.pos.x = align(self, size.width, region.size.width, region.pos.x);
        region.size.width = size.width;
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
    pub fn align_vertically(self, size: Size2D, mut region: Region2D) -> Region2D {
        region.pos.y = align(self, size.height, region.size.height, region.pos.y);
        region.size.height = size.height;
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
