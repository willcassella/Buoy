
#[derive(Clone)]
pub struct Grid {
    pub rows: Vec<GridLine>,
    pub cols: Vec<GridLine>,
}

#[derive(Copy, Clone)]
pub enum GridLine {
    Auto,
    Abs(f32),
    Rem(u32),
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct GridChild {
    start_row: u32,
    end_row: u32,
    start_column: u32,
    end_column: u32,
}
