use std::cmp::{min, max};
use std::ops::RangeInclusive;

use crate::prelude::*;

#[derive(Clone)]
pub struct Grid {
    pub cols: Vec<GridTrack>,
    pub rows: Vec<GridTrack>,
    pub regions: Vec<GridRegion>,
}

#[derive(Copy, Clone)]
pub enum GridTrack {
    Auto,
    Fr(u32),
}

impl GridTrack {
    pub fn is_auto(self) -> bool {
        match self {
            GridTrack::Auto => true,
            _ => false,
        }
    }

    pub fn get_frs(self) -> Option<u32> {
        match self {
            GridTrack::Auto => None,
            GridTrack::Fr(x) => Some(x),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
enum TrackIndex {
    FromStart(u32),
    FromEnd(u32),
    // Overflow(+/-)
}

impl TrackIndex {
    pub fn ordinal(idx: i32) -> Option<Self> {
        if idx > 0 {
            Some(TrackIndex::FromStart((idx - 1) as u32))
        } else if idx < 0 {
            Some(TrackIndex::FromEnd((-idx - 1) as u32))
        } else {
            None
        }
    }

    fn to_array_index(self, total_tracks: usize) -> usize {
        match self {
            TrackIndex::FromStart(idx) => min(idx as usize, total_tracks - 1),
            TrackIndex::FromEnd(idx) => total_tracks - min(idx as usize + 1, total_tracks),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ColIndex(TrackIndex);

impl ColIndex {
    pub fn ordinal(idx: i32) -> Option<Self> {
        TrackIndex::ordinal(idx).map(|x| ColIndex(x))
    }

    pub fn from_left(idx: u32) -> Self {
        ColIndex(TrackIndex::FromStart(idx))
    }

    pub fn from_right(idx: u32) -> Self {
        ColIndex(TrackIndex::FromEnd(idx))
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RowIndex(TrackIndex);

impl RowIndex {
    pub fn ordinal(idx: i32) -> Option<Self> {
        TrackIndex::ordinal(idx).map(|x| RowIndex(x))
    }

    pub fn from_top(idx: u32) -> Self {
        RowIndex(TrackIndex::FromStart(idx))
    }

    pub fn from_bot(idx: u32) -> Self {
        RowIndex(TrackIndex::FromEnd(idx))
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct GridRegion {
    pub name: SocketName,
    pub start_col: ColIndex,
    pub end_col: ColIndex,
    pub start_row: RowIndex,
    pub end_row: RowIndex,
}

impl GridRegion {
    pub fn cell(name: SocketName, col: ColIndex, row: RowIndex) -> Self {
        GridRegion {
            name,
            start_col: col,
            end_col: col,
            start_row: row,
            end_row: row,
        }
    }

    pub fn col_span(name: SocketName, start_col: ColIndex, end_col: ColIndex, row: RowIndex) -> Self {
        GridRegion {
            name,
            start_col,
            end_col,
            start_row: row,
            end_row: row,
        }
    }

    pub fn row_span(name: SocketName, col: ColIndex, start_row: RowIndex, end_row: RowIndex) -> Self {
        GridRegion {
            name,
            start_col: col,
            end_col: col,
            start_row,
            end_row,
        }
    }

    pub fn span(name: SocketName, start_col: ColIndex, end_col: ColIndex, start_row: RowIndex, end_row: RowIndex) -> Self {
        GridRegion {
            name,
            start_col,
            end_col,
            start_row,
            end_row,
        }
    }

    fn get_col_range(&self, total_cols: usize) -> RangeInclusive<usize> {
        let start = self.start_col.0.to_array_index(total_cols);
        let end = self.end_col.0.to_array_index(total_cols);
        min(start, end)..=max(start, end)
    }

    fn get_row_range(&self, total_rows: usize) -> RangeInclusive<usize> {
        let start = self.start_row.0.to_array_index(total_rows);
        let end = self.end_row.0.to_array_index(total_rows);
        min(start, end)..=max(start, end)
    }
}

#[derive(Clone, Copy)]
struct TrackLayout {
    // Specification for this track
    track: GridTrack,

    // The calculated size of this track. If this track is 'auto-sized', this is determined during the Element pass.
    // For fractional tracks, this is determined during the Layout pass.
    // If there are no fractional tracks, then additional space is allocated equally to auto tracks.
    size: f32,

    // The absolute starting point of this track. Computed during the layout pass.
    start: f32,
}

impl TrackLayout {
    pub fn end(self) -> f32 {
        self.start + self.size
    }
}

struct ExamineGridTracksResult {
    frs: u32,
    layouts: Vec<TrackLayout>,
}

// Counts up the number of fractional tracks, and fills a vec of CalculatedTrack for each track
fn examine_grid_tracks(tracks: &Vec<GridTrack>) -> ExamineGridTracksResult {
    assert_ne!(tracks.len(), 0, "0-track grids are not currently supported");

    let mut frs = 0;
    let mut layouts = Vec::with_capacity(tracks.len());
    for &track in tracks {
        layouts.push(TrackLayout {
            track: track,
            size: 0_f32,
            start: 0_f32,
        });

        match track {
            GridTrack::Auto => (),
            GridTrack::Fr(x) => frs += x,
        }
    }

    ExamineGridTracksResult {
        frs,
        layouts,
    }
}

#[derive(Clone)]
struct ExamineRegionTracksResult {
    auto_tracks: u32,
    frs: u32,
    range: RangeInclusive<usize>,
}

fn examine_region_tracks(tracks: &Vec<GridTrack>, range: RangeInclusive<usize>) -> ExamineRegionTracksResult {
    let mut auto_tracks = 0;
    let mut frs = 0;
    for track in tracks[range.clone()].iter() {
        match track {
            GridTrack::Auto => auto_tracks += 1,
            GridTrack::Fr(x) => frs += x,
        }
    }

    ExamineRegionTracksResult {
        auto_tracks,
        frs,
        range: range,
    }
}

fn expand_region_tracks(min_size: f32, region_tracks: &ExamineRegionTracksResult, track_layouts: &mut Vec<TrackLayout>, fr_min_size: &mut f32) {
    if region_tracks.auto_tracks > 0 {
        for track_layout in track_layouts[region_tracks.range.clone()].iter_mut().filter(|layout| layout.track.is_auto()) {
            track_layout.size = track_layout.size.max(min_size / region_tracks.auto_tracks as f32);
        }
    } else {
        *fr_min_size = fr_min_size.max(min_size / region_tracks.frs as f32);
    }
}

struct RegionLayout<'frm> {
    layout: LayoutNode<'frm>,
    pub start_col: u32,
    pub end_col: u32,
    pub start_row: u32,
    pub end_row: u32,
}

impl Element for Grid {
    fn run<'ctx, 'frm>(self, mut ctx: Context<'ctx, 'frm>, _id: Id) -> LayoutNode<'frm> {
        // Count up the number of fractions in rows and columns
        let ExamineGridTracksResult{ frs: col_frs, layouts: mut col_layouts } = examine_grid_tracks(&self.cols);
        let ExamineGridTracksResult{ frs: row_frs, layouts: mut row_layouts } = examine_grid_tracks(&self.rows);

        let auto_max_area = ctx.max_area();
        let fr_max_area = Area {
            width: auto_max_area.width / col_frs as f32,
            height: auto_max_area.height / row_frs as f32,
        };

        let mut fr_min_area = Area::zero();
        let mut region_layouts = Vec::with_capacity(self.regions.len());
        for region in self.regions {
            // Figure out the max area for the region
            let region_cols = examine_region_tracks(&self.cols, region.get_col_range(self.cols.len()));
            let region_rows = examine_region_tracks(&self.rows, region.get_row_range(self.rows.len()));
            let max_region_area = Area {
                width: if region_cols.auto_tracks > 0 { auto_max_area.width } else { fr_max_area.width * region_cols.frs as f32 },
                height: if region_rows.auto_tracks > 0 { auto_max_area.height } else { fr_max_area.height * region_rows.frs as f32 },
            };

            // Calculate layout
            let mut layout = None;
            ctx.open_socket(region.name, max_region_area, &mut layout);

            // If layout failed for this region, then don't need to do anything
            let layout = match layout {
                Some(x) => x,
                None => continue,
            };

            // Expand each affected track
            expand_region_tracks(layout.min_area.width, &region_cols, &mut col_layouts, &mut fr_min_area.width);
            expand_region_tracks(layout.min_area.height, &region_rows, &mut row_layouts, &mut fr_min_area.height);

            region_layouts.push(RegionLayout {
                layout,
                start_col: *region_cols.range.start() as u32,
                end_col: *region_cols.range.end() as u32,
                start_row: *region_rows.range.start() as u32,
                end_row: *region_rows.range.end() as u32,
            });
        }

        // Calcuate minimum area required for the grid
        let grid_min_area = Area {
            // Not filtering fr tracks when folding because their size should be 0
            width: col_layouts.iter().fold(0_f32, |acc, col| acc + col.size) + fr_min_area.width * col_frs as f32,
            height: row_layouts.iter().fold(0_f32, |acc, row| acc + row.size) + fr_min_area.height * row_frs as f32,
        };

        ctx.new_layout(grid_min_area, GridLayout {
            min_area: grid_min_area,
            col_frs,
            row_frs,
            cols: col_layouts,
            rows: row_layouts,
            regions: region_layouts,
        })
    }
}

struct GridLayout<'frm> {
    min_area: Area,
    col_frs: u32,
    row_frs: u32,
    rows: Vec<TrackLayout>,
    cols: Vec<TrackLayout>,
    regions: Vec<RegionLayout<'frm>>,
}

fn layout_tracks(frs: u32, min_size: f32, given_size: f32, mut start: f32, tracks: &mut Vec<TrackLayout>) {
    let (fr_size_each, auto_additional_size_each) = if min_size < given_size {
        let additional_size = given_size - min_size;
        if frs > 0 {
            (additional_size / frs as f32, 0_f32)
        } else {
            (0_f32, additional_size / tracks.len() as f32)
        }
    } else {
        (0_f32, 0_f32)
    };

    for track in tracks {
        match track.track {
            GridTrack::Auto => track.size += auto_additional_size_each,
            GridTrack::Fr(frs) => track.size = fr_size_each * frs as f32,
        }

        track.start = start;
        start += track.size;
    }
}

impl<'frm> Layout for GridLayout<'frm> {
    fn render(mut self, grid_layout_region: Region, cmds: &mut render::CommandList) {
        // Compute final size and offsets for tracks
        println!("Running grid layout");
        layout_tracks(self.col_frs, self.min_area.width, grid_layout_region.area.width, grid_layout_region.pos.x, &mut self.cols);
        layout_tracks(self.row_frs, self.min_area.height, grid_layout_region.area.height, grid_layout_region.pos.y, &mut self.rows);

        // Lay out regions
        for grid_region in self.regions {
            // Figure out layout region for grid
            let pos = Point {
                x: self.cols[grid_region.start_col as usize].start,
                y: self.rows[grid_region.start_row as usize].start,
            };
            let area = Area {
                width: self.cols[grid_region.end_col as usize].end() - pos.x,
                height: self.rows[grid_region.end_row as usize].end() - pos.y,
            };

            // Render the regon
            grid_region.layout.render(Region { pos, area }, cmds);
        }
    }
}
