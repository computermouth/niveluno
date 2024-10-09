use std::ops::Bound;

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

// Get collision info between ray and mesh
pub fn get_padded_ray_collision_mesh(
    ray: Ray,
    mesh: Vec<[Vector3; 3]>,
    transform: Matrix,
    padding: f32,
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

    // Check if mesh vertex data on CPU for testing
    // if (mesh.vertices != NULL)
    // {
    //     int triangleCount = mesh.triangleCount;

    // Test against all triangles in mesh
    for tri in mesh {
        let mut a = tri[0];
        let mut b = tri[1];
        let mut c = tri[2];

        a = vector3_transform(a, transform);
        b = vector3_transform(b, transform);
        c = vector3_transform(c, transform);

        let tri_hit_info = get_padded_ray_collision_triangle(ray.clone(), padding, a, b, c);

        if tri_hit_info.hit {
            // Save the closest hit triangle
            if (!collision.hit) || (collision.distance > tri_hit_info.distance) {
                collision = tri_hit_info;
            }
        }
    }
    // }

    return collision;
}

// Get collision info between ray and triangle
// NOTE: The points are expected to be in counter-clockwise winding
// NOTE: Based on https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
pub fn get_padded_ray_collision_triangle(
    ray: Ray,
    padding: f32,
    p1: Vector3,
    p2: Vector3,
    p3: Vector3,
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

    // Find vectors for two edges sharing V1
    let edge1 = vector3_subtract(p2, p1);
    let edge2 = vector3_subtract(p3, p1);

    // commit sins
    let normal = vector3_normalize(vector3_cross_product(edge1, edge2));
    let pad = vector3_scale(vector3_negate(normal), padding);
    let p1 = vector3_add(p1, pad);
    let p2 = vector3_add(p2, pad);
    let p3 = vector3_add(p3, pad);
    let edge1 = vector3_subtract(p2, p1);
    let edge2 = vector3_subtract(p3, p1);

    // Begin calculating determinant - also used to calculate u parameter
    let p = vector3_cross_product(ray.direction, edge2);

    // If determinant is near zero, ray lies in plane of triangle or ray is parallel to plane of triangle
    let det = vector3_dot_product(edge1, p);

    // Avoid culling!
    if (det > -f32::EPSILON) && (det < f32::EPSILON) {
        return collision;
    };

    let inv_det = 1.0 / det;

    // Calculate distance from V1 to ray origin
    let tv = vector3_subtract(ray.position, p1);

    // Calculate u parameter and test bound
    let u = vector3_dot_product(tv, p) * inv_det;

    // The intersection lies outside the triangle
    if (u < 0.0) || (u > 1.0) {
        return collision;
    }

    // Prepare to test v parameter
    let q = vector3_cross_product(tv, edge1);

    // Calculate V parameter and test bound
    let v = vector3_dot_product(ray.direction, q) * inv_det;

    // The intersection lies outside the triangle
    if (v < 0.0) || ((u + v) > 1.0) {
        return collision;
    }

    let t = vector3_dot_product(edge2, q) * inv_det;

    if t > f32::EPSILON {
        // Ray hit, get hit point and normal
        collision.hit = true;
        collision.distance = t;
        collision.normal = normal;
        collision.point = vector3_add(ray.position, vector3_scale(ray.direction, t));
    }

    return collision;
}

fn get_box_vertices(bbox: BoundingBox) -> [Vector3; 8] {
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
pub fn sat_aabb_tri(bbox: BoundingBox, tri: [Vector3; 3]) -> bool {
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
            y: 1.0,
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
