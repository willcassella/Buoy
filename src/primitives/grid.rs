use crate::prelude::*;

#[derive(Clone)]
pub struct Grid {
    pub rows: Vec<GridLine>,
    pub cols: Vec<GridLine>,
    pub areas: Vec<GridArea>,
}

#[derive(Copy, Clone)]
pub enum GridLine {
    /*TOOD: Auto*/
    Abs(f32),
    Rem(u32),
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct GridArea {
    start_row: u32,
    end_row: u32,
    start_column: u32,
    end_column: u32,
    name: SocketName,
}

impl Element for Grid {
    fn run(
        &self,
        mut ctx: Context,
        id: Id,
    ) -> LayoutObj {
        unimplemented!()
    }
}