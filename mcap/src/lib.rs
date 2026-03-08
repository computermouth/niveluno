pub mod scrap {

    use core::f32;
    use core::f32::{INFINITY, NEG_INFINITY};
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::collections::VecDeque;

    pub use glam::Vec2;
    pub use glam::Vec3;

    use line_clipping::cohen_sutherland::clip_line;
    use line_clipping::{LineSegment, Point, Window};
    use raylib::color::Color;
    use raylib::prelude::RaylibDraw;
    use raylib::prelude::RaylibDrawHandle;

    #[derive(Debug, Clone, Copy)]
    pub struct Triangle {
        pub verts: [Vec3; 3],
        pub normal: Vec3,
        pub origin_offset: f32,
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

    pub fn get_step_push_most_opposing(
        // pos is feet position
        pos: Vec3,
        step: Vec3,
        radius: f32,
        chest_height: f32,
        floor_snap_dist: f32,
        surfaces: &[&Surface],
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
            let mut collision: Option<(Vec3, Triangle, Vec3)> = None;
            let mut collided_this_iteration = false;

            for wall in surfaces.iter().filter_map(|s| match s {
                Surface::Wall(w) => Some(w),
                _ => None,
            }) {
                // naive push, but maybe it should do the projection like with floor height below
                if let Some((push, point)) = check_circle_tri_collision(
                    target_pos.with_y(target_pos.y + chest_height),
                    radius,
                    wall,
                ) {
                    collided = true;
                    collided_this_iteration = true;

                    if let Some((_old_push, old_wall, _old_point)) = collision {
                        let old_dot = step.dot(old_wall.normal);
                        let new_dot = step.dot(wall.normal);

                        // heading more into new wall
                        if new_dot < old_dot {
                            collision = Some((push, *wall, point));
                        }
                    } else {
                        collision = Some((push, *wall, point))
                    }
                }
            }

            if let Some((push, _wall, _point)) = collision {
                target_pos += push;
            }

            if let Some((floor, y)) = find_floor_height_m64(target_pos, floor_snap_dist, surfaces) {
                target_pos.y = y;
                step.y = 0.;
                // project step onto floor normal
                step -= floor.normal * step.dot(floor.normal);
            } else {
                // walked off ledge, become airborne
                airborne = true;
                break;
            }

            if !collided_this_iteration {
                break;
            }
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
        surfaces: &[&Surface],
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
            let mut collided_this_iteration = false;

            for wall in surfaces.iter().filter_map(|s| match s {
                Surface::Wall(w) => Some(w),
                _ => None,
            }) {
                // naive push, but maybe it should do the projection like with floor height below
                if let Some((push, _)) = check_circle_tri_collision(
                    target_pos.with_y(target_pos.y + chest_height),
                    radius,
                    wall,
                ) {
                    collided = true;
                    collided_this_iteration = true;
                    target_pos += push;
                    continue;
                }
            }

            if let Some((floor, y)) = find_floor_height_m64(target_pos, floor_snap_dist, surfaces) {
                target_pos.y = y;
                step.y = 0.;
                // project step onto floor normal
                step -= floor.normal * step.dot(floor.normal);
            } else {
                // walked off ledge, become airborne
                airborne = true;
                break;
            }

            if !collided_this_iteration {
                break;
            }
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

    // ground_step vs air_step?
    // todo, add some kind of event pump to return?
    //
    // todo, always return vec3 of diff, optional list of collisions,
    // also whether or not we've stopped short and changed state
    //
    // todo, have one step function, set callbacks for each or groups
    // of surface type?
    pub fn get_step_push_m64(
        // pos is feet position
        pos: Vec3,
        step: Vec3,
        radius: f32,
        chest_height: f32,
        floor_snap_dist: f32,
        surfaces: &[&Surface],
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
            let mut collided_this_iteration = false;

            for wall in surfaces.iter().filter_map(|s| match s {
                Surface::Wall(w) => Some(w),
                _ => None,
            }) {
                // naive push, but maybe it should do the projection like with floor height below
                if let Some(push) = check_circle_tri_collision_m64(
                    target_pos.with_y(target_pos.y + chest_height),
                    radius,
                    wall,
                ) {
                    collided = true;
                    collided_this_iteration = true;
                    target_pos += push;
                }
            }

            if let Some((floor, y)) = find_floor_height_m64(target_pos, floor_snap_dist, surfaces) {
                target_pos.y = y;
                step.y = 0.;
                // project step onto floor normal
                step -= floor.normal * step.dot(floor.normal);
            } else {
                // walked off ledge, become airborne
                airborne = true;
                break;
            }

            if !collided_this_iteration {
                break;
            }
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

    pub fn get_step_push_oot(
        pos: Vec3,
        step: Vec3,
        radius: f32,
        _chest_height: f32,
        _floor_snap_dist: f32,
        surfaces: &[&Surface],
    ) -> StepResult {
        let target = pos + step;

        let walls: Vec<&Triangle> = surfaces
            .iter()
            .filter_map(|s| match s {
                Surface::Wall(w) => Some(w),
                _ => None,
            })
            .collect();

        let (out_pos, collided) = push_out_walls(target, radius, walls);

        StepResult {
            collided: collided,
            collisions: vec![],
            collision_points: vec![],
            final_pos: out_pos,
            out_step: step,
            became_airborne: false,
            landed: false,
        }
    }

    pub fn push_out_walls(pos: Vec3, radius: f32, walls: Vec<&Triangle>) -> (Vec3, bool) {
        let mut out_x = pos.x;
        let mut out_z = pos.z;
        let mut hit = false;

        let radius = radius + radius * SKIN_FACTOR * 10.;

        for tri in walls {
            let n = tri.normal;
            let xz_len = (n.x * n.x + n.z * n.z).sqrt();

            let cur = Vec3::new(out_x, pos.y, out_z);
            let nearest = closest_point_triangle(cur, &tri.verts);
            let dist = nearest.distance(cur);

            if dist.abs() >= radius {
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

    // flatten on y, if nearest point on edge_xz is within radius of pos_xz
    // (this would also be sphere_intersects_segment if we didn't flatten y)
    //
    // todo, also check with unflattened??
    // todo, replace with raylib's -- CheckCollisionCircleLine * 3??
    pub fn flattened_cylinder_intersects_flattened_triangle(
        pos: Vec3,
        radius: f32,
        tri: &[Vec3; 3],
    ) -> Option<Vec3> {
        let pos_xz = pos.with_y(0.);

        // could return immediately after each of these, if one is <= radius
        let edge_xz0 = closest_point_on_segment_v3(pos_xz, tri[0].with_y(0.), tri[1].with_y(0.));
        if (pos_xz - edge_xz0).length() <= radius {
            return Some(edge_xz0);
        }
        let edge_xz1 = closest_point_on_segment_v3(pos_xz, tri[1].with_y(0.), tri[2].with_y(0.));
        if (pos_xz - edge_xz1).length() <= radius {
            return Some(edge_xz1);
        }
        let edge_xz2 = closest_point_on_segment_v3(pos_xz, tri[2].with_y(0.), tri[0].with_y(0.));
        if (pos_xz - edge_xz2).length() <= radius {
            return Some(edge_xz2);
        }

        None
    }

    // find collision, and return nearest xz point
    pub fn circle_wall_for_hotdog(pos: Vec3, radius: f32, tri: &[Vec3; 3]) -> Option<Vec3> {
        let pos_xz = pos.with_y(0.);

        let edge_xz0 = closest_point_on_segment_v3(pos_xz, tri[0].with_y(0.), tri[1].with_y(0.));
        let edge_xz1 = closest_point_on_segment_v3(pos_xz, tri[1].with_y(0.), tri[2].with_y(0.));
        let edge_xz2 = closest_point_on_segment_v3(pos_xz, tri[2].with_y(0.), tri[0].with_y(0.));

        let d0 = (pos_xz - edge_xz0).length();
        let d1 = (pos_xz - edge_xz1).length();
        let d2 = (pos_xz - edge_xz2).length();

        let nearest = d0.min(d1.min(d2));
        // too far, skip
        if nearest > radius {
            return None;
        }
        // correlate to the hit
        if nearest == d0 {
            return Some(edge_xz0);
        } else if nearest == d1 {
            return Some(edge_xz1);
        } else if nearest == d2 {
            return Some(edge_xz2);
        }

        None
    }

    // find collision of a rect with each triangle edge
    // and return nearest xz point inside rect
    pub fn rect_wall_for_hotdog(
        _src: Vec3,
        _dst: Vec3,
        _radius: f32,
        _tri: &[Vec3; 3],
    ) -> Option<Vec3> {
        todo!()
    }

    pub fn check_circle_tri_collision(
        pos: Vec3,
        radius: f32,
        wall: &Triangle,
    ) -> Option<(Vec3, Vec3)> {
        // check if pos is in the direction of the wall's normal
        let t1 = wall.verts()[0];
        let v = pos - t1;
        if v.dot(wall.normal) < 0. {
            return None;
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

        if let Some(point) =
            flattened_cylinder_intersects_flattened_triangle(pos, radius, &wall.verts)
        {
            let push = radius - offset;
            Some((
                Vec3::new(wall.normal.x * push, 0., wall.normal.z * push),
                point,
            ))
        } else {
            None
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

        // the above should be fine if we located the nearest collision
        //   - if there are 2 matches, find the one which has a normal least similar to current direction
        // for 0..4 {
        //   1. loop over all walls
        //   2. store nearest with tiebreaker on normal
        //   3. process the push of the chosen collision
        //   4. maybe increase iter count
        // }

        // ========================================================================================

        // // sm64 wall check, I don't really understand how it works,
        // // it also seems to have a problem with walking through corners
        // let (px, py, pz) = (pos.x, pos.y, pos.z);

        // let use_x_projection = wall.normal.x.abs() > wall.normal.z.abs();
        // if use_x_projection {
        //     // project on yz
        //     let (w1, w2, w3) = (-wall.verts[0].z, -wall.verts[1].z, -wall.verts[2].z);
        //     let (y1, y2, y3) = (wall.verts[0].y, wall.verts[1].y, wall.verts[2].y);

        //     if wall.normal.x > 0.0 {
        //         if (y1 - py) * (w2 - w1) - (w1 - -pz) * (y2 - y1) > 0.0 { return None; }
        //         if (y2 - py) * (w3 - w2) - (w2 - -pz) * (y3 - y2) > 0.0 { return None; }
        //         if (y3 - py) * (w1 - w3) - (w3 - -pz) * (y1 - y3) > 0.0 { return None; }
        //     } else {
        //         if (y1 - py) * (w2 - w1) - (w1 - -pz) * (y2 - y1) < 0.0 { return None; }
        //         if (y2 - py) * (w3 - w2) - (w2 - -pz) * (y3 - y2) < 0.0 { return None; }
        //         if (y3 - py) * (w1 - w3) - (w3 - -pz) * (y1 - y3) < 0.0 { return None; }
        //     }
        // } else {
        //     // porject on xy
        //     let (w1, w2, w3) = (wall.verts[0].x, wall.verts[1].x, wall.verts[2].x);
        //     let (y1, y2, y3) = (wall.verts[0].y, wall.verts[1].y, wall.verts[2].y);

        //     if wall.normal.z > 0.0 {
        //         if (y1 - py) * (w2 - w1) - (w1 - px) * (y2 - y1) > 0.0 { return None; }
        //         if (y2 - py) * (w3 - w2) - (w2 - px) * (y3 - y2) > 0.0 { return None; }
        //         if (y3 - py) * (w1 - w3) - (w3 - px) * (y1 - y3) > 0.0 { return None; }
        //     } else {
        //         if (y1 - py) * (w2 - w1) - (w1 - px) * (y2 - y1) < 0.0 { return None; }
        //         if (y2 - py) * (w3 - w2) - (w2 - px) * (y3 - y2) < 0.0 { return None; }
        //         if (y3 - py) * (w1 - w3) - (w3 - px) * (y1 - y3) < 0.0 { return None; }
        //     }
        // }

        // let push = radius - offset;

        // Some(Vec3::new(wall.normal.x * push, 0., wall.normal.z * push))
    }

    pub fn check_circle_tri_collision_m64(pos: Vec3, radius: f32, wall: &Triangle) -> Option<Vec3> {
        // check if pos is in the direction of the wall's normal
        let t1 = wall.verts()[0];
        let v = pos - t1;
        if v.dot(wall.normal) < 0. {
            return None;
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

        // the above should be fine if we located the nearest collision
        //   - if there are 2 matches, find the one which has a normal least similar to current direction
        // for 0..4 {
        //   1. loop over all walls
        //   2. store nearest with tiebreaker on normal
        //   3. process the push of the chosen collision
        //   4. maybe increase iter count
        // }

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
                if (y1 - py) * (w2 - w1) - (w1 - -pz) * (y2 - y1) > 0.0 {
                    return None;
                }
                if (y2 - py) * (w3 - w2) - (w2 - -pz) * (y3 - y2) > 0.0 {
                    return None;
                }
                if (y3 - py) * (w1 - w3) - (w3 - -pz) * (y1 - y3) > 0.0 {
                    return None;
                }
            } else {
                if (y1 - py) * (w2 - w1) - (w1 - -pz) * (y2 - y1) < 0.0 {
                    return None;
                }
                if (y2 - py) * (w3 - w2) - (w2 - -pz) * (y3 - y2) < 0.0 {
                    return None;
                }
                if (y3 - py) * (w1 - w3) - (w3 - -pz) * (y1 - y3) < 0.0 {
                    return None;
                }
            }
        } else {
            // porject on xy
            let (w1, w2, w3) = (wall.verts[0].x, wall.verts[1].x, wall.verts[2].x);
            let (y1, y2, y3) = (wall.verts[0].y, wall.verts[1].y, wall.verts[2].y);

            if wall.normal.z > 0.0 {
                if (y1 - py) * (w2 - w1) - (w1 - px) * (y2 - y1) > 0.0 {
                    return None;
                }
                if (y2 - py) * (w3 - w2) - (w2 - px) * (y3 - y2) > 0.0 {
                    return None;
                }
                if (y3 - py) * (w1 - w3) - (w3 - px) * (y1 - y3) > 0.0 {
                    return None;
                }
            } else {
                if (y1 - py) * (w2 - w1) - (w1 - px) * (y2 - y1) < 0.0 {
                    return None;
                }
                if (y2 - py) * (w3 - w2) - (w2 - px) * (y3 - y2) < 0.0 {
                    return None;
                }
                if (y3 - py) * (w1 - w3) - (w3 - px) * (y1 - y3) < 0.0 {
                    return None;
                }
            }
        }

        let push = radius - offset;

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
    pub fn find_floor_height_hotdog(
        pos: Vec3,
        floor_snap_dist: f32,
        surfaces: &[&Surface],
        radius: f32,
    ) -> Option<(Triangle, f32)> {
        let mut best_y = f32::NEG_INFINITY;
        let mut best_tri = None;

        // center ray check
        for s in surfaces {
            // all upward facing normals
            let tri = match s {
                Surface::Floor(t) => t,
                _ => continue,
            };

            // check if point is inside triangle
            if !flattened_point_inside_flattened_triangle(
                pos,
                tri.verts[0],
                tri.verts[1],
                tri.verts[2],
            ) {
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

        // early return if the point is inside a triangle
        // otherwise snapping floors on change to/from incline gets weird
        match best_tri {
            Some(tri) => return Some((tri, best_y)),
            None => {}
        }

        // check if we're teetering on an edge with more than floor snap distance
        for s in surfaces {
            // all upward facing normals
            let tri = match s {
                Surface::Floor(t) | Surface::Slide(t) => t,
                _ => continue,
            };

            let posv2 = Vec2::new(pos.x, pos.z);
            let p1v2 = Vec2::new(tri.verts[0].x, tri.verts[0].z);
            let p2v2 = Vec2::new(tri.verts[1].x, tri.verts[1].z);
            let p3v2 = Vec2::new(tri.verts[2].x, tri.verts[2].z);

            // check if point is inside radius-expanded edges,
            // 2x SKIN_FACTOR to ensure we're always dropping down outside a wall that connects to a floor
            //
            // but also fall back to using point in triangle with slides
            if !flattened_point_inside_flattened_triangle(
                pos,
                tri.verts[0],
                tri.verts[1],
                tri.verts[2],
            ) && !(closest_point_on_segment_v2(posv2, p1v2, p2v2).distance(posv2)
                <= radius + radius * SKIN_FACTOR * 2.)
                && !(closest_point_on_segment_v2(posv2, p2v2, p3v2).distance(posv2)
                    <= radius + radius * SKIN_FACTOR * 2.)
                && !(closest_point_on_segment_v2(posv2, p3v2, p1v2).distance(posv2)
                    <= radius + radius * SKIN_FACTOR * 2.)
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

        match best_tri {
            Some(tri) => Some((tri, best_y)),
            None => None,
        }
    }

    // pos here is the feet/bottom
    pub fn find_floor_height_hotdog_v2(
        pos: Vec3,
        floor_snap_dist: f32,
        surfaces: &[&Surface],
        radius: f32,
    ) -> Option<(Triangle, f32)> {
        let mut best_dist = f32::INFINITY;
        let mut best_y = f32::NEG_INFINITY;
        let mut best_tri = None;

        let posv2 = Vec2::new(pos.x, pos.z);

        // check if we're teetering on an edge with more than floor snap distance
        for s in surfaces {
            // all upward facing normals
            let tri = match s {
                Surface::Floor(t) | Surface::Slide(t) => t,
                _ => continue,
            };

            let p1v2 = Vec2::new(tri.verts[0].x, tri.verts[0].z);
            let p2v2 = Vec2::new(tri.verts[1].x, tri.verts[1].z);
            let p3v2 = Vec2::new(tri.verts[2].x, tri.verts[2].z);

            // check if point is inside radius-expanded edges,
            // 2x SKIN_FACTOR to ensure we're always dropping down outside a wall that connects to a floor
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
                let threshold = radius + radius * SKIN_FACTOR * 2.;
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

            // within floor snap range both below and above pos (bottom)
            // add a SKIN_FACTOR just for fun (my step-{up,down} is coincidentally a multiple of my minimum alignment in blender)
            let snap = floor_snap_dist + SKIN_FACTOR;
            let dist = plane_pos.distance(posv2);
            // store according to which plane_pos is nearest to pos_v2
            if (pos.y - snap..=pos.y + snap).contains(&y) && dist < best_dist {
                best_dist = dist;
                best_y = y;
                best_tri = Some(*tri);
            }
        }

        match best_tri {
            Some(tri) => Some((tri, best_y)),
            None => None,
        }
    }

    // pos here is the feet/bottom
    pub fn find_floor_height_hotdog_v3(
        pos: Vec3,
        snap_up: f32,
        snap_down: f32,
        surfaces: &[&Surface],
        radius: f32,
    ) -> Option<(Surface, f32)> {
        let mut best_dist = f32::INFINITY;
        let mut best_y = f32::NEG_INFINITY;
        let mut best_surf = None;

        let posv2 = Vec2::new(pos.x, pos.z);

        // check if we're teetering on an edge with more than floor snap distance
        for s in surfaces {
            // all upward facing normals
            let tri = match s {
                Surface::Floor(t) | Surface::Slide(t) => t,
                _ => continue,
            };

            let p1v2 = Vec2::new(tri.verts[0].x, tri.verts[0].z);
            let p2v2 = Vec2::new(tri.verts[1].x, tri.verts[1].z);
            let p3v2 = Vec2::new(tri.verts[2].x, tri.verts[2].z);

            // check if point is inside radius-expanded edges,
            // 2x SKIN_FACTOR to ensure we're always dropping down outside a wall that connects to a floor
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
                let threshold = radius + radius * SKIN_FACTOR * 2.;
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

            // within floor snap range both below and above pos (bottom)
            // add a SKIN_FACTOR just for fun (my step-{up,down} is coincidentally a multiple of my minimum alignment in map)
            let bottom = pos.y - snap_down - SKIN_FACTOR;
            let top = pos.y + snap_up + SKIN_FACTOR;
            let dist = plane_pos.distance(posv2);
            // store according to which plane_pos is nearest to pos_v2
            if (bottom..=top).contains(&y) && dist < best_dist {
                best_dist = dist;
                best_y = y;
                best_surf = Some(**s);
            }
        }

        match best_surf {
            Some(surf) => Some((surf, best_y)),
            None => None,
        }
    }

    // pos here is the feet/bottom
    pub fn find_floor_height_m64(
        pos: Vec3,
        floor_snap_dist: f32,
        surfaces: &[&Surface],
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
            if !flattened_point_inside_flattened_triangle(
                pos,
                tri.verts[0],
                tri.verts[1],
                tri.verts[2],
            ) {
                continue;
            }

            // get y at xz
            let y = solve_plane_y(tri.normal, tri.origin_offset, pos.x, pos.z);

            // within floor snap range both below and above pos (bottom)
            // add a SKIN_FACTOR just for fun (my step-{up,down} is coincidentally a multiple of my minimum alignment in blender)
            let snap = floor_snap_dist + SKIN_FACTOR;
            if (pos.y - snap..=pos.y + snap).contains(&y) && y > best_y {
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

    // pos here is the feet/bottom
    pub fn find_floor_height_m64_2(
        pos: Vec3,
        floor_snap_dist: f32,
        surfaces: &[&Surface],
    ) -> Option<(Surface, f32)> {
        let mut best_y = f32::NEG_INFINITY;
        let mut best_surf = None;

        for s in surfaces {
            // all upward facing normals
            let tri = match s {
                Surface::Floor(t) | Surface::Slide(t) => t,
                _ => continue,
            };

            // todo, could swap this with flattened_cylinder_intersects_flattened_triangle(very_small_radius)
            // to ensure that we don't floating point raycast down between the seam of 2 neighboring triangles
            if !flattened_point_inside_flattened_triangle(
                pos,
                tri.verts[0],
                tri.verts[1],
                tri.verts[2],
            ) {
                continue;
            }

            // get y at xz
            let y = solve_plane_y(tri.normal, tri.origin_offset, pos.x, pos.z);

            // within floor snap range both below and above pos (bottom)
            // add a SKIN_FACTOR just for fun (my step-{up,down} is coincidentally a multiple of my minimum alignment in blender)
            let snap = floor_snap_dist + SKIN_FACTOR;
            if (pos.y - snap..=pos.y + snap).contains(&y) && y > best_y {
                best_y = y;
                best_surf = Some(**s);
            }
        }

        if best_surf.is_some() {
            Some((best_surf.unwrap(), best_y))
        } else {
            None
        }
    }

    // pos here is the feet/bottom
    pub fn find_cieling_height_m64(
        pos: Vec3,
        height: f32,      // offset from the bottom
        range_above: f32, // range above offset which procs
        surfaces: &[&Surface],
    ) -> Option<(Triangle, f32)> {
        let mut best_y = f32::INFINITY;
        let mut best_tri = None;

        for s in surfaces {
            // all downward facing normals
            let tri = match s {
                Surface::Cieling(t) => t,
                _ => continue,
            };

            // todo, could swap this with flattened_cylinder_intersects_flattened_triangle(very_small_radius)
            // to ensure that we don't floating point raycast down between the seam of 2 neighboring triangles
            if !flattened_point_inside_flattened_triangle(
                pos,
                tri.verts[0],
                tri.verts[1],
                tri.verts[2],
            ) {
                continue;
            }

            // get y at xz
            let y = solve_plane_y(tri.normal, tri.origin_offset, pos.x, pos.z);
            // move down by a SKIN_FACTOR just for fun
            let y = y - SKIN_FACTOR;

            // within head snap range
            if (pos.y + height..=pos.y + height + range_above).contains(&y) && y < best_y {
                // if y > pos.y && (pos.y + height + SKIN_FACTOR) > y && y < best_y {
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

    // pos here is the feet/bottom
    pub fn find_ciel_height_hotdog_v3(
        pos: Vec3,
        offset: f32,      // offset from the bottom
        range_above: f32, // range above offset which procs
        surfaces: &[&Surface],
        radius: f32,
    ) -> Option<(Surface, f32)> {
        let mut best_dist = f32::INFINITY;
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
            let dist = plane_pos.distance(posv2);
            // store according to which plane_pos is nearest to pos_v2
            if (bottom..=top).contains(&y) && dist < best_dist {
                best_dist = dist;
                best_y = y;
                best_surf = Some(**s);
            }
        }

        match best_surf {
            Some(surf) => Some((surf, best_y)),
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

    // get line segment where horizontal plane at y intersects triangle
    pub fn triangle_slice_at_y(verts: &[Vec3; 3], y: f32) -> Option<(Vec2, Vec2)> {
        let mut points = Vec::new();

        let edges = [
            (&verts[0], &verts[1]),
            (&verts[1], &verts[2]),
            (&verts[2], &verts[0]),
        ];
        for (v1, v2) in edges {
            // edge is the intersection
            if v1.y == y && v2.y == y {
                return Some((Vec2::new(v1.x, v1.z), Vec2::new(v2.x, v2.z)));
            }

            // skip if edge doesn't cross y
            if (v1.y <= y && v2.y <= y) || (v1.y >= y && v2.y >= y) {
                continue;
            }

            // Find intersection point using linear interpolation
            let t = (y - v1.y) / (v2.y - v1.y);
            let x = v1.x + t * (v2.x - v1.x);
            let z = v1.z + t * (v2.z - v1.z);
            points.push(Vec2::new(x, z));
        }

        if points.len() >= 2 {
            return Some((points[0], points[1]));
        }

        None
    }

    #[derive(Debug, Copy, Clone)]
    pub struct HotDog {
        src: Vec2,
        // todo, remove??
        srcv3: Vec3,
        dst: Vec2,
        skin: f32,
        radius: f32,
        y_dir: Vec2,
        x_dir: Vec2,
        window: Window,
        original_dir: Vec2,
    }

    // todo, remove these, or only do them in debug builds
    thread_local! {
        static HDLIST: RefCell<VecDeque<HotDog>> = RefCell::new(VecDeque::new());
        static FI_LIST: RefCell<HashMap<usize,u128>> = RefCell::new(HashMap::new());
    }

    pub fn print_fi() {
        FI_LIST.with_borrow_mut(|hm| {
            let mut v: Vec<(&usize, &u128)> = hm.iter().collect();
            v.sort_by(|(ai, _), (bi, _)| ai.cmp(bi));
            eprintln!("hm: {:?}", v);
        });
    }

    // quake uses ~0.2%
    pub const SKIN_FACTOR: f32 = 0.002;

    impl HotDog {
        pub fn new(srcv3: Vec3, dstv3: Vec3, radius: f32, original_dir: Vec3) -> Self {
            let src = Vec2::new(srcv3.x, srcv3.z);
            let dst = Vec2::new(dstv3.x, dstv3.z);
            let diff = dst - src;
            let y_dir = diff.normalize();
            let length = diff.length();
            let skin_factor = SKIN_FACTOR;
            let skin = radius * skin_factor;
            let hd = Self {
                src,
                srcv3,
                dst,
                radius,
                skin,
                y_dir,
                x_dir: Vec2::new(-y_dir.y, y_dir.x),
                window: Window::new(-radius as f64, radius as f64, 0., length as f64),
                original_dir: Vec2::new(original_dir.x, original_dir.z).normalize(),
            };

            HDLIST.with_borrow_mut(|hotdogs| {
                if hotdogs.len() == 5 {
                    hotdogs.pop_front();
                }
                hotdogs.push_back(hd);
            });

            hd
        }

        pub fn clip_line_segment(&self, p1: Vec2, p2: Vec2) -> Option<(Vec2, Vec2)> {
            // todo, make origin_point a type `struct OriginPoint(Vec2)`
            let rp1 = self.origin_point_to_rect_space(p1);
            let rp2 = self.origin_point_to_rect_space(p2);
            match clip_line(
                LineSegment {
                    p1: Point {
                        x: rp1.x as f64,
                        y: rp1.y as f64,
                    },
                    p2: Point {
                        x: rp2.x as f64,
                        y: rp2.y as f64,
                    },
                },
                self.window,
            ) {
                None => None,
                Some(l) => {
                    let op1 =
                        self.rect_point_to_origin_space(Vec2::new(l.p1.x as f32, l.p1.y as f32));
                    let op2 =
                        self.rect_point_to_origin_space(Vec2::new(l.p2.x as f32, l.p2.y as f32));
                    Some((op1, op2))
                }
            }
        }

        pub fn closest_point_on_segment_origin(&self, a: Vec2, b: Vec2) -> Vec2 {
            closest_point_on_segment_v2(Vec2::ZERO, a, b)
        }

        pub fn closest_point_on_segment_rect(&self, a: Vec2, b: Vec2) -> Vec2 {
            let a = self.origin_point_to_rect_space(a);
            let b = self.origin_point_to_rect_space(b);

            closest_point_on_segment_v2(Vec2::ZERO, a, b)
        }

        pub fn origin_point_to_rect_space(&self, p: Vec2) -> Vec2 {
            // p relative to p_src
            let p_trans = p - self.src;

            // project p_tran onto the rectangle
            Vec2::new(p_trans.dot(self.x_dir), p_trans.dot(self.y_dir))
        }

        pub fn rect_point_to_origin_space(&self, p: Vec2) -> Vec2 {
            // project back on src with
            self.src + self.x_dir * p.x + self.y_dir * p.y
        }

        pub fn get_window(&self) -> Window {
            self.window
        }

        // find nearest point to circle collision
        // assumes line is already clipped to rectangle space
        //
        // this isn't perfectly accurate
        // but seems good enough
        pub fn closest_point_on_segment_rect_circ(&self, a: Vec2, b: Vec2) -> Vec2 {
            let a = self.origin_point_to_rect_space(a);
            let b = self.origin_point_to_rect_space(b);

            let closest_to_src = closest_point_on_segment_v2(Vec2::ZERO, a, b);

            let p = Vec2::new(0., closest_to_src.y);

            let tri = [
                Vec3::new(a.x, 0., a.y),
                Vec3::new(b.x, 1., b.y),
                Vec3::new(b.x, 0., b.y),
            ];

            let norm = get_face_normal(tri[0], tri[1], tri[2]);

            // back up by the z of the norm
            let z_backup = self.radius * norm.z;

            let mut out = p.with_y(closest_to_src.y - z_backup);

            // get closest segment endpoint to outpos,
            // if it's still inside the radius, bump further back up
            // I don't really know why this isn't resolved by the projection above
            let a_dist = out.distance(a);
            let b_dist = out.distance(b);
            let min_dist = a_dist.min(b_dist);
            if min_dist < self.radius {
                if min_dist == a_dist {
                    out.y -= self.radius - a_dist;
                } else {
                    out.y -= self.radius - b_dist;
                }
            }
            out
        }

        pub fn nearest_point_on_surfaces_for_rect(
            &self,
            surfaces: &[Surface],
        ) -> Option<HotDogCollision> {
            let walls: Vec<&Triangle> = surfaces
                .iter()
                .filter_map(|s| match s {
                    Surface::Wall(w) => Some(w),
                    _ => None,
                })
                .collect();

            struct Nearest {
                p: Vec2,
                norm: Vec2,
            }

            let mut nearest: Option<Nearest> = None;
            for tri in walls {
                // get 2 most distant points
                let a = Vec2::new(tri.verts[0].x, tri.verts[0].z);
                let b = Vec2::new(tri.verts[1].x, tri.verts[1].z);
                let c = Vec2::new(tri.verts[2].x, tri.verts[2].z);

                let ab = a.distance(b);
                let bc = b.distance(c);
                let ca = c.distance(a);

                let dist = ab.max(bc.max(ca));
                let (d1, d2) = match dist {
                    d if d == ab => (a, b),
                    d if d == bc => (b, c),
                    d if d == ca => (c, a),
                    _ => unreachable!(),
                };

                let tn2 = Vec2::new(tri.normal.x, tri.normal.z).normalize();

                // clip the 2 farthest points, and find nearest
                if let Some((p1, p2)) = self.clip_line_segment(d1, d2) {
                    // eprintln!("pt: {:?} {:?}", p1, p2);
                    // closest point rect
                    let new_xz = self.closest_point_on_segment_rect_circ(p1, p2);
                    match &mut nearest {
                        None => {
                            nearest = Some(Nearest {
                                p: new_xz,
                                norm: tn2,
                            });
                        }
                        Some(Nearest { p, norm: _ }) => {
                            if p.distance(Vec2::ZERO) > new_xz.distance(Vec2::ZERO) {
                                nearest = Some(Nearest {
                                    p: new_xz,
                                    norm: tn2,
                                });
                            }
                        }
                    }
                }
            }

            match nearest {
                None => None,
                Some(tri) => {
                    // project step onto wall plane (remove component going into wall)
                    let step1 = self.dst - self.rect_point_to_origin_space(tri.p);
                    let rejection_vec = step1.project_onto(tri.norm);
                    let slide_vec = step1 - rejection_vec;
                    Some(HotDogCollision {
                        dest_xz: tri.p,
                        next_move: slide_vec,
                        push_out: rejection_vec,
                        next_move_len: 0.,
                        t: 0.,
                        angle_factor: 0.,
                    })
                }
            }
        }

        pub fn check_walls_c2(&self, surfaces: &[&Surface]) -> Option<HotDogCollision> {
            let walls: Vec<&Triangle> = surfaces
                .iter()
                .filter_map(|s| match s {
                    Surface::Wall(w) => Some(w),
                    _ => None,
                })
                .collect();

            let diff = self.dst - self.src;

            let ray = C2Ray {
                p: self.src,
                d: diff.normalize(),
                t: diff.length(),
            };

            let mut nearest: Option<C2Raycast> = None;
            for tri in &walls {
                // OPTIONAL OPTIMIZATIONS
                // seems like these can actually be slower
                // test later with bigger maps and mapchunks
                // also test order, circle intersection is probably most aggressive
                // but probably most computation
                {
                    // // REMOVED: back-face check causes bugs on curved surfaces
                    // // check if pos is in the direction of the wall's normal
                    // let t1 = tri.verts()[0];
                    // let v = self.srcv3 - t1;
                    // if v.dot(tri.normal) < 0. {
                    //     continue;
                    // }

                    // // circle intersection with infinite plane
                    // let normal_xz = Vec2::new(tri.normal.x, tri.normal.z);
                    // let offset = normal_xz.dot(self.src) + tri.origin_offset;
                    // if offset.abs() >= self.radius + diff.length() {
                    //     continue;
                    // }
                }

                // REQUIRED RELEVANCE CHECKS

                // get line segment on the y plane
                // -- trims deadspace on wall edges that aren't vertical/horizontal
                // -- ie a triangle on the side of a ramp
                // -- but also skips triangles that don't cross src.y
                let Some((d1, d2)) = triangle_slice_at_y(&tri.verts, self.srcv3.y) else {
                    continue;
                };

                let cap = C2Capsule {
                    a: d1,
                    b: d2,
                    r: self.radius + self.skin,
                };
                if let Some(coll) = c2_rayto_capsule(ray, cap) {
                    // // skip wall if we're not heading at it
                    // if ray.d.dot(coll.n) >= 0. {
                    //     continue;
                    // }

                    match &nearest {
                        None => nearest = Some(coll),
                        Some(n) => {
                            if n.t > coll.t {
                                // last nearest is further away than coll
                                nearest = Some(coll)
                            }
                        }
                    }
                }
            }

            // if the current resting position reads as a collision,
            // update nearest, otherwise just return
            let nearest = match nearest {
                Some(n) => Some(n),
                None => {
                    let validate_ray = C2Ray {
                        p: self.dst,
                        d: Vec2::ONE.normalize(),
                        t: 0.,
                    };

                    let validate_r = self.radius + self.skin;
                    let mut coll = None;

                    for tri in &walls {
                        let Some((d1, d2)) = triangle_slice_at_y(&tri.verts, self.srcv3.y) else {
                            continue;
                        };

                        let cap = C2Capsule {
                            a: d1,
                            b: d2,
                            r: validate_r,
                        };

                        if c2_rayto_capsule(validate_ray, cap).is_some() {
                            let wall_normal_xz = Vec2::new(tri.normal.x, tri.normal.z).normalize();
                            // mimic a collision at exactly the destination
                            coll = Some(C2Raycast {
                                t: ray.t,
                                n: wall_normal_xz,
                            });
                            break;
                        }
                    }

                    coll
                }
            };

            let n = match nearest {
                Some(n) => n,
                None => return None,
            };

            // todo, what if when dst is colliding,
            // always go back to src, project, no step back

            // check if we need to step back from the collision point
            let dest_xz = self.step_back(ray, &n, &walls, None);
            // let dest_xz = c2_impact(ray, n.t) - self.skin * n.n;

            // project step onto wall plane (remove component going into wall)
            let step = self.dst - dest_xz;
            let push_out = step.project_onto(n.n);
            let mut next_move = step - push_out;

            // see if we've bounced off so much stuff that now we're heading backwards
            let next_dir = ((dest_xz + next_move) - dest_xz).normalize();
            if next_dir.dot(self.original_dir) < 0. {
                next_move = Vec2::ZERO
            }
            Some(HotDogCollision {
                dest_xz,
                next_move,
                next_move_len: next_move.length(),
                push_out,
                t: n.t,
                angle_factor: 0.,
            })
        }

        // todo, add last_wall, and check that one first before all the other walls
        pub fn step_back(
            &self,
            ray: C2Ray,
            collision: &C2Raycast,
            walls: &[&Triangle],
            d: Option<&mut RaylibDrawHandle>,
        ) -> Vec2 {
            // todo, remove, along with the pub for this fn
            if (self.dst - self.src).length() <= self.skin {
                if let Some(d) = d {
                    d.draw_circle(
                        self.src.x as i32,
                        self.src.y as i32,
                        self.radius,
                        Color::GREEN,
                    );
                }
                return self.src;
            }

            // end at the collision point, minus skin
            let start = ray.p;
            // let end = start + ray.d * (collision.t - self.skin);
            let end = start + ray.d * collision.t - self.skin * collision.n;

            // todo, check end first, then do a binary search from halfway between
            // or maybe even assume end is bad, and just start with the binary search
            //
            // 0 is end
            // 3 is 25% away from start
            // I'd originally set this to 9, but in practice, most iterations
            // return on either 0 or `iterations`.
            //
            // after running through the profiler, this is like 1.5% execution time
            // it really doesn't matter
            let iterations = 4;
            let locations: Vec<Vec2> = (0..iterations)
                .map(|i| end.lerp(start, i as f32 / iterations as f32))
                .collect();

            if let Some(d) = d {
                for dest_xz in &locations {
                    d.draw_circle(
                        dest_xz.x as i32,
                        dest_xz.y as i32,
                        self.radius,
                        Color::ORANGE.alpha(0.5),
                    );
                }
            }

            let validate_r = self.radius + self.skin;
            let mut fi = 0;
            for (i, dest_xz) in locations.iter().enumerate() {
                fi = i;

                let ray = C2Ray {
                    p: *dest_xz,
                    d: Vec2::ONE.normalize(),
                    t: 0.,
                };

                let mut collided = false;

                // check if we're inside a wall after move
                for tri in walls {
                    // if can't collide, skip
                    let Some((d1, d2)) = triangle_slice_at_y(&tri.verts, self.srcv3.y) else {
                        continue;
                    };

                    // if we're still too close
                    let cap = C2Capsule {
                        a: d1,
                        b: d2,
                        r: validate_r,
                    };

                    if c2_rayto_capsule(ray, cap).is_some() {
                        collided = true;
                        break;
                    }
                }

                if !collided {
                    // todo, remove
                    FI_LIST.with_borrow_mut(|hm| {
                        let f = hm.get(&fi).unwrap_or(&0);
                        hm.insert(fi, f + 1);
                    });
                    return *dest_xz;
                }
            }

            // check start, panic on failure
            // as start should always be known-good
            let dest_xz = start;

            let ray = C2Ray {
                p: dest_xz,
                d: Vec2::ONE.normalize(),
                t: 0.,
            };

            // check if we're inside a wall after move
            for tri in walls {
                // if can't collide, skip
                let Some((d1, d2)) = triangle_slice_at_y(&tri.verts, self.srcv3.y) else {
                    continue;
                };

                // if we're still too close
                let cap = C2Capsule {
                    a: d1,
                    b: d2,
                    r: validate_r,
                };

                if c2_rayto_capsule(ray, cap).is_some() {
                    HDLIST.with_borrow(|hotdogs| {
                        eprintln!("hotdogs: {:?}", hotdogs);
                    });
                    panic!("start is in wall");
                }
            }

            // todo, remove
            FI_LIST.with_borrow_mut(|hm| {
                let f = hm.get(&fi).unwrap_or(&0);
                hm.insert(fi, f + 1);
            });

            dest_xz
        }
    }

    #[derive(Debug)]
    pub struct HotDogCollision {
        pub dest_xz: Vec2,
        pub next_move: Vec2,
        pub next_move_len: f32,
        pub push_out: Vec2,
        pub t: f32,
        pub angle_factor: f32,
    }

    #[derive(Debug, Copy, Clone)]
    pub struct HotDogv2 {
        src: Vec2,
        src_y: f32,
        dst: Vec2,
        skin: f32,
        radius: f32,
        original_dir: Vec2,
    }

    #[derive(Debug)]
    pub struct HotDogCollisionv2 {
        pub dest_xz: Vec2,
        pub next_move: Vec2,
        pub next_move_len: f32,
    }

    impl HotDogv2 {
        pub fn new(
            srcv3: Vec3,
            dstv3: Vec3,
            radius: f32,
            check_height: f32,
            skin_factor: f32,
            original_dir: Vec3,
        ) -> Self {
            let src = Vec2::new(srcv3.x, srcv3.z);
            let dst = Vec2::new(dstv3.x, dstv3.z);
            let skin = radius * skin_factor;
            let hd = Self {
                src,
                src_y: srcv3.y + check_height,
                dst,
                radius,
                skin,
                original_dir: Vec2::new(original_dir.x, original_dir.z).normalize(),
            };

            hd
        }

        pub fn check_walls_c2(&self, surfaces: &[&Surface]) -> Option<HotDogCollisionv2> {
            let walls: Vec<&Triangle> = surfaces
                .iter()
                .filter_map(|s| match s {
                    Surface::Wall(w) => Some(w),
                    _ => None,
                })
                .collect();

            let diff = self.dst - self.src;

            let ray = C2Ray {
                p: self.src,
                d: diff.normalize(),
                t: diff.length(),
            };

            let mut nearest: Option<C2Raycast> = None;
            for tri in &walls {
                // REQUIRED RELEVANCE CHECKS

                // get line segment on the y plane
                // -- trims deadspace on wall edges that aren't vertical/horizontal
                // -- ie a triangle on the side of a ramp
                // -- but also skips triangles that don't cross src.y
                let Some((d1, d2)) = triangle_slice_at_y(&tri.verts, self.src_y) else {
                    continue;
                };

                let cap = C2Capsule {
                    a: d1,
                    b: d2,
                    r: self.radius + self.skin,
                };
                if let Some(coll) = c2_rayto_capsule(ray, cap) {
                    match &nearest {
                        None => nearest = Some(coll),
                        Some(n) => {
                            if n.t > coll.t {
                                // last nearest is further away than coll
                                nearest = Some(coll)
                            }
                        }
                    }
                }
            }

            // if the current resting position reads as a collision,
            // update nearest, otherwise just return
            let nearest = match nearest {
                Some(n) => Some(n),
                None => {
                    let validate_ray = C2Ray {
                        p: self.dst,
                        d: Vec2::ONE.normalize(),
                        t: 0.,
                    };

                    let validate_r = self.radius + self.skin;
                    let mut coll = None;

                    for tri in &walls {
                        let Some((d1, d2)) = triangle_slice_at_y(&tri.verts, self.src_y) else {
                            continue;
                        };

                        let cap = C2Capsule {
                            a: d1,
                            b: d2,
                            r: validate_r,
                        };

                        if c2_rayto_capsule(validate_ray, cap).is_some() {
                            let wall_normal_xz = Vec2::new(tri.normal.x, tri.normal.z).normalize();
                            // mimic a collision at exactly the destination
                            coll = Some(C2Raycast {
                                t: ray.t,
                                n: wall_normal_xz,
                            });
                            break;
                        }
                    }

                    coll
                }
            };

            let n = match nearest {
                Some(n) => n,
                None => return None,
            };

            // todo, what if when dst is colliding,
            // always go back to src, project, no step back

            // check if we need to step back from the collision point
            let dest_xz = self.step_back(ray, &n, &walls);

            // project step onto wall plane (remove component going into wall)
            let step = self.dst - dest_xz;
            let push_out = step.project_onto(n.n);
            let mut next_move = step - push_out;

            // see if we've bounced off so much stuff that now we're heading backwards
            let next_dir = ((dest_xz + next_move) - dest_xz).normalize();
            if next_dir.dot(self.original_dir) < 0. {
                next_move = Vec2::ZERO
            }
            Some(HotDogCollisionv2 {
                dest_xz,
                next_move,
                next_move_len: next_move.length(),
            })
        }

        // todo, add last_wall, and check that one first before all the other walls
        pub fn step_back(&self, ray: C2Ray, collision: &C2Raycast, walls: &[&Triangle]) -> Vec2 {
            // todo, remove, along with the pub for this fn
            if (self.dst - self.src).length() <= self.skin {
                return self.src;
            }

            // end at the collision point, minus skin
            let start = ray.p;
            // let end = start + ray.d * (collision.t - self.skin);
            let end = start + ray.d * collision.t - self.skin * collision.n;

            // todo, check end first, then do a binary search from halfway between
            // or maybe even assume end is bad, and just start with the binary search
            //
            // 0 is end
            // 3 is 25% away from start
            // I'd originally set this to 9, but in practice, most iterations
            // return on either 0 or `iterations`.
            //
            // after running through the profiler, this is like 1.5% execution time
            // it really doesn't matter
            let iterations = 4;
            let locations: Vec<Vec2> = (0..iterations)
                .map(|i| end.lerp(start, i as f32 / iterations as f32))
                .collect();

            let validate_r = self.radius + self.skin;
            for (_i, dest_xz) in locations.iter().enumerate() {

                let ray = C2Ray {
                    p: *dest_xz,
                    d: Vec2::ONE.normalize(),
                    t: 0.,
                };

                let mut collided = false;

                // check if we're inside a wall after move
                for tri in walls {
                    // if can't collide, skip
                    let Some((d1, d2)) = triangle_slice_at_y(&tri.verts, self.src_y) else {
                        continue;
                    };

                    // if we're still too close
                    let cap = C2Capsule {
                        a: d1,
                        b: d2,
                        r: validate_r,
                    };

                    if c2_rayto_capsule(ray, cap).is_some() {
                        collided = true;
                        break;
                    }
                }

                if !collided {
                    return *dest_xz;
                }
            }

            // check start, panic on failure
            // as start should always be known-good
            let dest_xz = start;

            let ray = C2Ray {
                p: dest_xz,
                d: Vec2::ONE.normalize(),
                t: 0.,
            };

            // check if we're inside a wall after move
            for tri in walls {
                // if can't collide, skip
                let Some((d1, d2)) = triangle_slice_at_y(&tri.verts, self.src_y) else {
                    continue;
                };

                // if we're still too close
                let cap = C2Capsule {
                    a: d1,
                    b: d2,
                    r: validate_r,
                };

                if c2_rayto_capsule(ray, cap).is_some() {
                    panic!("start is in wall");
                }
            }

            dest_xz
        }
    }

    // ANGLEFACTORNOTES
    // ========================================================================
    //
    // angle factor minimum limit is inversely proportional to the skin factor
    // looks like
    // angle = .0000038 / skin_factor
    //
    // skin || angle
    // ====================
    // .05  -> .000076
    // .04  -> .000095
    // .03  -> .00012
    // .02  -> .00019
    // .01  -> .00038
    // .005 -> .00076
    // .001 -> .0039
    //
    // I don't know where this 38 value comes from,
    // apparently it's the answer to the universe
    //
    // f32::EPSILON * 32. ???

    // eprintln!("angle_factor: {}", angle_factor);

    // // magic number time,
    // // with skin = radius * 0.005,
    // // we don't start getting t == 0.
    // // until angle factor gets below ~ 0.00764
    // // (actually, like 7633, but erring on side of caution)
    // === starts while decreasing ===
    // angle_factor: 0.000763467
    // angle_factor: 0.00076332496
    // angle_factor: 0.99999976
    // HERE HERE HERE
    // angle_factor: 0.0007631812
    // angle_factor: 0.0007631812
    // === ends while increasing ===
    // angle_factor: 0.00076292217
    // angle_factor: 0.9999997
    // HERE HERE HERE
    // angle_factor: 0.0007631184
    // angle_factor: 0.99999976
    // HERE HERE HERE
    // angle_factor: 0.00076331454
    // angle_factor: 0.0007633148
    // angle_factor: 0.0007635089
    // === repeats ===
    // angle_factor: 0.0007635021
    // angle_factor: 0.0007633607
    // angle_factor: 0.9999997
    // HERE HERE HERE
    // angle_factor: 0.0007632193
    // angle_factor: 0.0007632193

    // // magic numbers for skin = radius * 0.001
    // // sub 0.0039
    // angle_factor: 0.0037163915
    // angle_factor: 0.9999931
    // HERE HERE HERE
    // angle_factor: 0.0038610834
    // angle_factor: 0.0038610834
    // angle_factor: 0.004010877
    // ========================================================================

    /*  CUTE HEADERS -- github.com/RandyGaul/cute_headers
        ------------------------------------------------------------------------------
        ALTERNATIVE B - Public Domain (www.unlicense.org)
        This is free and unencumbered software released into the public domain.
        Anyone is free to copy, modify, publish, use, compile, sell, or distribute this
        software, either in source code form or as a compiled binary, for any purpose,
        commercial or non-commercial, and by any means.
        In jurisdictions that recognize copyright laws, the author or authors of this
        software dedicate any and all copyright interest in the software to the public
        domain. We make this dedication for the benefit of the public at large and to
        the detriment of our heirs and successors. We intend this dedication to be an
        overt act of relinquishment in perpetuity of all present and future rights to
        this software under copyright law.
        THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
        IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
        FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
        AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
        ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
        WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
        ------------------------------------------------------------------------------
    */

    // typedef struct c2Capsule
    // {
    // 	c2v a;
    // 	c2v b;
    // 	float r;
    // } c2Capsule;
    // a capsule is defined as a line segment (from a to b) and radius r
    #[derive(Debug)]
    struct C2Capsule {
        a: Vec2,
        b: Vec2,
        r: f32,
    }

    // typedef struct c2Ray
    // {
    // 	c2v p;   // position
    // 	c2v d;   // direction (normalized)
    // 	float t; // distance along d from position p to find endpoint of ray
    // } c2Ray;
    /// IMPORTANT:
    /// Many algorithms in this file are sensitive to the magnitude of the
    /// ray direction (c2Ray::d). It is highly recommended to normalize the
    /// ray direction and use t to specify a distance. Please see this link
    /// for an in-depth explanation: https://github.com/RandyGaul/cute_headers/issues/30
    #[derive(Copy, Clone)]
    pub struct C2Ray {
        pub p: Vec2,
        pub d: Vec2,
        pub t: f32,
    }

    // todo, reduce publicness ^v

    // typedef struct c2Raycast
    // {
    // 	float t; // time of impact
    // 	c2v n;   // normal of surface at impact (unit length)
    // } c2Raycast;
    #[derive(Debug)]
    pub struct C2Raycast {
        pub t: f32,
        pub n: Vec2,
    }

    // #define c2Impact(ray, t) c2Add(ray.p, c2Mulvs(ray.d, t))
    // position of impact p = ray.p + ray.d * raycast.t
    fn c2_impact(ray: C2Ray, t: f32) -> Vec2 {
        ray.p + ray.d * t
    }

    // int c2RaytoCapsule(c2Ray A, c2Capsule B, c2Raycast* out)
    // {
    // 	c2m M;
    // 	M.y = c2Norm(c2Sub(B.b, B.a));
    // 	M.x = c2CCW90(M.y);

    // 	// rotate capsule to origin, along Y axis
    // 	// rotate the ray same way
    // 	c2v cap_n = c2Sub(B.b, B.a);
    // 	c2v yBb = c2MulmvT(M, cap_n);
    // 	c2v yAp = c2MulmvT(M, c2Sub(A.p, B.a));
    // 	c2v yAd = c2MulmvT(M, A.d);
    // 	c2v yAe = c2Add(yAp, c2Mulvs(yAd, A.t));

    // 	c2AABB capsule_bb;
    // 	capsule_bb.min = c2V(-B.r, 0);
    // 	capsule_bb.max = c2V(B.r, yBb.y);

    // 	out->n = c2Norm(cap_n);
    // 	out->t = 0;

    // 	// check and see if ray starts within the capsule
    // 	if (c2AABBtoPoint(capsule_bb, yAp)) {
    // 		return 1;
    // 	} else {
    // 		c2Circle capsule_a;
    // 		c2Circle capsule_b;
    // 		capsule_a.p = B.a;
    // 		capsule_a.r = B.r;
    // 		capsule_b.p = B.b;
    // 		capsule_b.r = B.r;

    // 		if (c2CircleToPoint(capsule_a, A.p)) {
    // 			return 1;
    // 		} else if (c2CircleToPoint(capsule_b, A.p)) {
    // 			return 1;
    // 		}
    // 	}

    // 	if (yAe.x * yAp.x < 0 || c2Min(c2Abs(yAe.x), c2Abs(yAp.x)) < B.r)
    // 	{
    // 		c2Circle Ca, Cb;
    // 		Ca.p = B.a;
    // 		Ca.r = B.r;
    // 		Cb.p = B.b;
    // 		Cb.r = B.r;

    // 		// ray starts inside capsule prism -- must hit one of the semi-circles
    // 		if (c2Abs(yAp.x) < B.r) {
    // 			if (yAp.y < 0) return c2RaytoCircle(A, Ca, out);
    // 			else return c2RaytoCircle(A, Cb, out);
    // 		}

    // 		// hit the capsule prism
    // 		else
    // 		{
    // 			float c = yAp.x > 0 ? B.r : -B.r;
    // 			float d = (yAe.x - yAp.x);
    // 			float t = (c - yAp.x) / d;
    // 			float y = yAp.y + (yAe.y - yAp.y) * t;
    // 			if (y <= 0) return c2RaytoCircle(A, Ca, out);
    // 			if (y >= yBb.y) return c2RaytoCircle(A, Cb, out);
    // 			else {
    // 				out->n = c > 0 ? M.x : c2Skew(M.y);
    // 				out->t = t * A.t;
    // 				return 1;
    // 			}
    // 		}
    // 	}

    // 	return 0;
    // }

    // typedef struct c2m
    // {
    // 	c2v x;
    // 	c2v y;
    // } c2m;
    // 2d rotation matrix
    #[derive(Clone, Copy)]
    struct C2M {
        x: Vec2,
        y: Vec2,
    }

    // typedef struct c2AABB
    // {
    // 	c2v min;
    // 	c2v max;
    // } c2AABB;
    struct C2AABB {
        min: Vec2,
        max: Vec2,
    }

    // c2v c2MulmvT(c2m a, c2v b) { c2v c; c.x = a.x.x * b.x + a.x.y * b.y; c.y = a.y.x * b.x + a.y.y * b.y; return c; }
    fn c2_mulmv_t(a: C2M, b: Vec2) -> Vec2 {
        let mut c = Vec2::ZERO;
        c.x = a.x.x * b.x + a.x.y * b.y;
        c.y = a.y.x * b.x + a.y.y * b.y;
        return c;
    }

    // int c2AABBtoPoint(c2AABB A, c2v B)
    // {
    // 	int d0 = B.x < A.min.x;
    // 	int d1 = B.y < A.min.y;
    // 	int d2 = B.x > A.max.x;
    // 	int d3 = B.y > A.max.y;
    // 	return !(d0 | d1 | d2 | d3);
    // }
    fn c2_aabbto_point(a: C2AABB, b: Vec2) -> bool {
        let d0 = b.x < a.min.x;
        let d1 = b.y < a.min.y;
        let d2 = b.x > a.max.x;
        let d3 = b.y > a.max.y;
        return !(d0 | d1 | d2 | d3);
    }

    // typedef struct c2Circle
    // {
    // 	c2v p;
    // 	float r;
    // } c2Circle;
    struct C2Circle {
        p: Vec2,
        r: f32,
    }

    // int c2CircleToPoint(c2Circle A, c2v B)
    // {
    // 	c2v n = c2Sub(A.p, B);
    // 	float d2 = c2Dot(n, n);
    // 	return d2 < A.r * A.r;
    // }
    fn c2_circle_to_point(a: C2Circle, b: Vec2) -> bool {
        let n = a.p - b;
        let d2 = n.dot(n);
        return d2 < a.r * a.r;
    }

    // int c2RaytoCircle(c2Ray A, c2Circle B, c2Raycast* out)
    // {
    // 	c2v p = B.p;
    // 	c2v m = c2Sub(A.p, p);
    // 	float c = c2Dot(m, m) - B.r * B.r;
    // 	float b = c2Dot(m, A.d);
    // 	float disc = b * b - c;
    // 	if (disc < 0) return 0;

    // 	float t = -b - c2Sqrt(disc);
    // 	if (t >= 0 && t <= A.t)
    // 	{
    // 		out->t = t;
    // 		c2v impact = c2Impact(A, t);
    // 		out->n = c2Norm(c2Sub(impact, p));
    // 		return 1;
    // 	}
    // 	return 0;
    // }
    fn c2_rayto_circle(a: C2Ray, b: C2Circle) -> Option<C2Raycast> {
        let p = b.p;
        let m = a.p - p;
        let c = m.dot(m) - b.r * b.r;
        let b = m.dot(a.d);
        let disc = b * b - c;
        if disc < 0. {
            return None;
        }

        let t = -b - disc.sqrt();
        if t >= 0. && t <= a.t {
            Some(C2Raycast {
                t: t,
                // TODO, check if we need to (n * -1.) if it's above 0.
                n: (c2_impact(a, t) - p).normalize(),
            })
        } else {
            None
        }
    }

    // c2v c2Skew(c2v a) { c2v b; b.x = -a.y; b.y = a.x; return b; }
    fn c2_skew(a: Vec2) -> Vec2 {
        Vec2::new(-a.y, a.x)
    }

    fn c2_rayto_capsule(a: C2Ray, b: C2Capsule) -> Option<C2Raycast> {
        let my = (b.b - b.a).normalize();
        let m = C2M {
            y: my,
            x: Vec2::new(-my.y, my.x),
        };

        // rotate capsule to origin, along Y axis
        // rotate the ray same way
        let cap_n = b.b - b.a;
        let y_bb = c2_mulmv_t(m, cap_n);
        let y_ap = c2_mulmv_t(m, a.p - b.a);
        let y_ad = c2_mulmv_t(m, a.d);
        let y_ae = y_ap + y_ad * a.t;

        let capsule_bb = C2AABB {
            min: Vec2::new(-b.r, 0.),
            max: Vec2::new(b.r, y_bb.y),
        };

        let out = C2Raycast {
            n: cap_n.normalize(),
            t: 0.,
        };

        // check and see if ray starts within the capsule
        if c2_aabbto_point(capsule_bb, y_ap) {
            return Some(out);
        } else {
            let capsule_a = C2Circle { p: b.a, r: b.r };
            let capsule_b = C2Circle { p: b.b, r: b.r };

            if c2_circle_to_point(capsule_a, a.p) {
                return Some(out);
            } else if c2_circle_to_point(capsule_b, a.p) {
                return Some(out);
            }
        }

        if y_ae.x * y_ap.x < 0. || y_ae.x.abs().min(y_ap.x.abs()) < b.r {
            let ca = C2Circle { p: b.a, r: b.r };
            let cb = C2Circle { p: b.b, r: b.r };

            // ray starts inside capsule prism -- must hit one of the semi-circles
            if y_ap.x.abs() < b.r {
                if y_ap.y < 0. {
                    return c2_rayto_circle(a, ca);
                } else {
                    return c2_rayto_circle(a, cb);
                }
            }
            // hit the capsule prism
            else {
                let c = if y_ap.x > 0. { b.r } else { -b.r };
                let d = y_ae.x - y_ap.x;
                let t = (c - y_ap.x) / d;
                let y = y_ap.y + (y_ae.y - y_ap.y) * t;
                if y <= 0. {
                    return c2_rayto_circle(a, ca);
                }
                if y >= y_bb.y {
                    return c2_rayto_circle(a, cb);
                } else {
                    return Some(C2Raycast {
                        // TODO, check if we need to (n * -1.) if it's above 0.
                        n: if c > 0. { m.x } else { c2_skew(m.y) },
                        t: t * a.t,
                    });
                }
            }
        }

        None
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
}

pub mod real {

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

    pub fn push_out_walls_2(
        pos: Vec3,
        check_height: f32,
        radius: f32,
        surfaces: &Vec<Surface>,
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
        surfaces: &Vec<Surface>,
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
            Some(surf) => Some((surf, best_y)),
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
        surfaces: &Vec<Surface>,
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

                    // // OLD METHOD
                    // let y = solve_plane_y(tri.normal, tri.origin_offset, posv2.x, posv2.y).clamp(tri.min_y, tri.max_y);
                    //
                    // this keeps toes outside the slide
                    // we could kill this whole chunk and
                    // go with the above, but then toes will
                    // clip the inclines, AND we get horrible
                    // bonk at the top
                    let uphill = Vec2::new(-tri.normal.x, -tri.normal.z);
                    let uphill_len = uphill.length();
                    let sample = if uphill_len > f32::EPSILON {
                        posv2 + uphill * (radius / uphill_len)
                    } else {
                        posv2
                    };
                    let y = solve_plane_y(tri.normal, tri.origin_offset, sample.x, sample.y).clamp(tri.min_y, tri.max_y);

                    // raycast down
                    if flattened_point_inside_flattened_triangle(
                        pos,
                        tri.verts[0],
                        tri.verts[1],
                        tri.verts[2],
                    ) {
                        break 'ray_c y;
                    }

                    // also check down
                    let p1v2 = Vec2::new(tri.verts[0].x, tri.verts[0].z);
                    let p2v2 = Vec2::new(tri.verts[1].x, tri.verts[1].z);
                    let p3v2 = Vec2::new(tri.verts[2].x, tri.verts[2].z);

                    let cp1 = closest_point_on_segment_v2(posv2, p1v2, p2v2);
                    let cp2 = closest_point_on_segment_v2(posv2, p2v2, p3v2);
                    let cp3 = closest_point_on_segment_v2(posv2, p3v2, p1v2);
                    let cp1d = cp1.distance(posv2);
                    let cp2d = cp2.distance(posv2);
                    let cp3d = cp3.distance(posv2);
                    let threshold = radius + radius * SKIN_FACTOR * 2.;
                    if cp1d > threshold && cp2d > threshold && cp3d > threshold {
                        // neither inside flattened triangle, nor within threshold
                        continue 'sloop;
                    }

                    // we have to clamp this one, because we don't want an invisible upward/downward
                    // extension of a sloped floor
                    y
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
            Some(surf) => Some((surf, best_y)),
            None => None,
        }
    }

}