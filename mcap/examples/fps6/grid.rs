use std::ptr;
use rustc_hash::FxHashMap;
use mcap::{Surface, Vec3};

pub const GRID_SIZE: f32 = 5.;

fn surface_verts(s: &Surface) -> &[Vec3; 3] {
    match s {
        Surface::Wall(t)
        | Surface::Floor(t)
        | Surface::Slide(t)
        | Surface::Cieling(t) => &t.verts,
    }
}


/*

     ## One Grid Cell
     +===========================+
     | Bob  | Nmap | Test | E1M1 |
     |---------------------------|
TRIS | 962  | 7234 | 2258 | 3564 |
     |===========================|
     |             10            |
     |===========================|
ALL  | 4280 | 3599 | 4679 | 3116 |
CUBE | 4690 | 4832 | 4869 | 3650 |
ONE  | 4800 | 5435 | 5835 | 3800 |
     |===========================|
     |           10+1            |
     |===========================|
ONE  | 4805 | 5257 | 5350 | 3650 |
     |===========================|
     |            5+1            |
     |===========================|
ONE  | 4813 | 5460 | 5784 | 3760 | <- do this
     |===========================|
     |              5            |
     |===========================|
CUBE | 4576 | 5266 | 5507 | 3730 |
ONE  | 4722 | 5440 | 5830 | 3787 |
     +===========================+

*/

pub struct SurfaceGrid {
    _surfaces: Box<[Surface]>,
    all_ptrs: Vec<*const Surface>,
    grid: FxHashMap<(u32, u32, u32), Vec<*const Surface>>,
}

impl SurfaceGrid {
    pub fn new(surfaces: Vec<Surface>) -> Self {
        let surfaces: Box<[Surface]> = surfaces.into_boxed_slice();
        let mut grid: FxHashMap<(u32, u32, u32), Vec<*const Surface>> = FxHashMap::default();

        for surf in surfaces.iter() {
            let verts = surface_verts(surf);

            let min_x = verts[0].x.min(verts[1].x).min(verts[2].x);
            let max_x = verts[0].x.max(verts[1].x).max(verts[2].x);
            let min_y = verts[0].y.min(verts[1].y).min(verts[2].y);
            let max_y = verts[0].y.max(verts[1].y).max(verts[2].y);
            let min_z = verts[0].z.min(verts[1].z).min(verts[2].z);
            let max_z = verts[0].z.max(verts[1].z).max(verts[2].z);

            // +/- 1
            // registers every triangle in all neighboring grids,
            // getting cube functionality in 1 lookup - no alloc
            let grid_min_x = (min_x / GRID_SIZE).floor() as i32 - 1;
            let grid_max_x = (max_x / GRID_SIZE).floor() as i32 + 1;
            let grid_min_y = (min_y / GRID_SIZE).floor() as i32 - 1 ;
            let grid_max_y = (max_y / GRID_SIZE).floor() as i32 + 1;
            let grid_min_z = (min_z / GRID_SIZE).floor() as i32 - 1;
            let grid_max_z = (max_z / GRID_SIZE).floor() as i32 + 1;

            let ptr: *const Surface = ptr::from_ref(surf);
            for x in grid_min_x..=grid_max_x {
                for y in grid_min_y..=grid_max_y {
                    for z in grid_min_z..=grid_max_z {
                        grid.entry((x as u32, y as u32, z as u32))
                            .or_default()
                            .push(ptr);
                    }
                }
            }
        }

        let all_ptrs: Vec<*const Surface> = surfaces.iter().map(ptr::from_ref).collect();
        Self { _surfaces: surfaces, all_ptrs, grid }
    }

    pub fn all_surfaces(&self) -> Option<&[&Surface]> {
        if self.all_ptrs.is_empty() {
            None
        } else {
            Some(unsafe {
                // black magic
                &*(self.all_ptrs.as_slice() as *const [*const Surface] as *const [&Surface])
            })
        }
    }

    pub fn surfaces_in_cell(&self, cell: (u32, u32, u32)) -> Option<&[&Surface]> {
        self.grid.get(&cell).map(|v| {
            // black magic
            unsafe { &*(v.as_slice() as *const [*const Surface] as *const [&Surface]) }
        })
    }

    pub fn surfaces_in_cell_and_adjacent(&self, cell: (u32, u32, u32)) -> Vec<&Surface> {
        let mut result = Vec::new();
        for dx in -1i32..=1 {
            let Some(x) = cell.0.checked_add_signed(dx) else { continue };
            for dy in -1i32..=1 {
                let Some(y) = cell.1.checked_add_signed(dy) else { continue };
                for dz in -1i32..=1 {
                    let Some(z) = cell.2.checked_add_signed(dz) else { continue };
                    if let Some(surfaces) = self.surfaces_in_cell((x, y, z)) {
                        result.extend_from_slice(surfaces);
                    }
                }
            }
        }
        // vec will have duplicates of any triangle that crosses the line
        // between 2 grid cells, here we dedupe
        result.sort_unstable_by_key(|s| *s as *const Surface);
        result.dedup_by_key(|s| *s as *const Surface);
        result
    }
}