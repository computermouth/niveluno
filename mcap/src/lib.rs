use core::f32;
use std::f32::{INFINITY, NEG_INFINITY};

pub use glam::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Wall {
    tri: [Vec3; 3],
    normal: Vec3,
    origin_offset: f32,
}
#[derive(Debug, Clone, Copy)]
pub struct Floor {
    tri: [Vec3; 3],
    normal: Vec3,
}
#[derive(Debug, Clone, Copy)]
pub struct Cieling {
    tri: [Vec3; 3],
    normal: Vec3,
}

#[derive(Debug, Clone, Copy)]
pub enum Surface {
    Wall(Wall),
    Floor(Floor),
    Cieling(Cieling),
}

impl Surface {
    pub fn new(tri: [Vec3; 3], normal: Vec3) -> Self {
        match normal.y {
            y if (-f32::INFINITY..-0.01).contains(&y) => Surface::Cieling(Cieling { tri, normal }),
            y if (0.01..f32::INFINITY).contains(&y) => Surface::Floor(Floor { tri, normal }),
            _ => Surface::Wall(Wall {
                tri,
                normal,
                origin_offset: -normal.dot(tri[0]),
            }),
        }
    }
}

pub fn get_face_normal(v1_pos: Vec3, v2_pos: Vec3, v3_pos: Vec3) -> Vec3 {
    let edge1 = v2_pos - v1_pos;
    let edge2 = v3_pos - v1_pos;

    edge1.cross(edge2).normalize()
}

// ground_step vs air_step?
// todo, add some kind of event pump to return?
pub fn get_step_push(
    pos: Vec3,
    diff: Vec3,
    radius: f32,
    height: f32,
    surfaces: &[Surface],
) -> Option<Vec3> {
    let mut target_pos = pos + diff;
    let mut has_diff = false;
    let mut out_diff = Vec3::ZERO;

    // max wall checks, break on no collisions
    // todo, pass in max count, or make this a method on a
    // config object that has a member of max_hits
    for _ in 0..4 {
        let mut collided = false;

        for wall in surfaces.iter().filter_map(|s| match s {
            Surface::Wall(w) => Some(w),
            _ => None,
        }) {
            if let Some(push) = check_wall_collision(target_pos, radius, height, wall) {
                collided = true;
                has_diff = true;
                target_pos += push;
                out_diff = target_pos - pos;
            }
        }

        // no collisions, skip other checks
        if !collided {
            break;
        }
    }

    match has_diff {
        true => Some(out_diff),
        false => None,
    }
}

// this could be vec2, but might actually be faster with Vec3A
fn closest_point_on_segment(p: Vec3, a: Vec3, b: Vec3) -> Vec3 {
    let ab = b - a;
    let ap = p - a;

    let proj = ap.dot(ab);
    let ab_len_sq = ab.length().powi(2);
    let d = proj / ab_len_sq;

    match d {
        NEG_INFINITY..=0f32 => a,
        1f32..=INFINITY => b,
        d => a + ab * d,
    }
}

// is it ok to do this check in 3d, despite plane being in 2d?
fn cylinder_intersects_segment(pos: Vec3, radius: f32, tri: &[Vec3; 3]) -> bool {
    // get closest for each segment
    let c_edge0 = closest_point_on_segment(pos, tri[0], tri[1]);
    let c_edge1 = closest_point_on_segment(pos, tri[1], tri[2]);
    let c_edge2 = closest_point_on_segment(pos, tri[2], tri[0]);

    (pos - c_edge0).length() <= radius
        || (pos - c_edge1).length() <= radius
        || (pos - c_edge2).length() <= radius
}

pub fn check_wall_collision(pos: Vec3, radius: f32, height: f32, wall: &Wall) -> Option<Vec3> {
    // cylinder intersection with infinite plane
    let offset = wall.normal.dot(pos) + wall.origin_offset;
    if offset.abs() >= radius {
        return None;
    }

    // triangle's min and max y
    let min_y = wall.tri[0].y.min(wall.tri[1].y).min(wall.tri[2].y);
    let max_y = wall.tri[0].y.max(wall.tri[1].y).max(wall.tri[2].y);

    let bot = pos.y;
    let top = pos.y + height;

    // skip due to height
    if top < min_y || bot > max_y {
        return None;
    }

    // skip due to distance to segments
    if !cylinder_intersects_segment(pos, radius, &wall.tri) {
        return None;
    }

    let depth = radius - offset.abs();
    let push = depth * offset.signum();

    Some(Vec3::new(wall.normal.x * push, 0., wall.normal.z * push))
}
