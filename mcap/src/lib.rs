use core::f32::{INFINITY, NEG_INFINITY};
use std::ops::Neg;

pub use glam::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    verts: [Vec3; 3],
    normal: Vec3,
    origin_offset: f32,
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
    pub collided: bool,
    pub collisions: Vec<Surface>,
    pub collision_points: Vec<Vec3>,
    pub final_pos: Vec3,
    pub out_step: Vec3,
    pub became_airborne: bool,
    pub landed: bool,
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
    floor_snap_dist: f32,
    surfaces: &[Surface],
) -> StepResult {
    let mut target_pos = pos + step;
    let mut collided = false;
    let mut step = step;
    let mut airborne = false;

    // max wall checks, break on no collisionsb
    // todo, pass in max count, or make this a method on a
    // config object that has a member of max_hits
    //
    // or check a 2d capsule (hotddog) to see where it intersects the triangle
    // and skip the stepping
    for _ in 0..4 {
        for wall in surfaces.iter().filter_map(|s| match s {
            Surface::Wall(w) => Some(w),
            _ => None,
        }) {
            // naive push, but maybe it should do the projection like with floor height below
            if let Some(push) = check_circle_tri_collision(
                target_pos.with_y(target_pos.y + chest_height),
                radius,
                wall,
            ) {
                collided = true;
                target_pos += push;
                continue;
            }
        }
        break;
    }

    if let Some((floor, y)) = find_floor_height(target_pos, floor_snap_dist, surfaces) {
        target_pos.y = y;
        step.y = 0.;
        // project step onto floor normal
        step -= floor.normal * step.dot(floor.normal);
    } else {
        // walked off ledge, become airborne
        airborne = true;
    }

    StepResult {
        collided: collided,
        collisions: vec![],
        collision_points: vec![],
        final_pos: target_pos,
        out_step: step,
        became_airborne: airborne,
        landed: false,
    }
}

pub fn closest_point_on_segment(p: Vec3, a: Vec3, b: Vec3) -> Vec3 {
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
//
// todo, also check with unflattened??
// todo, replace with raylib's -- CheckCollisionCircleLine * 3??
pub fn flattened_cylinder_intersects_flattened_triangle(
    pos: Vec3,
    radius: f32,
    tri: &[Vec3; 3],
) -> bool {
    let pos_xz = pos.with_y(0.);

    // could return immediately after each of these, if one is <= radius
    let edge_xz0 = closest_point_on_segment(pos_xz, tri[0].with_y(0.), tri[1].with_y(0.));
    let edge_xz1 = closest_point_on_segment(pos_xz, tri[1].with_y(0.), tri[2].with_y(0.));
    let edge_xz2 = closest_point_on_segment(pos_xz, tri[2].with_y(0.), tri[0].with_y(0.));

    (pos_xz - edge_xz0).length() <= radius
        || (pos_xz - edge_xz1).length() <= radius
        || (pos_xz - edge_xz2).length() <= radius
}

pub fn check_circle_tri_collision(pos: Vec3, radius: f32, wall: &Triangle) -> Option<Vec3> {

    // check if pos is in the direction of the wall's normal
    let t1 = wall.verts()[0];
    let v = pos - t1;
    if v.dot(wall.normal) < 0. {
        return None
    }

    // cylinder intersection with infinite plane
    let offset = wall.normal.dot(pos) + wall.origin_offset;
    if offset.abs() >= radius {
        return None;
    }

    // triangle's min and max y
    let min_y = wall.verts[0].y.min(wall.verts[1].y).min(wall.verts[2].y);
    let max_y = wall.verts[0].y.max(wall.verts[1].y).max(wall.verts[2].y);

    // check if posy is within miny/maxy of triangle
    if !(min_y..=max_y).contains(&pos.y) {
        return None;
    }
    
    // // skip due to distance to segments
    // // this has an issue with colliding with 2 walls
    // if !flattened_cylinder_intersects_flattened_triangle(pos, radius, &wall.verts) {
    //     return None;
    // }

    // // adding a skin to the above helps a little bit with
    // // the corner bumping, but it's not a solution
    // let push = (radius - offset) + 0.01;

    // ========================================================================================

    // sm64 wall check, I don't really understand how it works,
    // it also seems to have a problem with walking through corners
    let (px, py, pz) = (pos.x, pos.y, pos.z);

    let use_x_projection = wall.normal.x.abs() > wall.normal.z.abs();
    if use_x_projection {
        // project on yz
        let (w1, w2, w3) = (-wall.verts[0].z, -wall.verts[1].z, -wall.verts[2].z);
        let (y1, y2, y3) = (wall.verts[0].y, wall.verts[1].y, wall.verts[2].y);

        if wall.normal.x > 0.0 {
            if (y1 - py) * (w2 - w1) - (w1 - -pz) * (y2 - y1) > 0.0 { return None; }
            if (y2 - py) * (w3 - w2) - (w2 - -pz) * (y3 - y2) > 0.0 { return None; }
            if (y3 - py) * (w1 - w3) - (w3 - -pz) * (y1 - y3) > 0.0 { return None; }
        } else {
            if (y1 - py) * (w2 - w1) - (w1 - -pz) * (y2 - y1) < 0.0 { return None; }
            if (y2 - py) * (w3 - w2) - (w2 - -pz) * (y3 - y2) < 0.0 { return None; }
            if (y3 - py) * (w1 - w3) - (w3 - -pz) * (y1 - y3) < 0.0 { return None; }
        }
    } else {
        // porject on xy
        let (w1, w2, w3) = (wall.verts[0].x, wall.verts[1].x, wall.verts[2].x);
        let (y1, y2, y3) = (wall.verts[0].y, wall.verts[1].y, wall.verts[2].y);

        if wall.normal.z > 0.0 {
            if (y1 - py) * (w2 - w1) - (w1 - px) * (y2 - y1) > 0.0 { return None; }
            if (y2 - py) * (w3 - w2) - (w2 - px) * (y3 - y2) > 0.0 { return None; }
            if (y3 - py) * (w1 - w3) - (w3 - px) * (y1 - y3) > 0.0 { return None; }
        } else {
            if (y1 - py) * (w2 - w1) - (w1 - px) * (y2 - y1) < 0.0 { return None; }
            if (y2 - py) * (w3 - w2) - (w2 - px) * (y3 - y2) < 0.0 { return None; }
            if (y3 - py) * (w1 - w3) - (w3 - px) * (y1 - y3) < 0.0 { return None; }
        }
    }

    let push = (radius - offset) + 0.01;
    eprintln!("push: {}", push);

    Some(Vec3::new(wall.normal.x * push, 0., wall.normal.z * push))
}

pub fn solve_plane_y(normal: Vec3, origin_offset: f32, x: f32, z: f32) -> f32 {
    //   Ax + By + Cz + D   = 0
    //   Ax + Cz + D        = -By
    //  (Ax + Cz + D) / B   = -y
    // -(Ax + Cz + D) / B   = y
    -(normal.x * x + normal.z * z + origin_offset) / normal.y
}

// pos here is the feet/bottom
pub fn find_floor_height(
    pos: Vec3,
    floor_snap_dist: f32,
    surfaces: &[Surface],
) -> Option<(Triangle, f32)> {
    let mut best_y = f32::NEG_INFINITY;
    let mut best_tri = None;

    for s in surfaces {
        // all upward facing normals
        let tri = match s {
            Surface::Floor(t) | Surface::Slide(t) => t,
            _ => continue,
        };

        // todo, could swap this with flattened_cylinder_intersects_flattened_triangle(very_small_radius)
        // to ensure that we don't floating point raycast down between the seam of 2 neighboring triangles
        if !flattened_point_inside_flattened_triangle(pos, tri.verts[0], tri.verts[1], tri.verts[2])
        {
            continue;
        }

        // get y at xz
        let y = solve_plane_y(tri.normal, tri.origin_offset, pos.x, pos.z);

        // within floor snap range both below and above pos (bottom)
        if (pos.y - floor_snap_dist..=pos.y + floor_snap_dist).contains(&y) && y > best_y {
            best_y = y;
            best_tri = Some(*tri);
        }
    }

    if best_tri.is_some() {
        Some((best_tri.unwrap(), best_y))
    } else {
        None
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
