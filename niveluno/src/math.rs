pub use raymath::*;

pub fn scale(v: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    out_min + ((out_max) - out_min) * (((v) - in_min) / ((in_max) - in_min))
}

pub fn vec3_face_normal(v0: Vector3, v1: Vector3, v2: Vector3) -> Vector3 {
    let lh = vector3_subtract(v0, v1);
    let rh = vector3_subtract(v2, v1);

    let cross = vector3_cross_product(lh, rh);

    vector3_normalize(cross)
}

/// Transforms a Vector3 by a given Matrix
pub fn vector3_transform(v: Vector3, mat: Matrix) -> Vector3 {
    Vector3 {
        x: mat.m0 * v.x + mat.m4 * v.y + mat.m8 * v.z + mat.m12,
        y: mat.m1 * v.x + mat.m5 * v.y + mat.m9 * v.z + mat.m13,
        z: mat.m2 * v.x + mat.m6 * v.y + mat.m10 * v.z + mat.m14,
    }
}

/// Transforms a Vec<[Vector3;3]> by a given Matrix
pub fn mesh_tranform(mesh: Vec<[Vector3; 3]>, mat: Matrix) -> Vec<[Vector3; 3]> {
    let mut out = vec![];
    for tri in mesh {
        let v1 = vector3_transform(tri[0], mat);
        let v2 = vector3_transform(tri[1], mat);
        let v3 = vector3_transform(tri[2], mat);
        out.push([v1, v2, v3]);
    }
    out
}

fn get_box_vertices(bbox: &BoundingBox) -> [Vector3; 8] {
    [
        bbox.min,
        Vector3 {
            x: bbox.max.x,
            y: bbox.min.y,
            z: bbox.min.z,
        },
        Vector3 {
            x: bbox.min.x,
            y: bbox.max.y,
            z: bbox.min.z,
        },
        Vector3 {
            x: bbox.min.x,
            y: bbox.min.y,
            z: bbox.max.z,
        },
        bbox.max,
        Vector3 {
            x: bbox.min.x,
            y: bbox.max.y,
            z: bbox.max.z,
        },
        Vector3 {
            x: bbox.max.x,
            y: bbox.min.y,
            z: bbox.max.z,
        },
        Vector3 {
            x: bbox.max.x,
            y: bbox.max.y,
            z: bbox.min.z,
        },
    ]
}

fn get_triangle_edges(tri: [Vector3; 3]) -> [Vector3; 3] {
    [
        vector3_subtract(tri[1], tri[0]),
        vector3_subtract(tri[2], tri[1]),
        vector3_subtract(tri[0], tri[2]),
    ]
}

fn project_shape_on_axis(vertices: &[Vector3], axis: Vector3) -> (f32, f32) {
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    for v in vertices {
        let projection = vector3_dot_product(*v, axis);
        if projection < min {
            min = projection;
        }
        if projection > max {
            max = projection;
        }
    }

    (min, max)
}

fn lines_overlap(min1: f32, max1: f32, min2: f32, max2: f32) -> bool {
    !(min1 > max2 || min2 > max1)
}

fn is_point_in_box(point: Vector3, bbox: &BoundingBox) -> bool {
    point.x >= bbox.min.x
        && point.x <= bbox.max.x
        && point.y >= bbox.min.y
        && point.y <= bbox.max.y
        && point.z >= bbox.min.z
        && point.z <= bbox.max.z
}

// SAT intersection between an AABB and a triangle
pub fn sat_aabb_tri(bbox: &BoundingBox, tri: [Vector3; 3]) -> bool {
    // return early if a vertex is in the box
    // maybe remove, might not actually save much perf
    for &vertex in &tri {
        if is_point_in_box(vertex, &bbox) {
            return true;
        }
    }

    let box_vertices = get_box_vertices(bbox);
    let tri_vertices = [tri[0], tri[1], tri[2]];
    let tri_edges = get_triangle_edges(tri);

    let box_normals = vec![
        Vector3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    ];

    let tri_normal = vector3_normalize(vector3_cross_product(tri_edges[0], tri_edges[1]));

    // get cross products of box normals and triangle edges
    let mut cross_axes = vec![];
    for box_normal in &box_normals {
        for tri_edge in &tri_edges {
            cross_axes.push(vector3_normalize(vector3_cross_product(
                *box_normal,
                *tri_edge,
            )));
        }
    }

    let mut axes = box_normals;
    axes.push(tri_normal);
    axes.append(&mut cross_axes);

    for axis in axes {
        let (box_min, box_max) = project_shape_on_axis(&box_vertices, axis);
        let (tri_min, tri_max) = project_shape_on_axis(&tri_vertices, axis);

        // if no overlap, no collision
        if !lines_overlap(box_min, box_max, tri_min, tri_max) {
            return false;
        }
    }

    // all axes overlap
    true
}

pub fn world_point_to_screen_coord(
    location: Vector3,
    camera_pos: Vector3,
    camera_yaw: f32,
    camera_pitch: f32,
    screen_width: f32,
    screen_height: f32,
) -> Option<Vector2> {
    // get pos relative to camera
    let view_pos = vector3_subtract(location, camera_pos);

    // get rotated pos
    let yaw_matrix = matrix_rotate_y(-camera_yaw);
    let pitch_matrix = matrix_rotate_x(-camera_pitch);
    let rotation_matrix = matrix_multiply(yaw_matrix, pitch_matrix);
    let rotated_pos = vector3_transform(view_pos, rotation_matrix);

    // point is behind the camera
    if rotated_pos.z <= 0.0 {
        return None;
    }

    // project
    let aspect_ratio = screen_width / screen_height;
    let screen_x = (rotated_pos.x / rotated_pos.z) * (screen_width * 0.5) + (screen_width * 0.5);
    let screen_y = (rotated_pos.y / rotated_pos.z) * (screen_height * 0.5 * aspect_ratio)
        + (screen_height * 0.5);

    // Flip the Y-axis to match screen coordinates (top-left origin)
    Some(Vector2 {
        x: screen_x,
        y: screen_height - screen_y,
    })
}



// Get collision info between ray and mesh
pub fn get_ray_collision_mesh(
    ray: Ray,
    mesh: Vec<[Vector3; 3]>,
    transform: Matrix,
    check_within: Option<(Vector3, f32)>,
) -> RayCollision {
    let mut collision = RayCollision {
        hit: false,
        distance: 0.,
        point: Vector3 {
            x: 0.,
            y: 0.,
            z: 0.,
        },
        normal: Vector3 {
            x: 0.,
            y: 0.,
            z: 0.,
        },
    };

    // Test against all triangles in mesh
    for tri in mesh {
        let mut a = tri[0];
        let mut b = tri[1];
        let mut c = tri[2];

        a = vector3_transform(a, transform);
        b = vector3_transform(b, transform);
        c = vector3_transform(c, transform);

        if let Some((pos, max_dist)) = check_within {
            if (vector3_distance(pos, a) > max_dist)
                && (vector3_distance(pos, b) > max_dist)
                && (vector3_distance(pos, c) > max_dist)
            {
                continue;
            }
        }

        let tri_hit_info = get_ray_collision_triangle(ray.clone(), a, b, c);

        if tri_hit_info.hit {
            // Save the closest hit triangle
            if (!collision.hit) || (collision.distance > tri_hit_info.distance) {
                collision = tri_hit_info;
            }
        }
    }

    return collision;
}