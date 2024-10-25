// Improved Collision detection and Response
// Kasper Fauerby
// https://www.peroxide.dk/papers/collision/collision.pdf
// kasper@peroxide.dk

use raymath::{
    matrix_translate, vector3_add, vector3_cross_product, vector3_distance, vector3_dot_product,
    vector3_length, vector3_length_sqr, vector3_multiply, vector3_negate, vector3_normalize,
    vector3_scale, vector3_subtract, Vector3,
};

use crate::{
    e_player::{MAX_COLLISION_DIST, MAX_SLOPE},
    g_instance::{self, get_decor_instances},
    math::{closest_point_to_triangle, mesh_tranform, vec3_face_normal},
    render,
};

pub struct Plane {
    equation: [f32; 4],
    origin: Vector3,
    normal: Vector3,
}

impl Plane {
    // Constructor from origin and normal
    pub fn new_from_normal(origin: Vector3, normal: Vector3) -> Self {
        let equation = [
            normal.x,
            normal.y,
            normal.z,
            -(normal.x * origin.x + normal.y * origin.y + normal.z * origin.z),
        ];
        Self {
            equation,
            origin,
            normal,
        }
    }

    // Constructor from three points (defining a triangle)
    pub fn new_from_points(p1: Vector3, p2: Vector3, p3: Vector3) -> Self {
        let normal = vector3_normalize(vector3_cross_product(
            vector3_subtract(p2, p1),
            vector3_subtract(p3, p1),
        ));
        let equation = [
            normal.x,
            normal.y,
            normal.z,
            -(normal.x * p1.x + normal.y * p1.y + normal.z * p1.z),
        ];
        Self {
            equation,
            origin: p1,
            normal,
        }
    }

    // Check if the plane is front-facing to the direction Vector3
    pub fn is_front_facing_to(&self, direction: &Vector3) -> bool {
        let dot = vector3_dot_product(self.normal, *direction);
        dot <= 0.0
    }

    // Get the signed distance from a point to the plane
    pub fn signed_distance_to(&self, point: &Vector3) -> f64 {
        (vector3_dot_product(*point, self.normal) as f64) + self.equation[3] as f64
    }
}

fn check_point_in_triangle(point: &Vector3, pa: &Vector3, pb: &Vector3, pc: &Vector3) -> bool {
    let e10 = vector3_subtract(*pb, *pa);
    let e20 = vector3_subtract(*pc, *pa);

    let a = vector3_dot_product(e10, e10);
    let b = vector3_dot_product(e10, e20);
    let c = vector3_dot_product(e20, e20);
    let ac_bb = (a * c) - (b * b);

    let vp = Vector3::new(point.x - pa.x, point.y - pa.y, point.z - pa.z);
    let d = vector3_dot_product(vp, e10);
    let e = vector3_dot_product(vp, e20);

    let x = (d * c) - (e * b);
    let y = (e * a) - (d * b);
    let z = x + y - ac_bb;

    let in_x = unsafe { std::mem::transmute::<f32, u32>(x) };
    let in_y = unsafe { std::mem::transmute::<f32, u32>(y) };
    let in_z = unsafe { std::mem::transmute::<f32, u32>(z) };

    // x >= 0.0 && y >= 0.0 && z <= 0.0 ???
    ((in_z & !(in_x | in_y)) & 0x80000000) != 0
}

fn get_lowest_root(a: f32, b: f32, c: f32, max_r: f32, root: &mut f32) -> bool {
    // Check if a solution exists
    let determinant = b * b - 4.0 * a * c;

    // If determinant is negative, it means no solutions.
    if determinant < 0.0 {
        return false;
    }

    // Calculate the two roots (if determinant == 0 then x1 == x2)
    let sqrt_d = determinant.sqrt();
    let r1 = (-b - sqrt_d) / (2.0 * a);
    let r2 = (-b + sqrt_d) / (2.0 * a);

    // Sort so r1 <= r2
    let (r1, r2) = if r1 > r2 { (r2, r1) } else { (r1, r2) };

    // Get lowest root:
    if r1 > 0.0 && r1 < max_r {
        *root = r1;
        return true;
    }

    // Check if we want r2 instead (this can happen if r1 < 0)
    if r2 > 0.0 && r2 < max_r {
        *root = r2;
        return true;
    }

    // No (valid) solutions
    false
}

pub struct CollisionPacket {
    pub e_radius: Vector3, // Ellipsoid radius
    // Information about the move being requested (in R3)
    pub r3_velocity: Vector3, // R3 velocity
    pub r3_position: Vector3, // R3 position
    // Information about the move being requested (in eSpace)
    pub velocity: Vector3,            // Velocity in eSpace
    pub normalized_velocity: Vector3, // Normalized velocity
    pub base_point: Vector3,          // Base point
    // Hit information
    pub found_collision: bool,       // Whether a collision was found
    pub nearest_distance: f64,       // Nearest distance to a collision
    pub intersection_point: Vector3, // Point of intersection
}

impl CollisionPacket {
    pub fn new(
        e_radius: Vector3,
        r3_velocity: Vector3,
        r3_position: Vector3,
        velocity: Vector3,
        normalized_velocity: Vector3,
        base_point: Vector3,
    ) -> Self {
        Self {
            e_radius,
            r3_velocity,
            r3_position,
            velocity,
            normalized_velocity,
            base_point,
            found_collision: false,     // Default initialization
            nearest_distance: f64::MAX, // Start with a large value
            intersection_point: Vector3::zero(),
        }
    }
}

fn check_triangle(col_package: &mut CollisionPacket, p1: Vector3, p2: Vector3, p3: Vector3) {
    // Create the plane containing this triangle
    let triangle_plane = Plane::new_from_points(p1, p2, p3);

    // Check if the triangle is front-facing to the velocity Vector3
    if triangle_plane.is_front_facing_to(&col_package.normalized_velocity) {
        // Variables to hold interval of plane intersection
        let mut t0;
        let mut t1;
        let mut embedded_in_plane = false;

        // Calculate the signed distance from the sphere position to the triangle plane
        let signed_dist_to_triangle_plane =
            triangle_plane.signed_distance_to(&col_package.base_point) as f32;
        let normal_dot_velocity = vector3_dot_product(triangle_plane.normal, col_package.velocity);

        // Check if the sphere is traveling parallel to the plane
        if normal_dot_velocity == 0.0 {
            if signed_dist_to_triangle_plane.abs() >= 1.0 {
                // Sphere is not embedded in the plane; no collision possible
                return;
            } else {
                // Sphere is embedded in the plane; it intersects in the whole range [0..1]
                embedded_in_plane = true;
                t0 = 0.0;
                t1 = 1.0;
            }
        } else {
            // Calculate the intersection interval
            t0 = (-1.0 - signed_dist_to_triangle_plane) / normal_dot_velocity;
            t1 = (1.0 - signed_dist_to_triangle_plane) / normal_dot_velocity;

            // Swap if necessary to ensure t0 < t1
            if t0 > t1 {
                std::mem::swap(&mut t0, &mut t1);
            }

            // Check that at least one result is within the range [0, 1]
            if t0 > 1.0 || t1 < 0.0 {
                // Both t values are outside the range, so no collision is possible
                return;
            }

            // Clamp to [0, 1]
            t0 = t0.clamp(0.0, 1.0);
            t1 = t1.clamp(0.0, 1.0);
        }

        // At this point, we have two time values t0 and t1 between which the sphere intersects with the plane
        let mut collision_point = Vector3::zero();
        let mut found_collision = false;
        let mut t = 1.0;

        // Check for the simple case: collision inside the triangle
        if !embedded_in_plane {
            let plane_intersection_point = vector3_add(
                vector3_subtract(col_package.base_point, triangle_plane.normal),
                vector3_scale(col_package.velocity, t0),
            );
            if check_point_in_triangle(&plane_intersection_point, &p1, &p2, &p3) {
                found_collision = true;
                t = t0;
                collision_point = plane_intersection_point;
            }
        }

        // If no collision found, sweep against points and edges of the triangle
        if !found_collision {
            let velocity = col_package.velocity;
            let base = col_package.base_point;
            let velocity_squared_length = vector3_length_sqr(velocity);

            let mut a;
            let mut b;
            let mut c;
            let mut new_t = 0.;

            // Check against points:
            a = velocity_squared_length;

            // P1
            b = 2.0 * vector3_dot_product(velocity, vector3_subtract(base, p1));
            c = vector3_length_sqr(vector3_subtract(p1, base)) - 1.0;
            if get_lowest_root(a, b, c, t, &mut new_t) {
                t = new_t;
                found_collision = true;
                collision_point = p1;
            }

            // P2
            b = 2.0 * vector3_dot_product(velocity, vector3_subtract(base, p2));
            c = vector3_length_sqr(vector3_subtract(p2, base)) - 1.0;
            if get_lowest_root(a, b, c, t, &mut new_t) {
                t = new_t;
                found_collision = true;
                collision_point = p2;
            }

            // P3
            b = 2.0 * vector3_dot_product(velocity, vector3_subtract(base, p3));
            c = vector3_length_sqr(vector3_subtract(p3, base)) - 1.0;
            if get_lowest_root(a, b, c, t, &mut new_t) {
                t = new_t;
                found_collision = true;
                collision_point = p3;
            }

            // Check against edges:
            // P1 -> P2
            let edge = vector3_subtract(p2, p1);
            let base_to_vertex = vector3_subtract(p1, base);
            let edge_squared_length = vector3_length_sqr(edge);
            let edge_dot_velocity = vector3_dot_product(edge, velocity);
            let edge_dot_base_to_vertex = vector3_dot_product(edge, base_to_vertex);

            a = edge_squared_length * -velocity_squared_length
                + edge_dot_velocity * edge_dot_velocity;
            b = edge_squared_length * (2.0 * vector3_dot_product(velocity, base_to_vertex))
                - 2.0 * edge_dot_velocity * edge_dot_base_to_vertex;
            c = edge_squared_length * (1.0 - vector3_length_sqr(base_to_vertex))
                + edge_dot_base_to_vertex * edge_dot_base_to_vertex;

            if get_lowest_root(a, b, c, t, &mut new_t) {
                let f = (edge_dot_velocity * new_t - edge_dot_base_to_vertex) / edge_squared_length;
                if f >= 0.0 && f <= 1.0 {
                    t = new_t;
                    found_collision = true;
                    collision_point = vector3_add(p1, vector3_scale(edge, f));
                }
            }

            // P2 -> P3
            let edge = vector3_subtract(p3, p2);
            let base_to_vertex = vector3_subtract(p2, base);
            let edge_squared_length = vector3_length_sqr(edge);
            let edge_dot_velocity = vector3_dot_product(edge, velocity);
            let edge_dot_base_to_vertex = vector3_dot_product(edge, base_to_vertex);

            a = edge_squared_length * -velocity_squared_length
                + edge_dot_velocity * edge_dot_velocity;
            b = edge_squared_length * (2.0 * vector3_dot_product(velocity, base_to_vertex))
                - 2.0 * edge_dot_velocity * edge_dot_base_to_vertex;
            c = edge_squared_length * (1.0 - vector3_length_sqr(base_to_vertex))
                + edge_dot_base_to_vertex * edge_dot_base_to_vertex;

            if get_lowest_root(a, b, c, t, &mut new_t) {
                let f = (edge_dot_velocity * new_t - edge_dot_base_to_vertex) / edge_squared_length;
                if f >= 0.0 && f <= 1.0 {
                    t = new_t;
                    found_collision = true;
                    collision_point = vector3_add(p2, vector3_scale(edge, f));
                }
            }

            // P3 -> P1
            let edge = vector3_subtract(p1, p3);
            let base_to_vertex = vector3_subtract(p3, base);
            let edge_squared_length = vector3_length_sqr(edge);
            let edge_dot_velocity = vector3_dot_product(edge, velocity);
            let edge_dot_base_to_vertex = vector3_dot_product(edge, base_to_vertex);

            a = edge_squared_length * -velocity_squared_length
                + edge_dot_velocity * edge_dot_velocity;
            b = edge_squared_length * (2.0 * vector3_dot_product(velocity, base_to_vertex))
                - 2.0 * edge_dot_velocity * edge_dot_base_to_vertex;
            c = edge_squared_length * (1.0 - vector3_length_sqr(base_to_vertex))
                + edge_dot_base_to_vertex * edge_dot_base_to_vertex;

            if get_lowest_root(a, b, c, t, &mut new_t) {
                let f = (edge_dot_velocity * new_t - edge_dot_base_to_vertex) / edge_squared_length;
                if f >= 0.0 && f <= 1.0 {
                    t = new_t;
                    found_collision = true;
                    collision_point = vector3_add(p3, vector3_scale(edge, f));
                }
            }
        }

        // Set result
        if found_collision {
            let dist_to_collision = t * vector3_length(col_package.velocity);
            if !col_package.found_collision
                || dist_to_collision < col_package.nearest_distance as f32
            {
                col_package.nearest_distance = dist_to_collision as f64;
                col_package.intersection_point = collision_point;
                col_package.found_collision = true;
            }
        }
    }
}

pub struct CharacterEntity {
    position: Vector3,
    collision_packet: CollisionPacket,
    collision_recursion_depth: usize,
    units_per_meter: f32,
}

impl CharacterEntity {
    pub fn new(
        position: Vector3,
        e_radius: Vector3,
        collision_recursion_depth: usize,
        units_per_meter: f32,
    ) -> Self {
        Self {
            position,
            collision_packet: CollisionPacket::new(
                e_radius,
                Vector3::zero(),
                Vector3::zero(),
                Vector3::zero(),
                Vector3::zero(),
                Vector3::zero(),
            ),
            collision_recursion_depth,
            units_per_meter,
        }
    }

    pub fn collide_and_slide(&mut self, vel: Vector3, gravity: Vector3) -> Vector3 {
        // Do collision detection:
        self.collision_packet.r3_position = self.position;
        self.collision_packet.r3_velocity = vel;

        // Calculate position and velocity in eSpace
        let e_space_position = Vector3::new(
            self.collision_packet.r3_position.x / self.collision_packet.e_radius.x,
            self.collision_packet.r3_position.y / self.collision_packet.e_radius.y,
            self.collision_packet.r3_position.z / self.collision_packet.e_radius.z,
        );
        let e_space_velocity = Vector3::new(
            self.collision_packet.r3_velocity.x / self.collision_packet.e_radius.x,
            self.collision_packet.r3_velocity.y / self.collision_packet.e_radius.y,
            self.collision_packet.r3_velocity.z / self.collision_packet.e_radius.z,
        );

        // Iterate until we have our final position.
        self.collision_recursion_depth = 0;
        let mut final_position = self.collide_with_world(e_space_position, e_space_velocity);
        eprintln!("final_position1: {:?}", final_position);

        // Add gravity pull:
        // To remove gravity uncomment from here .....

        // // Set the new R3 position (convert back from eSpace to R3)
        // self.collision_packet.r3_position =
        //     vector3_multiply(final_position, self.collision_packet.e_radius);
        // self.collision_packet.r3_velocity = gravity;

        // let e_space_velocity = Vector3::new(
        //     gravity.x / self.collision_packet.e_radius.x,
        //     gravity.y / self.collision_packet.e_radius.y,
        //     gravity.z / self.collision_packet.e_radius.z,
        // );
        // self.collision_recursion_depth = 0;
        // final_position = self.collide_with_world(final_position, e_space_velocity);

        // ... to here
        // Convert final result back to R3:
        final_position = vector3_multiply(final_position, self.collision_packet.e_radius);
        eprintln!("final_position2: {:?}", self.collision_packet.e_radius);

        // Move the entity (application specific function)
        final_position
    }

    fn check_collision(&mut self) {
        // self.packet: &mut CollisionPacket
        // todo, don't have to pass in packet
        // check_triangle()
        let decs = get_decor_instances().unwrap();

        for dec in decs {
            let mesh = dec.get_mesh();
            let mat = dec.get_matrix();
            let mesh = mesh_tranform(mesh, mat);

            for tri in mesh {
                check_triangle(&mut self.collision_packet, tri[0], tri[1], tri[2]);
            }
        }
    }

    fn check_collision_2(&mut self) {
        // self.packet: &mut CollisionPacket
        // todo, don't have to pass in packet
        // check_triangle()

        let mut wall_collisions = vec![];
        let decs = get_decor_instances().unwrap();

        for dec in decs {
            let mesh = dec.get_mesh();
            let mat = dec.get_matrix();
            let mesh = mesh_tranform(mesh, mat);

            for (tri, normal) in mesh.into_iter().map(|tri| {
                (
                    tri,
                    vector3_negate(vec3_face_normal(tri[0], tri[1], tri[2])),
                )
            }) {
                // if !(-normal.y < MAX_SLOPE) {
                //     continue;
                // }

                if (vector3_distance(self.position, tri[0]) > MAX_COLLISION_DIST)
                    && (vector3_distance(self.position, tri[1]) > MAX_COLLISION_DIST)
                    && (vector3_distance(self.position, tri[2]) > MAX_COLLISION_DIST)
                {
                    continue;
                }

                let closest = closest_point_to_triangle(tri, self.position);

                if cfg!(debug_assertions) {
                    let re = g_instance::ref_ent_from_str("icosphere").unwrap();
                    let mat = matrix_translate(closest.x, closest.y, closest.z);

                    let dc = render::DrawCall {
                        matrix: mat,
                        texture: re.texture_handle as u32,
                        f1: re.frame_handles[0] as i32,
                        f2: re.frame_handles[0] as i32,
                        mix: 0.0,
                        num_verts: re.num_verts,
                        glow: Some(Vector3::new(0.7, 0.7, 0.7)),
                    };
                    render::draw(dc).unwrap();

                    let mat = matrix_translate(
                        self.position.x + self.collision_packet.e_radius.x / 2.,
                        self.position.y,
                        self.position.z,
                    );
                    let dc = render::DrawCall {
                        matrix: mat,
                        texture: re.texture_handle as u32,
                        f1: re.frame_handles[0] as i32,
                        f2: re.frame_handles[0] as i32,
                        mix: 0.0,
                        num_verts: re.num_verts,
                        glow: Some(Vector3::new(0., 0., 0.7)),
                    };
                    render::draw(dc).unwrap();
                }
                // collides height-wise
                if vector3_distance(self.position, closest) <= self.collision_packet.e_radius.x / 2.
                {
                    // collides spherical
                    // if vector3_distance(*tmp_pos, closest) <= width / 2.
                    // {
                    // store the triangle we're colliding with,
                    // and store the y height we're going to collide at
                    // wall_collisions.push((tri, closest.y));
                    wall_collisions.push((closest, vector3_negate(normal)));
                }
            }
        }

        let coll_count = wall_collisions.len();

        // todo -- perf
        // 1, step back perfectly, using the closest collision
        // 2, exit early out of the loop above on first collision, and don't collect any walls
        // return to prior step's position
        if coll_count == 0 {
            return;
        }
        //  else {
        //     position.y = tmp_pos.y;
        //     continue;
        // }

        if coll_count > 0 {
            eprintln!("coll_count: {coll_count}");
            // eprintln!("colls[{coll_count}]: {:?}", wall_collisions);
        }

        // find closest intersection point to the center of the player's hitbox
        let mut intersector = None;
        let mut closest_distance = f32::INFINITY;
        for (coll, norm) in wall_collisions {
            let dist = vector3_distance(self.position, coll);
            // if closer, or equally close but better alignment
            if dist < closest_distance {
                intersector = Some((coll, norm));
                closest_distance = dist;
            }
        }

        match intersector {
            None => panic!("at least 1 collision, but no intersector"),
            Some((coll, _)) => {
                let dist_to_collision = vector3_distance(self.position, coll);
                if !self.collision_packet.found_collision
                    || dist_to_collision < self.collision_packet.nearest_distance as f32
                {
                    self.collision_packet.nearest_distance = dist_to_collision as f64;
                    self.collision_packet.intersection_point = coll;
                    self.collision_packet.found_collision = true;
                }
            }
        }
    }

    fn collide_with_world(&mut self, pos: Vector3, vel: Vector3) -> Vector3 {
        // All hard-coded distances in this function are
        // scaled to fit the setting above.
        let unit_scale = self.units_per_meter / 100.0;
        let very_close_distance = 0.005 * unit_scale;

        // Do we need to worry?
        if self.collision_recursion_depth > 5 {
            return pos;
        }

        // Ok, we need to worry:
        self.collision_packet.velocity = vel;
        self.collision_packet.normalized_velocity = vector3_normalize(vel);
        self.collision_packet.base_point = pos;
        self.collision_packet.found_collision = false;

        // Check for collision (calls the collision routines)
        // Application specific!!
        self.check_collision();

        // If no collision we just move along the velocity
        if !self.collision_packet.found_collision {
            return vector3_add(pos, vel);
        }

        // *** Collision occurred ***
        // The original destination point
        let destination_point = vector3_add(pos, vel);
        let mut new_base_point = pos;

        // Only update if we are not already very close
        // and if so we only move very close to intersection...not
        // to the exact spot.
        if self.collision_packet.nearest_distance >= very_close_distance as f64 {
            let mut v = vel;
            v = vector3_normalize(v);
            v = vector3_scale(
                v,
                self.collision_packet.nearest_distance as f32 - very_close_distance,
            );
            new_base_point = vector3_add(self.collision_packet.base_point, v);

            // Adjust polygon intersection point (so sliding
            // plane will be unaffected by the fact that we
            // move slightly less than collision tells us)
            let v = vector3_normalize(v);
            self.collision_packet.intersection_point = vector3_subtract(
                self.collision_packet.intersection_point,
                vector3_scale(v, very_close_distance),
            );
        }

        // Determine the sliding plane
        let slide_plane_origin = self.collision_packet.intersection_point;
        let mut slide_plane_normal =
            vector3_subtract(new_base_point, self.collision_packet.intersection_point);
        slide_plane_normal = vector3_negate(vector3_normalize(slide_plane_normal));

        let sliding_plane = Plane::new_from_normal(slide_plane_origin, slide_plane_normal);

        // Again, sorry about formatting.. but look carefully ;)
        let new_destination_point = vector3_subtract(
            destination_point,
            vector3_scale(
                slide_plane_normal,
                sliding_plane.signed_distance_to(&destination_point) as f32,
            ),
        );

        // Generate the slide Vector3, which will become our new
        // velocity Vector3 for the next iteration
        let new_velocity = vector3_subtract(
            new_destination_point,
            self.collision_packet.intersection_point,
        );

        // Recurse:
        // Don't recurse if the new velocity is very small
        if vector3_length(new_velocity) < very_close_distance {
            return new_base_point;
        }

        self.collision_recursion_depth += 1;
        return self.collide_with_world(new_base_point, new_velocity);
    }
}
