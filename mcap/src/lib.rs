
// todo, remove
pub mod scrap;

// real
pub use glam::{Vec2, Vec3A};
pub use Vec3A as Vec3;
use core::f32::{INFINITY,NEG_INFINITY};

// quake uses ~0.2%
pub const SKIN_FACTOR: f32 = 0.002;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub verts: [Vec3; 3],
    pub normal: Vec3,
    pub origin_offset: f32,
    pub min_y: f32,
    pub max_y: f32,
}

const FLOOR_EPS: f32 = 0.01;

impl Triangle {
    pub fn verts(&self) -> [Vec3; 3] {
        self.verts
    }
    pub fn normal(&self) -> Vec3 {
        self.normal
    }
    pub fn offset(&self) -> f32 {
        self.origin_offset
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Surface {
    Wall(Triangle),
    Floor(Triangle),
    Slide(Triangle),
    Cieling(Triangle),
}

impl Surface {
    pub fn new(verts: [Vec3; 3], normal: Vec3) -> Self {
        let t = Triangle {
            verts,
            normal,
            origin_offset: -normal.dot(verts[0]),
            min_y: verts[0].y.min(verts[1].y.min(verts[2].y)),
            max_y: verts[0].y.max(verts[1].y.max(verts[2].y)),
        };

        let slide_deg = 60f32.to_radians().cos();

        match normal.y {
            y if y.abs() < FLOOR_EPS => Surface::Wall(t),
            y if y > slide_deg => Surface::Floor(t),
            y if y > FLOOR_EPS => Surface::Slide(t),
            _ => Surface::Cieling(t),
        }
    }
}

pub fn closest_point_on_segment_v3(p: Vec3, a: Vec3, b: Vec3) -> Vec3 {
    let ab = b - a;
    let ap = p - a;

    let proj = ap.dot(ab);
    let ab_len_sq = ab.length_squared();
    let d = proj / ab_len_sq;

    match d {
        NEG_INFINITY..=0f32 => a,
        1f32..=INFINITY => b,
        d => a + ab * d,
    }
}

pub fn push_out_walls_2(
    pos: Vec3,
    check_height: f32,
    radius: f32,
    surfaces: &[&Surface],
) -> (Vec3, bool) {
    let radius = radius - SKIN_FACTOR * 2.;
    let sph_y = pos.y + check_height;
    let mut out_x = pos.x;
    let mut out_z = pos.z;
    let mut hit = false;

    for s in surfaces {
        // all downward facing normals
        let tri = match s {
            Surface::Wall(t) => t,
            Surface::Floor(t) => t,
            Surface::Slide(t) => t,
            _ => continue,
        };

        let n = tri.normal;
        let xz_len = (n.x * n.x + n.z * n.z).sqrt();

        let cur = Vec3::new(out_x, sph_y, out_z);

        // skip if on the back side of the triangle
        let signed = n.dot(cur) + tri.origin_offset;
        if signed < 0.0 {
            continue;
        }

        let nearest = closest_point_triangle(cur, &tri.verts);

        let dist = nearest.distance(cur);

        // // old radius check for both sides of wall
        // if dist.abs() >= radius {
        if dist >= radius {
            continue;
        }

        // get appropriate deflection dir
        let diff = cur - nearest;
        let diff_xz = Vec2::new(diff.x, diff.z);
        let diff_xz_len = diff_xz.length();

        let (push_dir_x, push_dir_z) = if diff_xz_len > f32::EPSILON {
            (diff_xz.x / diff_xz_len, diff_xz.y / diff_xz_len)
        } else {
            // diff is ~ 0
            // center is directly above/below the nearest point
            // fall back to face normal
            //
            // we should be able to ensure this doesn't happen
            // by padding downward raycasts a little bit
            (n.x / xz_len, n.z / xz_len)
        };

        let push = radius - dist;
        out_x += push_dir_x * push;
        out_z += push_dir_z * push;
        hit = true;
    }

    (Vec3::new(out_x, pos.y, out_z), hit)
}

/// Renderkit/Embree
/// Copyright 2009-2021 Intel Corporation
/// SPDX-License-Identifier: Apache-2.0
pub fn closest_point_triangle(p: Vec3, tri: &[Vec3; 3]) -> Vec3 {
    let a = &tri[0];
    let b = &tri[1];
    let c = &tri[2];
    let ab = b - a;
    let ac = c - a;
    let ap = p - a;

    let d1 = ab.dot(ap);
    let d2 = ac.dot(ap);
    if d1 <= 0. && d2 <= 0. {
        return *a;
    } //#1

    let bp = p - b;
    let d3 = ab.dot(bp);
    let d4 = ac.dot(bp);
    if d3 >= 0. && d4 <= d3 {
        return *b;
    } //#2

    let cp = p - c;
    let d5 = ab.dot(cp);
    let d6 = ac.dot(cp);
    if d6 >= 0. && d5 <= d6 {
        return *c;
    } //#3

    let vc = d1 * d4 - d3 * d2;
    if vc <= 0. && d1 >= 0. && d3 <= 0. {
        let v = d1 / (d1 - d3);
        return a + v * ab; //#4
    }

    let vb = d5 * d2 - d1 * d6;
    if vb <= 0. && d2 >= 0. && d6 <= 0. {
        let v = d2 / (d2 - d6);
        return a + v * ac; //#5
    }

    let va = d3 * d6 - d5 * d4;
    if va <= 0. && (d4 - d3) >= 0. && (d5 - d6) >= 0. {
        let v = (d4 - d3) / ((d4 - d3) + (d5 - d6));
        return b + v * (c - b); //#6
    }

    let denom = 1. / (va + vb + vc);
    let v = vb * denom;
    let w = vc * denom;
    a + v * ab + w * ac //#0
}

// pos here is the feet/bottom
pub fn find_ciel_height_hotdog_v3(
    pos: Vec3,
    offset: f32,      // offset from the bottom
    range_above: f32, // range above offset which procs
    surfaces: &[&Surface],
    radius: f32,
) -> Option<(Surface, f32)> {
    let mut best_y = f32::INFINITY;
    let mut best_surf = None;

    let posv2 = Vec2::new(pos.x, pos.z);

    for s in surfaces {
        // all downward facing normals
        let tri = match s {
            Surface::Cieling(t) => t,
            _ => continue,
        };

        let p1v2 = Vec2::new(tri.verts[0].x, tri.verts[0].z);
        let p2v2 = Vec2::new(tri.verts[1].x, tri.verts[1].z);
        let p3v2 = Vec2::new(tri.verts[2].x, tri.verts[2].z);

        // check if point is inside radius-expanded edges,
        //
        // but also fall back to using point in triangle with slides
        let plane_pos: Vec2 = if flattened_point_inside_flattened_triangle(
            pos,
            tri.verts[0],
            tri.verts[1],
            tri.verts[2],
        ) {
            posv2
        } else {
            let cp1 = closest_point_on_segment_v2(posv2, p1v2, p2v2);
            let cp2 = closest_point_on_segment_v2(posv2, p2v2, p3v2);
            let cp3 = closest_point_on_segment_v2(posv2, p3v2, p1v2);
            let cp1d = cp1.distance(posv2);
            let cp2d = cp2.distance(posv2);
            let cp3d = cp3.distance(posv2);
            let threshold = radius;
            if cp1d <= threshold || cp2d <= threshold || cp3d <= threshold {
                if cp1d <= cp2d && cp1d <= cp3d {
                    cp1
                } else if cp2d <= cp3d {
                    cp2
                } else {
                    cp3
                }
            } else {
                // neither inside flattened triangle, nor within threshold
                continue;
            }
        };

        // get y at plane_pos.xz
        let y = solve_plane_y(tri.normal, tri.origin_offset, plane_pos.x, plane_pos.y);

        // within cieling detection range
        // add a SKIN_FACTOR just for fun
        let bottom = pos.y + offset - SKIN_FACTOR;
        let top = pos.y + offset + range_above + SKIN_FACTOR;
        // store according to which plane_pos is nearest to pos_v2
        if (bottom..=top).contains(&y) && y < best_y {
            best_y = y;
            best_surf = Some(*s);
        }
    }

    match best_surf {
        Some(surf) => Some((*surf, best_y)),
        None => None,
    }
}

// raylib's function for - Check if point is inside a triangle defined by three points (p1, p2, p3)
pub fn flattened_point_inside_flattened_triangle(
    point: Vec3,
    p1: Vec3,
    p2: Vec3,
    p3: Vec3,
) -> bool {
    let alpha = ((p2.z - p3.z) * (point.x - p3.x) + (p3.x - p2.x) * (point.z - p3.z))
        / ((p2.z - p3.z) * (p1.x - p3.x) + (p3.x - p2.x) * (p1.z - p3.z));

    let beta = ((p3.z - p1.z) * (point.x - p3.x) + (p1.x - p3.x) * (point.z - p3.z))
        / ((p2.z - p3.z) * (p1.x - p3.x) + (p3.x - p2.x) * (p1.z - p3.z));

    let gamma = 1. - alpha - beta;

    let collision = (alpha > 0.) && (beta > 0.) && (gamma > 0.);

    return collision;
}

pub fn closest_point_on_segment_v2(p: Vec2, a: Vec2, b: Vec2) -> Vec2 {
    let ab = b - a;
    let ap = p - a;

    let proj = ap.dot(ab);
    let ab_len_sq = ab.length_squared();
    let d = proj / ab_len_sq;

    match d {
        NEG_INFINITY..=0f32 => a,
        1f32..=INFINITY => b,
        d => a + ab * d,
    }
}

pub fn solve_plane_y(normal: Vec3, origin_offset: f32, x: f32, z: f32) -> f32 {
    //   Ax + By + Cz + D   = 0
    //   Ax + Cz + D        = -By
    //  (Ax + Cz + D) / B   = -y
    // -(Ax + Cz + D) / B   = y
    -(normal.x * x + normal.z * z + origin_offset) / normal.y
}

// pos here is the feet/bottom
pub fn find_floor_height_hotdog_v4(
    pos: Vec3,
    snap_up: f32,
    snap_down: f32,
    surfaces: &[&Surface],
    radius: f32,
) -> Option<(Surface, f32)> {
    let mut best_y = f32::NEG_INFINITY;
    let mut best_surf = None;

    let posv2 = Vec2::new(pos.x, pos.z);

    // add a SKIN_FACTOR just for fun (my step-{up,down} is coincidentally a multiple of my minimum alignment in map)
    let bottom = pos.y - snap_down - SKIN_FACTOR;
    let top = pos.y + snap_up + SKIN_FACTOR;

    // check if we're teetering on an edge with more than floor snap distance
    'sloop: for s in surfaces {
        // all upward facing normals
        let y = match s {
            Surface::Slide(tri) => {
                // only raycast down
                if !flattened_point_inside_flattened_triangle(
                    pos,
                    tri.verts[0],
                    tri.verts[1],
                    tri.verts[2],
                ) {
                    continue;
                }
                solve_plane_y(tri.normal, tri.origin_offset, posv2.x, posv2.y)
            },
            Surface::Floor(tri) => 'ray_c: {

                // v2's
                let p1v2 = Vec2::new(tri.verts[0].x, tri.verts[0].z);
                let p2v2 = Vec2::new(tri.verts[1].x, tri.verts[1].z);
                let p3v2 = Vec2::new(tri.verts[2].x, tri.verts[2].z);

                // // OLD METHOD 0
                // let y = solve_plane_y(tri.normal, tri.origin_offset, posv2.x, posv2.y);
                // // OLD METHOD 1
                // let y = solve_plane_y(tri.normal, tri.origin_offset, posv2.x, posv2.y).clamp(tri.min_y, tri.max_y);
                // // OLD METHOD 2
                // let y = solve_plane_y(tri.normal, tri.origin_offset, highest.x, highest.y).clamp(tri.min_y, tri.max_y);
                // where highest is just posv2 + uphill * (radius / uphill_len); unchecked
                // a floor slanted up and away still suffered from this issue if you were
                // traveling up it's slope, and beyond an edge, but still inside min_y and max_y
                //
                // // FINAL METHOD
                // get a point uphill by radius / uphill_len
                // but make sure it's inside the triangle or clipped to
                // a nearest point on 2d segment.
                //
                // this is all to stop a horrible bonk at the top of a floor tri
                // we don't want to snap up it, and we also don't want to scale up
                // and invisible slope beyond it.
                let uphill = Vec2::new(-tri.normal.x, -tri.normal.z);
                let uphill_len = uphill.length();
                // why is this necessary again?? -- radius / 0 == NaN
                let highest = if uphill_len > f32::EPSILON {
                    posv2 + uphill * (radius / uphill_len)
                } else {
                    posv2
                };
                if flattened_point_inside_flattened_triangle(
                        Vec3::new(highest.x, 0., highest.y),
                        tri.verts[0],
                        tri.verts[1],
                        tri.verts[2],
                ){
                    // raycast hits inside tri
                    let y = solve_plane_y(tri.normal, tri.origin_offset, highest.x, highest.y);
                    break 'ray_c y;
                } else {
                    // get closest point to target position
                    let np1 = closest_point_on_segment_v2(highest, p1v2, p2v2);
                    let np2 = closest_point_on_segment_v2(highest, p2v2, p3v2);
                    let np3 = closest_point_on_segment_v2(highest, p3v2, p1v2);

                    let d1 = np1.distance(highest);
                    let d2 = np2.distance(highest);
                    let d3 = np3.distance(highest);

                    let highest = if d1 <= d2 && d1 <= d3 {
                        np1
                    } else if d2 <= d3 {
                        np2
                    } else {
                        np3
                    };

                    // see if it's inside radius + 2SKIN
                    let threshold = radius + radius * SKIN_FACTOR * 2.;
                    if highest.distance(posv2) > threshold {
                        // neither inside flattened triangle, nor within threshold
                        continue 'sloop;
                    }

                    solve_plane_y(tri.normal, tri.origin_offset, highest.x, highest.y)
                }
            }, 
            _ => continue,
        };

        // within floor snap range both below and above pos (bottom)
        // store according to which is highest
        if (bottom..=top).contains(&y) && y > best_y {
            best_y = y;
            best_surf = Some(*s);
        }
    }

    match best_surf {
        Some(surf) => Some((*surf, best_y)),
        None => None,
    }
}

use std::ptr;
use rustc_hash::FxHashMap;

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