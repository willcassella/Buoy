use crate::prelude::*;

#[derive(Clone)]
pub struct Grid {
    pub rows: Vec<GridLine>,
    pub cols: Vec<GridLine>,
    pub regions: Vec<GridRegion>,
}

#[derive(Copy, Clone)]
pub enum GridLine {
    /*TOOD: Auto*/
    Abs(f32),
    Rem(u32),
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct GridRegion {
    start_row: u32,
    end_row: u32,
    start_column: u32,
    end_column: u32,
    name: SocketName,
}

impl Element for Grid {
    fn run<'ctx, 'win>(&self, mut _ctx: Context<'ctx, 'win>, _id: Id) -> LayoutNode<'win> {
        unimplemented!()
    }
}
