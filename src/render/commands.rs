use super::color::RGBA8;
use crate::message::Outbox;
use crate::space::Region;

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
    pub color: RGBA8,
}

impl ColoredQuad {
    pub fn new(quad: Quad, color: RGBA8) -> Self {
        ColoredQuad { quad, color }
    }
}

pub struct HoverQuad {
    pub quad: Quad,
    // TODO: This should be Outbox<Vec2> or something
    pub message: Outbox<()>,
}

pub struct ClickQuad {
    pub quad: Quad,
    // TODO: This should be Outbox<Vec2> or something
    pub message: Outbox<()>,
}
