use crate::{Socket, context::Context};

pub struct Block {
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum BlockChildVAlign {
    Inside,
    Above,
    Below,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum BlockChildHAlign {
    Inside,
    Left,
    Right,
}

pub struct BlockChild {
    v_align: BlockChildVAlign,
    h_align: BlockChildHAlign,
}