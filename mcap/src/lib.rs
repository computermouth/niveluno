use core::f32;
use std::f32::{INFINITY, NEG_INFINITY};

pub use glam::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    verts: [Vec3; 3],
    normal: Vec3,
    origin_offset: f32,
}

const FLOOR_EPS: f32 = 0.01;
// todo, make configurable
const FLOOR_SNAP_HEIGHT: f32 = 0.4;

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

pub fn get_face_normal(v1_pos: Vec3, v2_pos: Vec3, v3_pos: Vec3) -> Vec3 {
    let edge1 = v2_pos - v1_pos;
    let edge2 = v3_pos - v1_pos;

    edge1.cross(edge2).normalize()
}

pub struct StepResult {
    collided: bool,
    collisions: Vec<Surface>,
    // can't really do this, as we just get the 2d point
    collision_points: Vec<Vec3>,
}

// ground_step vs air_step?
// todo, add some kind of event pump to return?
//
// todo, always return vec3 of diff, optional list of collisions,
// also whether or not we've stopped short and changed state
//
// todo, have one step function, set callbacks for each or groups
// of surface type?
pub fn get_step_push(
    // pos is feet position
    pos: Vec3,
    step: Vec3,
    radius: f32,
    chest_height: f32,
    surfaces: &[Surface],
) -> Option<Vec3> {
    let mut target_pos = pos + step;
    let mut collided = false;

    // max wall checks, break on no collisionsb
    // todo, pass in max count, or make this a method on a
    // config object that has a member of max_hits
    //
    // or check a 2d capsule to see where it intersects the triangle
    // and skip the stepping
    for _ in 0..4 {
        let mut collided_step = false;

        for wall in surfaces.iter().filter_map(|s| match s {
            Surface::Wall(w) => Some(w),
            _ => None,
        }) {
            if let Some(push) = check_circle_tri_collision(
                target_pos.with_y(target_pos.y + chest_height),
                radius,
                wall,
            ) {
                collided_step = true;
                collided = true;
                target_pos += push;
            }
        }

        // no collisions, skip other checks
        if !collided_step {
            break;
        }
    }

    match collided {
        true => Some(target_pos - pos),
        false => None,
    }
}

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

// flatten on y, if nearest point on edge_xz is within radius of pos_xz
// (this would also be sphere_intersects_segment if we didn't flatten y)
pub fn flattened_cylinder_intersects_flattened_triangle(
    pos: Vec3,
    radius: f32,
    tri: &[Vec3; 3],
) -> bool {
    let pos_xz = pos.with_y(0.);

    let edge_xz0 = closest_point_on_segment(pos_xz, tri[0].with_y(0.), tri[1].with_y(0.));
    let edge_xz1 = closest_point_on_segment(pos_xz, tri[1].with_y(0.), tri[2].with_y(0.));
    let edge_xz2 = closest_point_on_segment(pos_xz, tri[2].with_y(0.), tri[0].with_y(0.));

    (pos_xz - edge_xz0).length() <= radius
        || (pos_xz - edge_xz1).length() <= radius
        || (pos_xz - edge_xz2).length() <= radius
}

pub fn check_circle_tri_collision(pos: Vec3, radius: f32, wall: &Triangle) -> Option<Vec3> {
    // cylinder intersection with infinite plane
    let offset = wall.normal.dot(pos) + wall.origin_offset;
    if offset.abs() >= radius {
        return None;
    }

    // triangle's min and max y
    let min_y = wall.verts[0].y.min(wall.verts[1].y).min(wall.verts[2].y);
    let max_y = wall.verts[0].y.max(wall.verts[1].y).max(wall.verts[2].y);

    // check if posy is within miny/maxy of triangle
    if !(pos.y >= min_y) || !(pos.y <= max_y) {
        return None;
    }

    // skip due to distance to segments
    if !flattened_cylinder_intersects_flattened_triangle(pos, radius, &wall.verts) {
        return None;
    }

    let depth = radius - offset.abs();
    let push = depth * offset.signum();

    Some(Vec3::new(wall.normal.x * push, 0., wall.normal.z * push))
}
