//----------------------------------------------------------------------------------
// Consts
//----------------------------------------------------------------------------------

pub const DEG2RAD: f32 = std::f32::consts::PI / 180.0;
pub const RAD2DEG: f32 = 180.0 / std::f32::consts::PI;

//----------------------------------------------------------------------------------
// Types and Structures Definition
//----------------------------------------------------------------------------------

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

// deleteme
impl From<[f32; 5]> for Vector2 {
    fn from(f: [f32; 5]) -> Self {
        Self { x: f[0], y: f[1] }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<[f32; 3]> for Vector3 {
    fn from(f: [f32; 3]) -> Self {
        Self::new(f[0], f[1], f[2])
    }
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl From<Matrix> for [f32; 16] {
    fn from(m: Matrix) -> Self {
        [
            m.m0, m.m1, m.m2, m.m3, m.m4, m.m5, m.m6, m.m7, m.m8, m.m9, m.m10, m.m11, m.m12, m.m13,
            m.m14, m.m15,
        ]
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl From<[f32; 4]> for Vector4 {
    fn from(f: [f32; 4]) -> Self {
        Self::new(f[0], f[1], f[2], f[3])
    }
}

impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}

pub type Quaternion = Vector4;
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Matrix {
    pub m0: f32,
    pub m1: f32,
    pub m2: f32,
    pub m3: f32,
    pub m4: f32,
    pub m5: f32,
    pub m6: f32,
    pub m7: f32,
    pub m8: f32,
    pub m9: f32,
    pub m10: f32,
    pub m11: f32,
    pub m12: f32,
    pub m13: f32,
    pub m14: f32,
    pub m15: f32,
}

impl From<[f32; 16]> for Matrix {
    fn from(f: [f32; 16]) -> Self {
        Matrix {
            m0: f[0],
            m1: f[1],
            m2: f[2],
            m3: f[3],
            m4: f[4],
            m5: f[5],
            m6: f[6],
            m7: f[7],
            m8: f[8],
            m9: f[9],
            m10: f[10],
            m11: f[11],
            m12: f[12],
            m13: f[13],
            m14: f[14],
            m15: f[15],
        }
    }
}

#[derive(Copy, Clone)]
pub struct Float3 {
    pub v: [f32; 3],
}

#[derive(Copy, Clone)]
pub struct Float16 {
    pub v: [f32; 16],
}

//----------------------------------------------------------------------------------
// Module Functions Definition - Utils math
//----------------------------------------------------------------------------------

/// Calculate linear interpolation between two floats
pub fn lerp(start: f32, end: f32, amount: f32) -> f32 {
    start + amount * (end - start)
}

/// Normalize input value within input range
pub fn normalize(value: f32, start: f32, end: f32) -> f32 {
    (value - start) / (end - start)
}

/// Remap input value within input range to output range
pub fn remap(
    value: f32,
    input_start: f32,
    input_end: f32,
    output_start: f32,
    output_end: f32,
) -> f32 {
    (value - input_start) / (input_end - input_start) * (output_end - output_start) + output_start
}

/// Wrap input value from min to max
pub fn wrap(value: f32, min: f32, max: f32) -> f32 {
    value - (max - min) * (((value - min) / (max - min)).floor())
}

/// Check whether two given floats are almost equal
pub fn float_equals(x: f32, y: f32) -> bool {
    (x - y).abs() <= f32::EPSILON * 1.0f32.max(x.abs().max(y.abs()))
}

//----------------------------------------------------------------------------------
// Module Functions Definition - Vector2 math
//----------------------------------------------------------------------------------

/// Vector with components value 0.0f
pub fn vector2_zero() -> Vector2 {
    Vector2 {
        x: 0.0f32,
        y: 0.0f32,
    }
}

/// Vector with components value 1.0f
pub fn vector2_one() -> Vector2 {
    Vector2 {
        x: 1.0f32,
        y: 1.0f32,
    }
}

/// Add two vectors (v1 + v2)
pub fn vector2_add(v1: Vector2, v2: Vector2) -> Vector2 {
    Vector2 {
        x: v1.x + v2.x,
        y: v1.y + v2.y,
    }
}

/// Add vector and float value
pub fn vector2_add_value(v: Vector2, add: f32) -> Vector2 {
    Vector2 {
        x: v.x + add,
        y: v.y + add,
    }
}

/// Subtract two vectors (v1 - v2)
pub fn vector2_subtract(v1: Vector2, v2: Vector2) -> Vector2 {
    Vector2 {
        x: v1.x - v2.x,
        y: v1.y - v2.y,
    }
}

/// Subtract vector by float value
pub fn vector2_subtract_value(v: Vector2, sub: f32) -> Vector2 {
    Vector2 {
        x: v.x - sub,
        y: v.y - sub,
    }
}

/// Calculate vector length
pub fn vector2_length(v: Vector2) -> f32 {
    ((v.x * v.x) + (v.y * v.y)).sqrt()
}

/// Calculate vector square length
pub fn vector2_length_sqr(v: Vector2) -> f32 {
    (v.x * v.x) + (v.y * v.y)
}

/// Calculate distance between two vectors
pub fn vector2_dot_product(v1: Vector2, v2: Vector2) -> f32 {
    v1.x * v2.x + v1.y * v2.y
}

/// Calculate distance between two vectors
pub fn vector2_distance(v1: Vector2, v2: Vector2) -> f32 {
    vector2_distance_sqr(v1, v2).sqrt()
}

/// Calculate square distance between two vectors
pub fn vector2_distance_sqr(v1: Vector2, v2: Vector2) -> f32 {
    (v1.x - v2.x) * (v1.x - v2.x) + (v1.y - v2.y) * (v1.y - v2.y)
}

/// Calculate angle between two vectors
/// NOTE: Angle is calculated from origin point (0, 0)
pub fn vector2_angle(v1: Vector2, v2: Vector2) -> f32 {
    let dot = v1.x * v2.x + v1.y * v2.y;
    let det = v1.x * v2.y - v1.y * v2.x;

    det.atan2(dot)
}

// =======================================================
// HERE HERE HERE HERE HERE HERE HERE HERE HERE HERE
// =======================================================

/// Calculate angle defined by a two vectors line
/// NOTE: Parameters need to be normalized
/// Current implementation should be aligned with glm::angle
pub fn vector2_line_angle(start: Vector2, end: Vector2) -> f32 {
    // TODO(10/9/2023): Currently angles move clockwise, determine if this is wanted behavior
    -((end.y - start.y).atan2(end.x - start.x))
}

/// Scale vector (multiply by value)
pub fn vector2_scale(v: Vector2, scale: f32) -> Vector2 {
    Vector2 {
        x: v.x * scale,
        y: v.y * scale,
    }
}

/// Multiply vector by vector
pub fn vector2_multiply(v1: Vector2, v2: Vector2) -> Vector2 {
    Vector2 {
        x: v1.x * v2.x,
        y: v1.y * v2.y,
    }
}

/// Negate vector
pub fn vector2_negate(v: Vector2) -> Vector2 {
    Vector2 { x: -v.x, y: -v.y }
}

/// Divide vector by vector
pub fn vector2_divide(v1: Vector2, v2: Vector2) -> Vector2 {
    Vector2 {
        x: v1.x / v2.x,
        y: v1.y / v2.y,
    }
}

/// Normalize provided vector
pub fn vector2_normalize(v: Vector2) -> Vector2 {
    let mut result = Vector2 { x: 0., y: 0. };
    let length: f32 = (v.x * v.x + v.y * v.y).sqrt();
    if length > 0. {
        let ilength = 1.0 / length;
        result.x = v.x * ilength;
        result.y = v.y * ilength;
    }
    result
}

/// Transforms a Vector2 by a given Matrix
pub fn vector2_transform(v: Vector2, mat: Matrix) -> Vector2 {
    let mut result = Vector2 { x: 0., y: 0. };
    let x = v.x;
    let y = v.y;
    let z = 0.;
    result.x = mat.m0 * x + mat.m4 * y + mat.m8 * z + mat.m12;
    result.y = mat.m1 * x + mat.m5 * y + mat.m9 * z + mat.m13;
    result
}

/// Calculate linear interpolation between two vectors
pub fn vector2_lerp(v1: Vector2, v2: Vector2, amount: f32) -> Vector2 {
    Vector2 {
        x: lerp(v1.x, v2.x, amount),
        y: lerp(v1.y, v2.y, amount),
    }
}

/// Calculate reflected vector to normal
pub fn vector2_reflect(v: Vector2, normal: Vector2) -> Vector2 {
    let mut result = Vector2 { x: 0., y: 0. };
    let dot_product = vector2_dot_product(v, normal);
    result.x = v.x - 2.0f32 * normal.x * dot_product;
    result.y = v.y - 2.0f32 * normal.y * dot_product;
    result
}

/// Get min value for each pair of components
pub fn vector2_min(v1: Vector2, v2: Vector2) -> Vector2 {
    let mut result = Vector2 { x: 0., y: 0. };
    result.x = v1.x.min(v2.x);
    result.y = v1.y.min(v2.y);
    result
}

/// Get max value for each pair of components
pub fn vector2_max(v1: Vector2, v2: Vector2) -> Vector2 {
    let mut result = Vector2 { x: 0., y: 0. };
    result.x = v1.x.max(v2.x);
    result.y = v1.y.max(v2.y);
    result
}

/// Rotate vector by angle
pub fn vector2_rotate(v: Vector2, angle: f32) -> Vector2 {
    let mut result = Vector2 { x: 0., y: 0. };
    let cosres = angle.cos();
    let sinres = angle.sin();
    result.x = v.x * cosres - v.y * sinres;
    result.y = v.x * sinres + v.y * cosres;
    result
}

/// Move Vector towards target
pub fn vector2_move_towards(v: Vector2, target: Vector2, max_distance: f32) -> Vector2 {
    let mut result = Vector2 { x: 0., y: 0. };
    let dx = target.x - v.x;
    let dy = target.y - v.y;
    let value = dx * dx + dy * dy;
    if value == 0. || max_distance >= 0. && value <= max_distance * max_distance {
        return target;
    }
    let dist: f32 = value.sqrt();
    result.x = v.x + dx / dist * max_distance;
    result.y = v.y + dy / dist * max_distance;
    result
}

/// Invert the given vector
pub fn vector2_invert(v: Vector2) -> Vector2 {
    Vector2 {
        x: 1.0 / v.x,
        y: 1.0 / v.y,
    }
}

/// Clamp the components of the vector between min and max values specified by the given vectors
pub fn vector2_clamp(v: Vector2, min: Vector2, max: Vector2) -> Vector2 {
    let mut result = Vector2 { x: 0., y: 0. };
    result.x = v.x.clamp(min.x, max.x);
    result.y = v.y.clamp(min.y, max.y);
    result
}

/// Clamp the magnitude of the vector between two min and max values
pub fn vector2_clamp_value(v: Vector2, min: f32, max: f32) -> Vector2 {
    let mut result: Vector2 = v;
    let mut length = v.x * v.x + v.y * v.y;
    if length > 0.0 {
        length = length.sqrt();
        let mut scale = 1.;
        if length < min {
            scale = min / length;
        } else if length > max {
            scale = max / length;
        }
        result.x = v.x * scale;
        result.y = v.y * scale;
    }
    result
}

/// Check whether two given vectors are almost equal
pub fn vector2_equals(p: Vector2, q: Vector2) -> bool {
    float_equals(p.x, q.x) && float_equals(p.y, q.y)
}

/// Compute the direction of a refracted ray
///
/// v: normalized direction of the incoming ray
/// n: normalized normal vector of the interface of two optical media
/// r: ratio of the refractive index of the medium from where the ray comes
///    to the refractive index of the medium on the other side of the surface
pub fn vector2_refract(mut v: Vector2, n: Vector2, r: f32) -> Vector2 {
    let mut result = Vector2 { x: 0., y: 0. };
    let dot: f32 = vector2_dot_product(v, n);
    let mut d: f32 = 1.0f32 - r * r * (1.0f32 - dot * dot);
    if d >= 0.0f32 {
        d = d.sqrt();
        v.x = r * v.x - (r * dot + d) * n.x;
        v.y = r * v.y - (r * dot + d) * n.y;
        result = v;
    }
    result
}

//----------------------------------------------------------------------------------
// Module Functions Definition - Vector3 math
//----------------------------------------------------------------------------------

/// Vector with components value 0.0f
pub fn vector3_zero() -> Vector3 {
    Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }
}

/// Vector with components value 1.0f
pub fn vector3_one() -> Vector3 {
    Vector3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    }
}

/// Add two vectors
pub fn vector3_add(v1: Vector3, v2: Vector3) -> Vector3 {
    Vector3 {
        x: v1.x + v2.x,
        y: v1.y + v2.y,
        z: v1.z + v2.z,
    }
}

/// Add vector and float value
pub fn vector3_add_value(v: Vector3, add: f32) -> Vector3 {
    Vector3 {
        x: v.x + add,
        y: v.y + add,
        z: v.z + add,
    }
}

/// Subtract two vectors
pub fn vector3_subtract(v1: Vector3, v2: Vector3) -> Vector3 {
    Vector3 {
        x: v1.x - v2.x,
        y: v1.y - v2.y,
        z: v1.z - v2.z,
    }
}

/// Subtract vector by float value
pub fn vector3_subtract_value(v: Vector3, sub: f32) -> Vector3 {
    Vector3 {
        x: v.x - sub,
        y: v.y - sub,
        z: v.z - sub,
    }
}

/// Multiply vector by scalar
pub fn vector3_scale(v: Vector3, scalar: f32) -> Vector3 {
    Vector3 {
        x: v.x * scalar,
        y: v.y * scalar,
        z: v.z * scalar,
    }
}

/// Multiply vector by vector
pub fn vector3_multiply(v1: Vector3, v2: Vector3) -> Vector3 {
    Vector3 {
        x: v1.x * v2.x,
        y: v1.y * v2.y,
        z: v1.z * v2.z,
    }
}

/// Calculate two vectors cross product
pub fn vector3_cross_product(v1: Vector3, v2: Vector3) -> Vector3 {
    Vector3 {
        x: v1.y * v2.z - v1.z * v2.y,
        y: v1.z * v2.x - v1.x * v2.z,
        z: v1.x * v2.y - v1.y * v2.x,
    }
}

/// Calculate one vector perpendicular vector
pub fn vector3_perpendicular(v: Vector3) -> Vector3 {
    let mut result = Vector3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let mut min: f32 = v.x.abs();
    let mut cardinal_axis = Vector3 {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    if v.y.abs() < min {
        min = v.y.abs();
        let tmp = Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        cardinal_axis = tmp;
    }
    if v.z.abs() < min {
        let tmp = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
        cardinal_axis = tmp;
    }
    result.x = v.y * cardinal_axis.z - v.z * cardinal_axis.y;
    result.y = v.z * cardinal_axis.x - v.x * cardinal_axis.z;
    result.z = v.x * cardinal_axis.y - v.y * cardinal_axis.x;
    result
}

/// Calculate vector length
pub fn vector3_length(v: Vector3) -> f32 {
    (v.x * v.x + v.y * v.y + v.z * v.z).sqrt()
}

/// Calculate vector square length
pub fn vector3_length_sqr(v: Vector3) -> f32 {
    v.x * v.x + v.y * v.y + v.z * v.z
}

/// Calculate two vectors dot product
pub fn vector3_dot_product(v1: Vector3, v2: Vector3) -> f32 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

/// Calculate distance between two vectors
pub fn vector3_distance(v1: Vector3, v2: Vector3) -> f32 {
    vector3_distance_sqr(v1, v2).sqrt()
}

/// Calculate square distance between two vectors
pub fn vector3_distance_sqr(v1: Vector3, v2: Vector3) -> f32 {
    let dx: f32 = v2.x - v1.x;
    let dy: f32 = v2.y - v1.y;
    let dz: f32 = v2.z - v1.z;

    dx * dx + dy * dy + dz * dz
}

/// Calculate angle between two vectors
pub fn vector3_angle(v1: Vector3, v2: Vector3) -> f32 {
    let cross = vector3_cross_product(v1, v2);
    let len = vector3_length(cross);
    let dot: f32 = vector3_dot_product(v1, v2);
    len.atan2(dot)
}

/// Negate provided vector (invert direction)
pub fn vector3_negate(v: Vector3) -> Vector3 {
    Vector3 {
        x: -v.x,
        y: -v.y,
        z: -v.z,
    }
}

/// Divide vector by vector
pub fn vector3_divide(v1: Vector3, v2: Vector3) -> Vector3 {
    Vector3 {
        x: v1.x / v2.x,
        y: v1.y / v2.y,
        z: v1.z / v2.z,
    }
}

/// Normalize provided vector
pub fn vector3_normalize(v: Vector3) -> Vector3 {
    let mut result: Vector3 = v;
    let length = vector3_length(v);
    if length != 0.0f32 {
        let ilength: f32 = 1.0f32 / length;
        result = vector3_scale(result, ilength);
    }
    result
}

///Calculate the projection of the vector v1 on to v2
pub fn vector3_project(v1: Vector3, v2: Vector3) -> Vector3 {
    let v1dv2: f32 = v1.x * v2.x + v1.y * v2.y + v1.z * v2.z;
    let v2dv2: f32 = v2.x * v2.x + v2.y * v2.y + v2.z * v2.z;
    let mag: f32 = v1dv2 / v2dv2;
    vector3_scale(v2, mag)
}

///Calculate the rejection of the vector v1 on to v2
pub fn vector3_reject(v1: Vector3, v2: Vector3) -> Vector3 {
    let mut result = Vector3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let v1dv2: f32 = v1.x * v2.x + v1.y * v2.y + v1.z * v2.z;
    let v2dv2: f32 = v2.x * v2.x + v2.y * v2.y + v2.z * v2.z;
    let mag: f32 = v1dv2 / v2dv2;
    result.x = v1.x - v2.x * mag;
    result.y = v1.y - v2.y * mag;
    result.z = v1.z - v2.z * mag;
    result
}

/// Orthonormalize provided vectors
///
/// Makes vectors normalized and orthogonal to each other
/// Gram-Schmidt function implementation
pub fn vector3_ortho_normalize(v1: Vector3, v2: Vector3) -> (Vector3, Vector3) {
    let v1_out = vector3_normalize(v1);
    let vn = vector3_normalize(vector3_cross_product(v1_out, v2));
    let v2_out = vector3_cross_product(vn, v1_out);

    (v1_out, v2_out)
}

/// Transforms a Vector3 by a given Matrix
pub fn vector3_transform(v: Vector3, mat: Matrix) -> Vector3 {
    Vector3 {
        x: mat.m0 * v.x + mat.m4 * v.y + mat.m8 * v.z + mat.m12,
        y: mat.m1 * v.x + mat.m5 * v.y + mat.m9 * v.z + mat.m13,
        z: mat.m2 * v.x + mat.m6 * v.y + mat.m10 * v.z + mat.m14,
    }
}

/// Transform a vector by quaternion rotation
pub fn vector3_rotate_by_quaternion(v: Vector3, q: Quaternion) -> Vector3 {
    Vector3 {
        x: v.x * (q.x * q.x + q.w * q.w - q.y * q.y - q.z * q.z)
            + v.y * (2.0 * q.x * q.y - 2.0 * q.w * q.z)
            + v.z * (2.0 * q.x * q.z + 2.0 * q.w * q.y),
        y: v.x * (2.0 * q.w * q.z + 2.0 * q.x * q.y)
            + v.y * (q.w * q.w - q.x * q.x + q.y * q.y - q.z * q.z)
            + v.z * (-2.0 * q.w * q.x + 2.0 * q.y * q.z),
        z: v.x * (-2.0 * q.w * q.y + 2.0 * q.x * q.z)
            + v.y * (2.0 * q.w * q.x + 2.0 * q.y * q.z)
            + v.z * (q.w * q.w - q.x * q.x - q.y * q.y + q.z * q.z),
    }
}

/// Rotates a vector around an axis
///
/// Using Euler-Rodrigues Formula
/// Ref.: https://en.wikipedia.org/w/index.php?title=Euler%E2%80%93Rodrigues_formula
pub fn vector3_rotate_by_axis_angle(v: Vector3, axis: Vector3, angle: f32) -> Vector3 {
    let n_axis = vector3_normalize(axis);
    let h_angle = angle / 2.;

    let mut a: f32 = h_angle.sin();

    let w = vector3_scale(n_axis, a);

    a = h_angle.cos();

    let wv = vector3_cross_product(w, v);
    let wwv = vector3_cross_product(w, wv);

    let wv = vector3_scale(wv, 2. * a);
    let wwv = vector3_scale(wwv, 2.);

    vector3_add(vector3_add(v, wv), wwv)
}

/// Move Vector towards target
pub fn vector3_move_towards(v: Vector3, target: Vector3, max_distance: f32) -> Vector3 {
    let mut result = Vector3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let dx: f32 = target.x - v.x;
    let dy: f32 = target.y - v.y;
    let dz: f32 = target.z - v.z;
    let value: f32 = dx * dx + dy * dy + dz * dz;
    if value == 0. || max_distance >= 0. && value <= max_distance * max_distance {
        return target;
    }
    let dist: f32 = value.sqrt();
    result.x = v.x + dx / dist * max_distance;
    result.y = v.y + dy / dist * max_distance;
    result.z = v.z + dz / dist * max_distance;
    result
}

/// Calculate linear interpolation between two vectors
pub fn vector3_lerp(v1: Vector3, v2: Vector3, amount: f32) -> Vector3 {
    Vector3 {
        x: lerp(v1.x, v2.x, amount),
        y: lerp(v1.y, v2.y, amount),
        z: lerp(v1.z, v2.z, amount),
    }
}
/// Calculate cubic hermite interpolation between two vectors and their tangents
///
/// as described in the GLTF 2.0 specification: https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#interpolation-cubic
pub fn vector3_cubic_hermite(
    v1: Vector3,
    tangent1: Vector3,
    v2: Vector3,
    tangent2: Vector3,
    amount: f32,
) -> Vector3 {
    let amount_pow2: f32 = amount * amount;
    let amount_pow3: f32 = amount * amount * amount;
    Vector3 {
        x: (2. * amount_pow3 - 3. * amount_pow2 + 1.) * v1.x
            + (amount_pow3 - 2. * amount_pow2 + amount) * tangent1.x
            + (-2. * amount_pow3 + 3. * amount_pow2) * v2.x
            + (amount_pow3 - amount_pow2) * tangent2.x,
        y: (2. * amount_pow3 - 3. * amount_pow2 + 1.) * v1.y
            + (amount_pow3 - 2. * amount_pow2 + amount) * tangent1.y
            + (-2. * amount_pow3 + 3. * amount_pow2) * v2.y
            + (amount_pow3 - amount_pow2) * tangent2.y,
        z: (2. * amount_pow3 - 3. * amount_pow2 + 1.) * v1.z
            + (amount_pow3 - 2. * amount_pow2 + amount) * tangent1.z
            + (-2. * amount_pow3 + 3. * amount_pow2) * v2.z
            + (amount_pow3 - amount_pow2) * tangent2.z,
    }
}

/// Calculate reflected vector to normal
pub fn vector3_reflect(v: Vector3, normal: Vector3) -> Vector3 {
    let dot_product = vector3_dot_product(v, normal);
    Vector3 {
        x: v.x - (2.0 * normal.x) * dot_product,
        y: v.y - (2.0 * normal.y) * dot_product,
        z: v.z - (2.0 * normal.z) * dot_product,
    }
}

/// Get min value for each pair of components
pub fn vector3_min(v1: Vector3, v2: Vector3) -> Vector3 {
    Vector3 {
        x: v1.x.min(v2.x),
        y: v1.y.min(v2.y),
        z: v1.z.min(v2.z),
    }
}

/// Get max value for each pair of components
pub fn vector3_max(v1: Vector3, v2: Vector3) -> Vector3 {
    Vector3 {
        x: v1.x.max(v2.x),
        y: v1.y.max(v2.y),
        z: v1.z.max(v2.z),
    }
}

/// Compute barycenter coordinates (u, v, w) for point p with respect to triangle (a, b, c)
///
/// NOTE: Assumes P is on the plane of the triangle
pub fn vector3_barycenter(p: Vector3, a: Vector3, b: Vector3, c: Vector3) -> Vector3 {
    let v0 = vector3_subtract(b, a);
    let v1 = vector3_subtract(c, a);
    let v2 = vector3_subtract(p, a);
    let d00 = vector3_dot_product(v0, v0);
    let d01 = vector3_dot_product(v0, v1);
    let d11 = vector3_dot_product(v1, v1);
    let d20 = vector3_dot_product(v2, v0);
    let d21 = vector3_dot_product(v2, v1);
    let denom = d00 * d11 - d01 * d01;

    let y = (d11 * d20 - d01 * d21) / denom;
    let z = (d00 * d21 - d01 * d20) / denom;
    Vector3 {
        x: 1.0 - (z + y),
        y,
        z,
    }
}

/// Projects a Vector3 from screen space into object space
pub fn vector3_unproject(source: Vector3, projection: Matrix, view: Matrix) -> Vector3 {
    let mat_view_proj = matrix_multiply(view, projection);
    let mat_view_proj_inv = matrix_invert(mat_view_proj);

    let quat = Quaternion {
        x: source.x,
        y: source.y,
        z: source.z,
        w: 1.0f32,
    };
    let qtransformed = quaternion_transform(quat, mat_view_proj_inv);
    Vector3 {
        x: qtransformed.x / qtransformed.w,
        y: qtransformed.y / qtransformed.w,
        z: qtransformed.z / qtransformed.w,
    }
}

/// Get Vector3 as float array
pub fn vector3_to_float(v: Vector3) -> [f32; 3] {
    vector3_to_float_v(v).v
}

/// Get Vector3 as float array
pub fn vector3_to_float_v(v: Vector3) -> Float3 {
    Float3 { v: [v.x, v.y, v.z] }
}

/// Invert the given vector
pub fn vector3_invert(v: Vector3) -> Vector3 {
    Vector3 {
        x: 1.0f32 / v.x,
        y: 1.0f32 / v.y,
        z: 1.0f32 / v.z,
    }
}

/// Clamp the components of the vector between
/// min and max values specified by the given vectors
pub fn vector3_clamp(v: Vector3, min: Vector3, max: Vector3) -> Vector3 {
    Vector3 {
        x: v.x.clamp(min.x, max.x),
        y: v.y.clamp(min.y, max.y),
        z: v.z.clamp(min.z, max.z),
    }
}

/// Clamp the magnitude of the vector between two values
pub fn vector3_clamp_value(v: Vector3, min: f32, max: f32) -> Vector3 {
    let mut result: Vector3 = v;
    let mut length: f32 = v.x * v.x + v.y * v.y + v.z * v.z;
    if length > 0.0f32 {
        length = length.sqrt();
        let mut scale: f32 = 1.;
        if length < min {
            scale = min / length;
        } else if length > max {
            scale = max / length;
        }
        result.x = v.x * scale;
        result.y = v.y * scale;
        result.z = v.z * scale;
    }
    result
}

/// Check whether two given vectors are almost equal
pub fn vector3_equals(p: Vector3, q: Vector3) -> bool {
    float_equals(p.x, q.x) && float_equals(p.y, q.y) && float_equals(p.z, q.z)
}

/// Compute the direction of a refracted ray
///
/// v: normalized direction of the incoming ray
/// n: normalized normal vector of the interface of two optical media
/// r: ratio of the refractive index of the medium from where the ray comes
///    to the refractive index of the medium on the other side of the surface
pub fn vector3_refract(mut v: Vector3, n: Vector3, r: f32) -> Vector3 {
    let mut result = Vector3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let dot = vector3_dot_product(v, n);
    let mut d: f32 = 1.0 - r * r * (1.0 - dot * dot);
    if d >= 0.0f32 {
        d = d.sqrt();
        v.x = r * v.x - (r * dot + d) * n.x;
        v.y = r * v.y - (r * dot + d) * n.y;
        v.z = r * v.z - (r * dot + d) * n.z;
        result = v;
    }
    result
}

//----------------------------------------------------------------------------------
// Module Functions Definition - Vector4 math
//----------------------------------------------------------------------------------

pub fn vector4_zero() -> Vector4 {
    Vector4 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 0.0,
    }
}

pub fn vector4_one() -> Vector4 {
    Vector4 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
        w: 1.0,
    }
}

pub fn vector4_add(v1: Vector4, v2: Vector4) -> Vector4 {
    Vector4 {
        x: v1.x + v2.x,
        y: v1.y + v2.y,
        z: v1.z + v2.z,
        w: v1.w + v2.w,
    }
}

pub fn vector4_add_value(v: Vector4, add: f32) -> Vector4 {
    Vector4 {
        x: v.x + add,
        y: v.y + add,
        z: v.z + add,
        w: v.w + add,
    }
}

pub fn vector4_subtract(v1: Vector4, v2: Vector4) -> Vector4 {
    Vector4 {
        x: v1.x - v2.x,
        y: v1.y - v2.y,
        z: v1.z - v2.z,
        w: v1.w - v2.w,
    }
}

pub fn vector4_subtract_value(v: Vector4, add: f32) -> Vector4 {
    Vector4 {
        x: v.x - add,
        y: v.y - add,
        z: v.z - add,
        w: v.w - add,
    }
}

pub fn vector4_length(v: Vector4) -> f32 {
    vector4_length_sqr(v).sqrt()
}

pub fn vector4_length_sqr(v: Vector4) -> f32 {
    v.x * v.x + v.y * v.y + v.z * v.z + v.w * v.w
}

pub fn vector4_dot_product(v1: Vector4, v2: Vector4) -> f32 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z + v1.w * v2.w
}

/// Calculate distance between two vectors
pub fn vector4_distance(v1: Vector4, v2: Vector4) -> f32 {
    ((v1.x - v2.x) * (v1.x - v2.x)
        + (v1.y - v2.y) * (v1.y - v2.y)
        + (v1.z - v2.z) * (v1.z - v2.z)
        + (v1.w - v2.w) * (v1.w - v2.w))
        .sqrt()
}

/// Calculate square distance between two vectors
pub fn vector4_distance_sqr(v1: Vector4, v2: Vector4) -> f32 {
    (v1.x - v2.x) * (v1.x - v2.x)
        + (v1.y - v2.y) * (v1.y - v2.y)
        + (v1.z - v2.z) * (v1.z - v2.z)
        + (v1.w - v2.w) * (v1.w - v2.w)
}

pub fn vector4_scale(v: Vector4, scale: f32) -> Vector4 {
    Vector4 {
        x: v.x * scale,
        y: v.y * scale,
        z: v.z * scale,
        w: v.w * scale,
    }
}

/// Multiply vector by vector
pub fn vector4_multiply(v1: Vector4, v2: Vector4) -> Vector4 {
    Vector4 {
        x: v1.x * v2.x,
        y: v1.y * v2.y,
        z: v1.z * v2.z,
        w: v1.w * v2.w,
    }
}

/// Negate vector
pub fn vector4_negate(v: Vector4) -> Vector4 {
    Vector4 {
        x: -v.x,
        y: -v.y,
        z: -v.z,
        w: -v.w,
    }
}

/// Divide vector by vector
pub fn vector4_divide(v1: Vector4, v2: Vector4) -> Vector4 {
    Vector4 {
        x: v1.x / v2.x,
        y: v1.y / v2.y,
        z: v1.z / v2.z,
        w: v1.w / v2.w,
    }
}

/// Normalize provided vector
pub fn vector4_normalize(v: Vector4) -> Vector4 {
    let mut length: f32 = vector4_length(v);
    if length == 0.0 {
        length = 1.0;
    }
    let ilength = 1.0 / length;

    Quaternion {
        x: v.x * ilength,
        y: v.y * ilength,
        z: v.z * ilength,
        w: v.w * ilength,
    }
}

/// Get min value for each pair of components
pub fn vector4_min(v1: Vector4, v2: Vector4) -> Vector4 {
    Vector4 {
        x: v1.x.min(v2.x),
        y: v1.y.min(v2.y),
        z: v1.z.min(v2.z),
        w: v1.w.min(v2.w),
    }
}

/// Get max value for each pair of components
pub fn vector4_max(v1: Vector4, v2: Vector4) -> Vector4 {
    Vector4 {
        x: v1.x.max(v2.x),
        y: v1.y.max(v2.y),
        z: v1.z.max(v2.z),
        w: v1.w.max(v2.w),
    }
}

/// Calculate linear interpolation between two vectors
pub fn vector4_lerp(v1: Vector4, v2: Vector4, amount: f32) -> Vector4 {
    Vector4 {
        x: lerp(v1.x, v2.x, amount),
        y: lerp(v1.y, v2.y, amount),
        z: lerp(v1.z, v2.z, amount),
        w: lerp(v1.w, v2.w, amount),
    }
}

/// Move Vector towards target
pub fn vector4_move_towards(v: Vector4, target: Vector4, max_distance: f32) -> Vector4 {
    let dx: f32 = target.x - v.x;
    let dy: f32 = target.y - v.y;
    let dz: f32 = target.z - v.z;
    let dw: f32 = target.w - v.w;
    let value: f32 = dx * dx + dy * dy + dz * dz + dw * dw;
    if value == 0. || max_distance >= 0. && value <= max_distance * max_distance {
        return target;
    }
    let dist: f32 = value.sqrt();
    Vector4 {
        x: v.x + dx / dist * max_distance,
        y: v.y + dy / dist * max_distance,
        z: v.z + dz / dist * max_distance,
        w: v.w + dw / dist * max_distance,
    }
}

/// Invert the given vector
pub fn vector4_invert(v: Vector4) -> Vector4 {
    Vector4 {
        x: 1.0 / v.x,
        y: 1.0 / v.y,
        z: 1.0 / v.z,
        w: 1.0 / v.w,
    }
}

/// Check whether two given vectors are almost equal
pub fn vector4_equals(p: Vector4, q: Vector4) -> bool {
    float_equals(p.x, q.x)
        && float_equals(p.y, q.y)
        && float_equals(p.z, q.z)
        && float_equals(p.w, q.w)
}

//----------------------------------------------------------------------------------
// Module Functions Definition - Matrix math
//----------------------------------------------------------------------------------

/// Compute matrix determinant
pub fn matrix_determinant(mat: Matrix) -> f32 {
    let a00: f32 = mat.m0;
    let a01: f32 = mat.m1;
    let a02: f32 = mat.m2;
    let a03: f32 = mat.m3;
    let a10: f32 = mat.m4;
    let a11: f32 = mat.m5;
    let a12: f32 = mat.m6;
    let a13: f32 = mat.m7;
    let a20: f32 = mat.m8;
    let a21: f32 = mat.m9;
    let a22: f32 = mat.m10;
    let a23: f32 = mat.m11;
    let a30: f32 = mat.m12;
    let a31: f32 = mat.m13;
    let a32: f32 = mat.m14;
    let a33: f32 = mat.m15;

    a30 * a21 * a12 * a03 - a20 * a31 * a12 * a03 - a30 * a11 * a22 * a03
        + a10 * a31 * a22 * a03
        + a20 * a11 * a32 * a03
        - a10 * a21 * a32 * a03
        - a30 * a21 * a02 * a13
        + a20 * a31 * a02 * a13
        + a30 * a01 * a22 * a13
        - a00 * a31 * a22 * a13
        - a20 * a01 * a32 * a13
        + a00 * a21 * a32 * a13
        + a30 * a11 * a02 * a23
        - a10 * a31 * a02 * a23
        - a30 * a01 * a12 * a23
        + a00 * a31 * a12 * a23
        + a10 * a01 * a32 * a23
        - a00 * a11 * a32 * a23
        - a20 * a11 * a02 * a33
        + a10 * a21 * a02 * a33
        + a20 * a01 * a12 * a33
        - a00 * a21 * a12 * a33
        - a10 * a01 * a22 * a33
        + a00 * a11 * a22 * a33
}

/// Get the trace of the matrix (sum of the values along the diagonal)
pub fn matrix_trace(mat: Matrix) -> f32 {
    mat.m0 + mat.m5 + mat.m10 + mat.m15
}

/// Transposes provided matrix
pub fn matrix_transpose(mat: Matrix) -> Matrix {
    Matrix {
        m0: mat.m0,
        m1: mat.m4,
        m2: mat.m8,
        m3: mat.m12,
        m4: mat.m1,
        m5: mat.m5,
        m6: mat.m9,
        m7: mat.m13,
        m8: mat.m2,
        m9: mat.m6,
        m10: mat.m10,
        m11: mat.m14,
        m12: mat.m3,
        m13: mat.m7,
        m14: mat.m11,
        m15: mat.m15,
    }
}

/// Invert provided matrix
pub fn matrix_invert(mat: Matrix) -> Matrix {
    let a00: f32 = mat.m0;
    let a01: f32 = mat.m1;
    let a02: f32 = mat.m2;
    let a03: f32 = mat.m3;
    let a10: f32 = mat.m4;
    let a11: f32 = mat.m5;
    let a12: f32 = mat.m6;
    let a13: f32 = mat.m7;
    let a20: f32 = mat.m8;
    let a21: f32 = mat.m9;
    let a22: f32 = mat.m10;
    let a23: f32 = mat.m11;
    let a30: f32 = mat.m12;
    let a31: f32 = mat.m13;
    let a32: f32 = mat.m14;
    let a33: f32 = mat.m15;
    let b00: f32 = a00 * a11 - a01 * a10;
    let b01: f32 = a00 * a12 - a02 * a10;
    let b02: f32 = a00 * a13 - a03 * a10;
    let b03: f32 = a01 * a12 - a02 * a11;
    let b04: f32 = a01 * a13 - a03 * a11;
    let b05: f32 = a02 * a13 - a03 * a12;
    let b06: f32 = a20 * a31 - a21 * a30;
    let b07: f32 = a20 * a32 - a22 * a30;
    let b08: f32 = a20 * a33 - a23 * a30;
    let b09: f32 = a21 * a32 - a22 * a31;
    let b10: f32 = a21 * a33 - a23 * a31;
    let b11: f32 = a22 * a33 - a23 * a32;
    let inv_det: f32 =
        1.0f32 / (b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06);
    Matrix {
        m0: ((a11 * b11) - (a12 * b10) + (a13 * b09)) * inv_det,
        m1: ((-a01 * b11) + (a02 * b10) - (a03 * b09)) * inv_det,
        m2: ((a31 * b05) - (a32 * b04) + (a33 * b03)) * inv_det,
        m3: ((-a21 * b05) + (a22 * b04) - (a23 * b03)) * inv_det,
        m4: ((-a10 * b11) + (a12 * b08) - (a13 * b07)) * inv_det,
        m5: ((a00 * b11) - (a02 * b08) + (a03 * b07)) * inv_det,
        m6: ((-a30 * b05) + (a32 * b02) - (a33 * b01)) * inv_det,
        m7: ((a20 * b05) - (a22 * b02) + (a23 * b01)) * inv_det,
        m8: ((a10 * b10) - (a11 * b08) + (a13 * b06)) * inv_det,
        m9: ((-a00 * b10) + (a01 * b08) - (a03 * b06)) * inv_det,
        m10: ((a30 * b04) - (a31 * b02) + (a33 * b00)) * inv_det,
        m11: ((-a20 * b04) + (a21 * b02) - (a23 * b00)) * inv_det,
        m12: ((-a10 * b09) + (a11 * b07) - (a12 * b06)) * inv_det,
        m13: ((a00 * b09) - (a01 * b07) + (a02 * b06)) * inv_det,
        m14: ((-a30 * b03) + (a31 * b01) - (a32 * b00)) * inv_det,
        m15: ((a20 * b03) - (a21 * b01) + (a22 * b00)) * inv_det,
    }
}

/// Get identity matrix
pub fn matrix_identity() -> Matrix {
    Matrix {
        m0: 1.0f32,
        m4: 0.0f32,
        m8: 0.0f32,
        m12: 0.0f32,
        m1: 0.0f32,
        m5: 1.0f32,
        m9: 0.0f32,
        m13: 0.0f32,
        m2: 0.0f32,
        m6: 0.0f32,
        m10: 1.0f32,
        m14: 0.0f32,
        m3: 0.0f32,
        m7: 0.0f32,
        m11: 0.0f32,
        m15: 1.0f32,
    }
}

/// Add two matrices
pub fn matrix_add(left: Matrix, right: Matrix) -> Matrix {
    Matrix {
        m0: left.m0 + right.m0,
        m1: left.m1 + right.m1,
        m2: left.m2 + right.m2,
        m3: left.m3 + right.m3,
        m4: left.m4 + right.m4,
        m5: left.m5 + right.m5,
        m6: left.m6 + right.m6,
        m7: left.m7 + right.m7,
        m8: left.m8 + right.m8,
        m9: left.m9 + right.m9,
        m10: left.m10 + right.m10,
        m11: left.m11 + right.m11,
        m12: left.m12 + right.m12,
        m13: left.m13 + right.m13,
        m14: left.m14 + right.m14,
        m15: left.m15 + right.m15,
    }
}

/// Subtract two matrices (left - right)
pub fn matrix_subtract(left: Matrix, right: Matrix) -> Matrix {
    Matrix {
        m0: left.m0 - right.m0,
        m1: left.m1 - right.m1,
        m2: left.m2 - right.m2,
        m3: left.m3 - right.m3,
        m4: left.m4 - right.m4,
        m5: left.m5 - right.m5,
        m6: left.m6 - right.m6,
        m7: left.m7 - right.m7,
        m8: left.m8 - right.m8,
        m9: left.m9 - right.m9,
        m10: left.m10 - right.m10,
        m11: left.m11 - right.m11,
        m12: left.m12 - right.m12,
        m13: left.m13 - right.m13,
        m14: left.m14 - right.m14,
        m15: left.m15 - right.m15,
    }
}

/// Get two matrix multiplication
/// NOTE: When multiplying matrices... the order matters!
pub fn matrix_multiply(left: Matrix, right: Matrix) -> Matrix {
    Matrix {
        m0: left.m0 * right.m0 + left.m1 * right.m4 + left.m2 * right.m8 + left.m3 * right.m12,
        m1: left.m0 * right.m1 + left.m1 * right.m5 + left.m2 * right.m9 + left.m3 * right.m13,
        m2: left.m0 * right.m2 + left.m1 * right.m6 + left.m2 * right.m10 + left.m3 * right.m14,
        m3: left.m0 * right.m3 + left.m1 * right.m7 + left.m2 * right.m11 + left.m3 * right.m15,
        m4: left.m4 * right.m0 + left.m5 * right.m4 + left.m6 * right.m8 + left.m7 * right.m12,
        m5: left.m4 * right.m1 + left.m5 * right.m5 + left.m6 * right.m9 + left.m7 * right.m13,
        m6: left.m4 * right.m2 + left.m5 * right.m6 + left.m6 * right.m10 + left.m7 * right.m14,
        m7: left.m4 * right.m3 + left.m5 * right.m7 + left.m6 * right.m11 + left.m7 * right.m15,
        m8: left.m8 * right.m0 + left.m9 * right.m4 + left.m10 * right.m8 + left.m11 * right.m12,
        m9: left.m8 * right.m1 + left.m9 * right.m5 + left.m10 * right.m9 + left.m11 * right.m13,
        m10: left.m8 * right.m2 + left.m9 * right.m6 + left.m10 * right.m10 + left.m11 * right.m14,
        m11: left.m8 * right.m3 + left.m9 * right.m7 + left.m10 * right.m11 + left.m11 * right.m15,
        m12: left.m12 * right.m0 + left.m13 * right.m4 + left.m14 * right.m8 + left.m15 * right.m12,
        m13: left.m12 * right.m1 + left.m13 * right.m5 + left.m14 * right.m9 + left.m15 * right.m13,
        m14: left.m12 * right.m2
            + left.m13 * right.m6
            + left.m14 * right.m10
            + left.m15 * right.m14,
        m15: left.m12 * right.m3
            + left.m13 * right.m7
            + left.m14 * right.m11
            + left.m15 * right.m15,
    }
}

/// Get translation matrix
pub fn matrix_translate(x: f32, y: f32, z: f32) -> Matrix {
    Matrix {
        m0: 1.0f32,
        m4: 0.0f32,
        m8: 0.0f32,
        m12: x,
        m1: 0.0f32,
        m5: 1.0f32,
        m9: 0.0f32,
        m13: y,
        m2: 0.0f32,
        m6: 0.0f32,
        m10: 1.0f32,
        m14: z,
        m3: 0.0f32,
        m7: 0.0f32,
        m11: 0.0f32,
        m15: 1.0f32,
    }
}

/// Create rotation matrix from axis and angle
/// NOTE: Angle should be provided in radians
pub fn matrix_rotate(axis: Vector3, angle: f32) -> Matrix {
    let mut result = Matrix {
        m0: 0.,
        m4: 0.,
        m8: 0.,
        m12: 0.,
        m1: 0.,
        m5: 0.,
        m9: 0.,
        m13: 0.,
        m2: 0.,
        m6: 0.,
        m10: 0.,
        m14: 0.,
        m3: 0.,
        m7: 0.,
        m11: 0.,
        m15: 0.,
    };
    let mut x: f32 = axis.x;
    let mut y: f32 = axis.y;
    let mut z: f32 = axis.z;
    let length_squared: f32 = x * x + y * y + z * z;
    if length_squared != 1.0f32 && length_squared != 0.0f32 {
        let ilength: f32 = 1.0f32 / length_squared.sqrt();
        x *= ilength;
        y *= ilength;
        z *= ilength;
    }
    let sinres: f32 = angle.sin();
    let cosres: f32 = angle.cos();
    let t: f32 = 1.0f32 - cosres;
    result.m0 = x * x * t + cosres;
    result.m1 = y * x * t + z * sinres;
    result.m2 = z * x * t - y * sinres;
    result.m3 = 0.0f32;
    result.m4 = x * y * t - z * sinres;
    result.m5 = y * y * t + cosres;
    result.m6 = z * y * t + x * sinres;
    result.m7 = 0.0f32;
    result.m8 = x * z * t + y * sinres;
    result.m9 = y * z * t - x * sinres;
    result.m10 = z * z * t + cosres;
    result.m11 = 0.0f32;
    result.m12 = 0.0f32;
    result.m13 = 0.0f32;
    result.m14 = 0.0f32;
    result.m15 = 1.0f32;
    result
}

/// Get x-rotation matrix
/// NOTE: Angle must be provided in radians
pub fn matrix_rotate_x(angle: f32) -> Matrix {
    let mut result = matrix_identity();
    let cosres: f32 = angle.cos();
    let sinres: f32 = angle.sin();
    result.m5 = cosres;
    result.m6 = sinres;
    result.m9 = -sinres;
    result.m10 = cosres;
    result
}

/// Get y-rotation matrix
/// NOTE: Angle must be provided in radians
pub fn matrix_rotate_y(angle: f32) -> Matrix {
    let mut result = matrix_identity();
    let cosres: f32 = angle.cos();
    let sinres: f32 = angle.sin();
    result.m0 = cosres;
    result.m2 = -sinres;
    result.m8 = sinres;
    result.m10 = cosres;
    result
}

/// Get z-rotation matrix
/// NOTE: Angle must be provided in radians
pub fn matrix_rotate_z(angle: f32) -> Matrix {
    let mut result = matrix_identity();
    let cosres: f32 = angle.cos();
    let sinres: f32 = angle.sin();
    result.m0 = cosres;
    result.m1 = sinres;
    result.m4 = -sinres;
    result.m5 = cosres;
    result
}

/// Get xyz-rotation matrix
/// NOTE: Angle must be provided in radians
pub fn matrix_rotate_xyz(angle: Vector3) -> Matrix {
    let mut result = matrix_identity();

    let cosz: f32 = (-angle.z).cos();
    let sinz: f32 = (-angle.z).sin();
    let cosy: f32 = (-angle.y).cos();
    let siny: f32 = (-angle.y).sin();
    let cosx: f32 = (-angle.x).cos();
    let sinx: f32 = (-angle.x).sin();

    result.m0 = cosz * cosy;
    result.m1 = cosz * siny * sinx - sinz * cosx;
    result.m2 = cosz * siny * cosx + sinz * sinx;

    result.m4 = sinz * cosy;
    result.m5 = sinz * siny * sinx + cosz * cosx;
    result.m6 = sinz * siny * cosx - cosz * sinx;

    result.m8 = -siny;
    result.m9 = cosy * sinx;
    result.m10 = cosy * cosx;

    result
}

/// Get zyx-rotation matrix
/// NOTE: Angle must be provided in radians
pub fn matrix_rotate_zyx(angle: Vector3) -> Matrix {
    let mut result = Matrix {
        m0: 0.,
        m4: 0.,
        m8: 0.,
        m12: 0.,
        m1: 0.,
        m5: 0.,
        m9: 0.,
        m13: 0.,
        m2: 0.,
        m6: 0.,
        m10: 0.,
        m14: 0.,
        m3: 0.,
        m7: 0.,
        m11: 0.,
        m15: 0.,
    };
    let cz: f32 = angle.z.cos();
    let sz: f32 = angle.z.sin();
    let cy: f32 = angle.y.cos();
    let sy: f32 = angle.y.sin();
    let cx: f32 = angle.x.cos();
    let sx: f32 = angle.x.sin();
    result.m0 = cz * cy;
    result.m4 = cz * sy * sx - cx * sz;
    result.m8 = sz * sx + cz * cx * sy;
    result.m12 = 0.;
    result.m1 = cy * sz;
    result.m5 = cz * cx + sz * sy * sx;
    result.m9 = cx * sz * sy - cz * sx;
    result.m13 = 0.;
    result.m2 = -sy;
    result.m6 = cy * sx;
    result.m10 = cy * cx;
    result.m14 = 0.;
    result.m3 = 0.;
    result.m7 = 0.;
    result.m11 = 0.;
    result.m15 = 1.;
    result
}

/// Get scaling matrix
pub fn matrix_scale(x: f32, y: f32, z: f32) -> Matrix {
    Matrix {
        m0: x,
        m4: 0.0f32,
        m8: 0.0f32,
        m12: 0.0f32,
        m1: 0.0f32,
        m5: y,
        m9: 0.0f32,
        m13: 0.0f32,
        m2: 0.0f32,
        m6: 0.0f32,
        m10: z,
        m14: 0.0f32,
        m3: 0.0f32,
        m7: 0.0f32,
        m11: 0.0f32,
        m15: 1.0f32,
    }
}

/// Get perspective projection matrix
pub fn matrix_frustum(
    left: f64,
    right: f64,
    bottom: f64,
    top: f64,
    near_plane: f64,
    far_plane: f64,
) -> Matrix {
    let mut result = Matrix {
        m0: 0.,
        m4: 0.,
        m8: 0.,
        m12: 0.,
        m1: 0.,
        m5: 0.,
        m9: 0.,
        m13: 0.,
        m2: 0.,
        m6: 0.,
        m10: 0.,
        m14: 0.,
        m3: 0.,
        m7: 0.,
        m11: 0.,
        m15: 0.,
    };
    let rl: f32 = (right - left) as f32;
    let tb: f32 = (top - bottom) as f32;
    let fn_0: f32 = (far_plane - near_plane) as f32;
    result.m0 = near_plane as f32 * 2.0f32 / rl;
    result.m1 = 0.0f32;
    result.m2 = 0.0f32;
    result.m3 = 0.0f32;
    result.m4 = 0.0f32;
    result.m5 = near_plane as f32 * 2.0f32 / tb;
    result.m6 = 0.0f32;
    result.m7 = 0.0f32;
    result.m8 = (right as f32 + left as f32) / rl;
    result.m9 = (top as f32 + bottom as f32) / tb;
    result.m10 = -(far_plane as f32 + near_plane as f32) / fn_0;
    result.m11 = -1.0f32;
    result.m12 = 0.0f32;
    result.m13 = 0.0f32;
    result.m14 = -(far_plane as f32 * near_plane as f32 * 2.0f32) / fn_0;
    result.m15 = 0.0f32;
    result
}

/// Get perspective projection matrix
/// NOTE: Fovy angle must be provided in radians
pub fn matrix_perspective(fov_y: f64, aspect: f64, near_plane: f64, far_plane: f64) -> Matrix {
    let mut result = Matrix {
        m0: 0.,
        m4: 0.,
        m8: 0.,
        m12: 0.,
        m1: 0.,
        m5: 0.,
        m9: 0.,
        m13: 0.,
        m2: 0.,
        m6: 0.,
        m10: 0.,
        m14: 0.,
        m3: 0.,
        m7: 0.,
        m11: 0.,
        m15: 0.,
    };
    let top: f64 = near_plane * (fov_y * 0.5f64).tan();
    let bottom: f64 = -top;
    let right: f64 = top * aspect;
    let left: f64 = -right;
    let rl: f32 = (right - left) as f32;
    let tb: f32 = (top - bottom) as f32;
    let fn_0: f32 = (far_plane - near_plane) as f32;
    result.m0 = near_plane as f32 * 2.0f32 / rl;
    result.m5 = near_plane as f32 * 2.0f32 / tb;
    result.m8 = (right as f32 + left as f32) / rl;
    result.m9 = (top as f32 + bottom as f32) / tb;
    result.m10 = -(far_plane as f32 + near_plane as f32) / fn_0;
    result.m11 = -1.0f32;
    result.m14 = -(far_plane as f32 * near_plane as f32 * 2.0f32) / fn_0;
    result
}

/// Get orthographic projection matrix
pub fn matrix_ortho(
    left: f64,
    right: f64,
    bottom: f64,
    top: f64,
    near_plane: f64,
    far_plane: f64,
) -> Matrix {
    let mut result = Matrix {
        m0: 0.,
        m4: 0.,
        m8: 0.,
        m12: 0.,
        m1: 0.,
        m5: 0.,
        m9: 0.,
        m13: 0.,
        m2: 0.,
        m6: 0.,
        m10: 0.,
        m14: 0.,
        m3: 0.,
        m7: 0.,
        m11: 0.,
        m15: 0.,
    };
    let rl: f32 = (right - left) as f32;
    let tb: f32 = (top - bottom) as f32;
    let fn_0: f32 = (far_plane - near_plane) as f32;
    result.m0 = 2.0f32 / rl;
    result.m1 = 0.0f32;
    result.m2 = 0.0f32;
    result.m3 = 0.0f32;
    result.m4 = 0.0f32;
    result.m5 = 2.0f32 / tb;
    result.m6 = 0.0f32;
    result.m7 = 0.0f32;
    result.m8 = 0.0f32;
    result.m9 = 0.0f32;
    result.m10 = -2.0f32 / fn_0;
    result.m11 = 0.0f32;
    result.m12 = -(left as f32 + right as f32) / rl;
    result.m13 = -(top as f32 + bottom as f32) / tb;
    result.m14 = -(far_plane as f32 + near_plane as f32) / fn_0;
    result.m15 = 1.0f32;
    result
}

/// Get camera look-at matrix (view matrix)
pub fn matrix_look_at(eye: Vector3, target: Vector3, up: Vector3) -> Matrix {
    let mut result = Matrix {
        m0: 0.,
        m4: 0.,
        m8: 0.,
        m12: 0.,
        m1: 0.,
        m5: 0.,
        m9: 0.,
        m13: 0.,
        m2: 0.,
        m6: 0.,
        m10: 0.,
        m14: 0.,
        m3: 0.,
        m7: 0.,
        m11: 0.,
        m15: 0.,
    };
    let mut vz = Vector3 {
        x: eye.x - target.x,
        y: eye.y - target.y,
        z: eye.z - target.z,
    };
    let mut v: Vector3 = vz;
    let mut length = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
    if length == 0.0f32 {
        length = 1.0f32;
    }
    let mut ilength = 1.0f32 / length;
    vz.x *= ilength;
    vz.y *= ilength;
    vz.z *= ilength;
    let mut vx = Vector3 {
        x: up.y * vz.z - up.z * vz.y,
        y: up.z * vz.x - up.x * vz.z,
        z: up.x * vz.y - up.y * vz.x,
    };
    v = vx;
    length = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
    if length == 0.0f32 {
        length = 1.0f32;
    }
    ilength = 1.0f32 / length;
    vx.x *= ilength;
    vx.y *= ilength;
    vx.z *= ilength;
    let vy = Vector3 {
        x: vz.y * vx.z - vz.z * vx.y,
        y: vz.z * vx.x - vz.x * vx.z,
        z: vz.x * vx.y - vz.y * vx.x,
    };
    result.m0 = vx.x;
    result.m1 = vy.x;
    result.m2 = vz.x;
    result.m3 = 0.0f32;
    result.m4 = vx.y;
    result.m5 = vy.y;
    result.m6 = vz.y;
    result.m7 = 0.0f32;
    result.m8 = vx.z;
    result.m9 = vy.z;
    result.m10 = vz.z;
    result.m11 = 0.0f32;
    result.m12 = -(vx.x * eye.x + vx.y * eye.y + vx.z * eye.z);
    result.m13 = -(vy.x * eye.x + vy.y * eye.y + vy.z * eye.z);
    result.m14 = -(vz.x * eye.x + vz.y * eye.y + vz.z * eye.z);
    result.m15 = 1.0f32;
    result
}

/// Get float array of matrix data
pub fn matrix_to_float(mat: Matrix) -> [f32; 16] {
    matrix_to_float_v(mat).v
}

/// Get float array of matrix data
pub fn matrix_to_float_v(mat: Matrix) -> Float16 {
    Float16 {
        v: [
            mat.m0, mat.m1, mat.m2, mat.m3, mat.m4, mat.m5, mat.m6, mat.m7, mat.m8, mat.m9,
            mat.m10, mat.m11, mat.m12, mat.m13, mat.m14, mat.m15,
        ],
    }
}

//----------------------------------------------------------------------------------
// Module Functions Definition - Quaternion math
//----------------------------------------------------------------------------------

/// Add two quaternions
pub fn quaternion_add(q1: Quaternion, q2: Quaternion) -> Quaternion {
    Quaternion {
        x: q1.x + q2.x,
        y: q1.y + q2.y,
        z: q1.z + q2.z,
        w: q1.w + q2.w,
    }
}

/// Add quaternion and float value
pub fn quaternion_add_value(q: Quaternion, add: f32) -> Quaternion {
    Quaternion {
        x: q.x + add,
        y: q.y + add,
        z: q.z + add,
        w: q.w + add,
    }
}

/// Subtract two quaternions
pub fn quaternion_subtract(q1: Quaternion, q2: Quaternion) -> Quaternion {
    Quaternion {
        x: q1.x - q2.x,
        y: q1.y - q2.y,
        z: q1.z - q2.z,
        w: q1.w - q2.w,
    }
}

/// Subtract quaternion and float value
pub fn quaternion_subtract_value(q: Quaternion, sub: f32) -> Quaternion {
    Quaternion {
        x: q.x - sub,
        y: q.y - sub,
        z: q.z - sub,
        w: q.w - sub,
    }
}

/// Get identity quaternion
pub fn quaternion_identity() -> Quaternion {
    Quaternion {
        x: 0.0f32,
        y: 0.0f32,
        z: 0.0f32,
        w: 1.0f32,
    }
}

/// Computes the length of a quaternion
pub fn quaternion_length(q: Quaternion) -> f32 {
    (q.x * q.x + q.y * q.y + q.z * q.z + q.w * q.w).sqrt()
}

/// Normalize provided quaternion
pub fn quaternion_normalize(q: Quaternion) -> Quaternion {
    let mut length: f32 = vector4_length(q);
    if length == 0.0f32 {
        length = 1.0f32;
    }
    let ilength: f32 = 1.0f32 / length;

    vector4_scale(q, ilength)
}

/// Invert provided quaternion
pub fn quaternion_invert(q: Quaternion) -> Quaternion {
    let mut result: Quaternion = q;
    let length_sq: f32 = vector4_length_sqr(q);
    if length_sq != 0.0f32 {
        let inv_length: f32 = 1.0f32 / length_sq;
        result.x *= -inv_length;
        result.y *= -inv_length;
        result.z *= -inv_length;
        result.w *= inv_length;
    }
    result
}

/// Calculate two quaternion multiplication
pub fn quaternion_multiply(q1: Quaternion, q2: Quaternion) -> Quaternion {
    let mut result = Quaternion {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    let qax: f32 = q1.x;
    let qay: f32 = q1.y;
    let qaz: f32 = q1.z;
    let qaw: f32 = q1.w;
    let qbx: f32 = q2.x;
    let qby: f32 = q2.y;
    let qbz: f32 = q2.z;
    let qbw: f32 = q2.w;
    result.x = qax * qbw + qaw * qbx + qay * qbz - qaz * qby;
    result.y = qay * qbw + qaw * qby + qaz * qbx - qax * qbz;
    result.z = qaz * qbw + qaw * qbz + qax * qby - qay * qbx;
    result.w = qaw * qbw - qax * qbx - qay * qby - qaz * qbz;
    result
}

/// Scale quaternion by float value
pub fn quaternion_scale(q: Quaternion, mul: f32) -> Quaternion {
    Quaternion {
        x: q.x * mul,
        y: q.y * mul,
        z: q.z * mul,
        w: q.w * mul,
    }
}

/// Divide two quaternions
pub fn quaternion_divide(q1: Quaternion, q2: Quaternion) -> Quaternion {
    Quaternion {
        x: q1.x / q2.x,
        y: q1.y / q2.y,
        z: q1.z / q2.z,
        w: q1.w / q2.w,
    }
}

/// Calculate linear interpolation between two quaternions
pub fn quaternion_lerp(q1: Quaternion, q2: Quaternion, amount: f32) -> Quaternion {
    Quaternion {
        x: q1.x + amount * (q2.x - q1.x),
        y: q1.y + amount * (q2.y - q1.y),
        z: q1.z + amount * (q2.z - q1.z),
        w: q1.w + amount * (q2.w - q1.w),
    }
}

/// Calculate slerp-optimized interpolation between two quaternions
pub fn quaternion_nlerp(q1: Quaternion, q2: Quaternion, amount: f32) -> Quaternion {
    quaternion_normalize(quaternion_lerp(q1, q2, amount))
}

/// Calculates spherical linear interpolation between two quaternions
pub fn quaternion_slerp(q1: Quaternion, q2: Quaternion, amount: f32) -> Quaternion {
    let cos_half_theta = q1.x * q2.x + q1.y * q2.y + q1.z * q2.z + q1.w * q2.w;

    if cos_half_theta.abs() >= 1.0 {
        q1
    } else if cos_half_theta > 0.95 {
        quaternion_nlerp(q1, q2, amount)
    } else {
        let half_theta = cos_half_theta.acos();
        let sin_half_theta = (1.0 - cos_half_theta * cos_half_theta).sqrt();

        if sin_half_theta.abs() < 0.001 {
            Quaternion {
                x: (q1.x * 0.5 + q2.x * 0.5),
                y: (q1.y * 0.5 + q2.y * 0.5),
                z: (q1.z * 0.5 + q2.z * 0.5),
                w: (q1.w * 0.5 + q2.w * 0.5),
            }
        } else {
            let ratio_a = ((1.0 - amount) * half_theta).sin() / sin_half_theta;
            let ratio_b = (amount * half_theta).sin() / sin_half_theta;

            Quaternion {
                x: (q1.x * ratio_a + q2.x * ratio_b),
                y: (q1.y * ratio_a + q2.y * ratio_b),
                z: (q1.z * ratio_a + q2.z * ratio_b),
                w: (q1.w * ratio_a + q2.w * ratio_b),
            }
        }
    }
}

/// Calculate quaternion cubic spline interpolation using Cubic Hermite Spline algorithm
///
/// as described in the GLTF 2.0 specification: https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#interpolation-cubic
pub fn quaternion_cubic_hermite_spline(
    q1: Quaternion,
    out_tangent1: Quaternion,
    q2: Quaternion,
    in_tangent2: Quaternion,
    t: f32,
) -> Quaternion {
    let t2: f32 = t * t;
    let t3: f32 = t2 * t;
    let h00: f32 = 2. * t3 - 3. * t2 + 1.;
    let h10: f32 = t3 - 2. * t2 + t;
    let h01: f32 = -2. * t3 + 3. * t2;
    let h11: f32 = t3 - t2;
    let p0: Quaternion = quaternion_scale(q1, h00);
    let m0: Quaternion = quaternion_scale(out_tangent1, h10);
    let p1: Quaternion = quaternion_scale(q2, h01);
    let m1: Quaternion = quaternion_scale(in_tangent2, h11);
    let mut result;
    result = quaternion_add(p0, m0);
    result = quaternion_add(result, p1);
    result = quaternion_add(result, m1);
    result = quaternion_normalize(result);
    result
}

/// Calculate quaternion based on the rotation from one vector to another
pub fn quaternion_from_vector3_to_vector3(from: Vector3, to: Vector3) -> Quaternion {
    let cross = vector3_cross_product(from, to);
    vector4_normalize(Quaternion {
        x: cross.x,
        y: cross.y,
        z: cross.z,
        w: 1.0 + vector3_dot_product(from, to),
    })
}

/// Get a quaternion for a given rotation matrix
pub fn quaternion_from_matrix(mat: Matrix) -> Quaternion {
    let mut result = Quaternion {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    let four_wsquared_minus1: f32 = mat.m0 + mat.m5 + mat.m10;
    let four_xsquared_minus1: f32 = mat.m0 - mat.m5 - mat.m10;
    let four_ysquared_minus1: f32 = mat.m5 - mat.m0 - mat.m10;
    let four_zsquared_minus1: f32 = mat.m10 - mat.m0 - mat.m5;
    let mut biggest_index = 0;
    let mut four_biggest_squared_minus1: f32 = four_wsquared_minus1;
    if four_xsquared_minus1 > four_biggest_squared_minus1 {
        four_biggest_squared_minus1 = four_xsquared_minus1;
        biggest_index = 1;
    }
    if four_ysquared_minus1 > four_biggest_squared_minus1 {
        four_biggest_squared_minus1 = four_ysquared_minus1;
        biggest_index = 2;
    }
    if four_zsquared_minus1 > four_biggest_squared_minus1 {
        four_biggest_squared_minus1 = four_zsquared_minus1;
        biggest_index = 3;
    }
    let biggest_val: f32 = (four_biggest_squared_minus1 + 1.0f32).sqrt() * 0.5f32;
    let mult: f32 = 0.25f32 / biggest_val;
    match biggest_index {
        0 => {
            result.w = biggest_val;
            result.x = (mat.m6 - mat.m9) * mult;
            result.y = (mat.m8 - mat.m2) * mult;
            result.z = (mat.m1 - mat.m4) * mult;
        }
        1 => {
            result.x = biggest_val;
            result.w = (mat.m6 - mat.m9) * mult;
            result.y = (mat.m1 + mat.m4) * mult;
            result.z = (mat.m8 + mat.m2) * mult;
        }
        2 => {
            result.y = biggest_val;
            result.w = (mat.m8 - mat.m2) * mult;
            result.x = (mat.m1 + mat.m4) * mult;
            result.z = (mat.m6 + mat.m9) * mult;
        }
        3 => {
            result.z = biggest_val;
            result.w = (mat.m1 - mat.m4) * mult;
            result.x = (mat.m8 + mat.m2) * mult;
            result.y = (mat.m6 + mat.m9) * mult;
        }
        _ => {}
    }
    result
}

/// Get a matrix for a given quaternion
pub fn quaternion_to_matrix(q: Quaternion) -> Matrix {
    let mut result = matrix_identity();
    let a2: f32 = q.x * q.x;
    let b2: f32 = q.y * q.y;
    let c2: f32 = q.z * q.z;
    let ac: f32 = q.x * q.z;
    let ab: f32 = q.x * q.y;
    let bc: f32 = q.y * q.z;
    let ad: f32 = q.w * q.x;
    let bd: f32 = q.w * q.y;
    let cd: f32 = q.w * q.z;
    result.m0 = 1. - 2. * (b2 + c2);
    result.m1 = 2. * (ab + cd);
    result.m2 = 2. * (ac - bd);
    result.m4 = 2. * (ab - cd);
    result.m5 = 1. - 2. * (a2 + c2);
    result.m6 = 2. * (bc + ad);
    result.m8 = 2. * (ac + bd);
    result.m9 = 2. * (bc - ad);
    result.m10 = 1. - 2. * (a2 + b2);
    result
}

/// Get rotation quaternion for an angle and axis
/// NOTE: Angle must be provided in radians
pub fn quaternion_from_axis_angle(axis: Vector3, mut angle: f32) -> Quaternion {
    let mut result = Quaternion {
        x: 0.0f32,
        y: 0.0f32,
        z: 0.0f32,
        w: 1.0f32,
    };
    let axis_length: f32 = (axis.x * axis.x + axis.y * axis.y + axis.z * axis.z).sqrt();
    if axis_length != 0.0f32 {
        angle *= 0.5f32;

        let axis = vector3_normalize(axis);

        let sinres: f32 = angle.sin();
        let cosres: f32 = angle.cos();
        result.x = axis.x * sinres;
        result.y = axis.y * sinres;
        result.z = axis.z * sinres;
        result.w = cosres;

        result = quaternion_normalize(result);
    }
    result
}

/// Get the rotation angle and axis for a given quaternion
pub fn quaternion_to_axis_angle(mut q: Quaternion) -> (Vector3, f32) {
    if q.w.abs() > 1.0f32 {
        q = quaternion_normalize(q);
    }
    let mut res_axis = vector3_zero();
    let res_angle = 2.0 * (q.w.acos());
    let den: f32 = (1. - q.w * q.w).sqrt();
    if den > f32::EPSILON {
        res_axis.x = q.x / den;
        res_axis.y = q.y / den;
        res_axis.z = q.z / den;
    } else {
        // This occurs when the angle is zero.
        // Not a problem: just set an arbitrary normalized axis.
        res_axis.x = 1.0;
    }

    (res_axis, res_angle)
}

/// Get the quaternion equivalent to Euler angles
/// NOTE: Rotation order is ZYX
pub fn quaternion_from_euler(pitch: f32, yaw: f32, roll: f32) -> Quaternion {
    let mut result = Quaternion {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    let x0: f32 = (pitch * 0.5f32).cos();
    let x1: f32 = (pitch * 0.5f32).sin();
    let y0: f32 = (yaw * 0.5f32).cos();
    let y1: f32 = (yaw * 0.5f32).sin();
    let z0: f32 = (roll * 0.5f32).cos();
    let z1: f32 = (roll * 0.5f32).sin();
    result.x = x1 * y0 * z0 - x0 * y1 * z1;
    result.y = x0 * y1 * z0 + x1 * y0 * z1;
    result.z = x0 * y0 * z1 - x1 * y1 * z0;
    result.w = x0 * y0 * z0 + x1 * y1 * z1;
    result
}

/// Get the Euler angles equivalent to quaternion (roll, pitch, yaw)
///
/// NOTE: Angles are returned in a Vector3 struct in radians
pub fn quaternion_to_euler(q: Quaternion) -> Vector3 {
    let mut result = Vector3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };

    // Roll (x-axis rotation)
    let x0: f32 = 2.0f32 * (q.w * q.x + q.y * q.z);
    let x1: f32 = 1.0f32 - 2.0f32 * (q.x * q.x + q.y * q.y);
    result.x = x0.atan2(x1);

    // Pitch (y-axis rotation)
    let mut y0: f32 = 2.0f32 * (q.w * q.y - q.z * q.x);
    y0 = if y0 > 1.0f32 { 1.0f32 } else { y0 };
    y0 = if y0 < -1.0f32 { -1.0f32 } else { y0 };
    result.y = y0.asin();

    // Yaw (z-axis rotation)
    let z0: f32 = 2.0f32 * (q.w * q.z + q.x * q.y);
    let z1: f32 = 1.0f32 - 2.0f32 * (q.y * q.y + q.z * q.z);
    result.z = z0.atan2(z1);
    result
}

/// Transform a quaternion given a transformation matrix
pub fn quaternion_transform(q: Quaternion, mat: Matrix) -> Quaternion {
    Quaternion {
        x: mat.m0 * q.x + mat.m4 * q.y + mat.m8 * q.z + mat.m12 * q.w,
        y: mat.m1 * q.x + mat.m5 * q.y + mat.m9 * q.z + mat.m13 * q.w,
        z: mat.m2 * q.x + mat.m6 * q.y + mat.m10 * q.z + mat.m14 * q.w,
        w: mat.m3 * q.x + mat.m7 * q.y + mat.m11 * q.z + mat.m15 * q.w,
    }
}

/// Check whether two given quaternions are almost equal
pub fn quaternion_equals(p: Quaternion, q: Quaternion) -> bool {
    vector4_equals(p, q)
}

/// Decompose a transformation matrix into its rotational, translational and scaling components
pub fn matrix_decompose(mat: Matrix) -> (Vector3, Quaternion, Vector3) {
    let translation = Vector3 {
        x: mat.m12,
        y: mat.m13,
        z: mat.m14,
    };

    let a = mat.m0;
    let b = mat.m4;
    let c = mat.m8;
    let d = mat.m1;
    let e = mat.m5;
    let f = mat.m9;
    let g = mat.m2;
    let h = mat.m6;
    let i = mat.m10;
    let det_a = e * i - f * h;
    let det_b = f * g - d * i;
    let det_c = d * h - e * g;

    let det: f32 = a * det_a + b * det_b + c * det_c;
    let abc = Vector3 { x: a, y: b, z: c };
    let def = Vector3 { x: d, y: e, z: f };
    let ghi = Vector3 { x: g, y: h, z: i };

    let mut scale = Vector3 {
        x: vector3_length(abc),
        y: vector3_length(def),
        z: vector3_length(ghi),
    };
    if det < 0. {
        scale = vector3_negate(scale);
    }

    let mut clone: Matrix = mat;
    let mut rotation = quaternion_identity();
    if !float_equals(det, 0.) {
        clone.m0 /= scale.x;
        clone.m5 /= scale.y;
        clone.m10 /= scale.z;
        rotation = quaternion_from_matrix(clone);
    }

    (translation, rotation, scale)
}

// ==============================================================================

// Ray, ray for raycasting
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    // Ray position (origin)
    pub position: Vector3,
    // Ray direction (normalized)
    pub direction: Vector3,
}

// Rectangle, 4 components
#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

// deleteme
impl From<[f32; 5]> for Rectangle {
    fn from(f: [f32; 5]) -> Self {
        Self {
            x: f[0],
            y: f[1],
            width: f[2],
            height: f[3],
        }
    }
}

// BoundingBox
pub struct BoundingBox {
    // Minimum vertex box-corner
    pub min: Vector3,
    // Maximum vertex box-corner
    pub max: Vector3,
}

// RayCollision, ray hit information
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RayCollision {
    // Did the ray hit something?
    pub hit: bool,
    // Distance to the nearest hit
    pub distance: f32,
    // Point of the nearest hit
    pub point: Vector3,
    // Surface normal of hit
    pub normal: Vector3,
}

//----------------------------------------------------------------------------------
// Module Functions Definition - Collision Detection functions
//----------------------------------------------------------------------------------

// Check if point is inside rectangle
pub fn check_collision_point_rec(point: Vector2, rec: Rectangle) -> bool {
    (point.x >= rec.x)
        && (point.x < (rec.x + rec.width))
        && (point.y >= rec.y)
        && (point.y < (rec.y + rec.height))
}

// Check if point is inside circle
pub fn check_collision_point_circle(point: Vector2, center: Vector2, radius: f32) -> bool {
    let mut collision = false;

    let distance_squared =
        (point.x - center.x) * (point.x - center.x) + (point.y - center.y) * (point.y - center.y);

    if distance_squared <= radius * radius {
        collision = true;
    }

    return collision;
}

// Check if point is inside a triangle defined by three points (p1, p2, p3)
pub fn check_collision_point_triangle(
    point: Vector2,
    p1: Vector2,
    p2: Vector2,
    p3: Vector2,
) -> bool {
    let alpha = ((p2.y - p3.y) * (point.x - p3.x) + (p3.x - p2.x) * (point.y - p3.y))
        / ((p2.y - p3.y) * (p1.x - p3.x) + (p3.x - p2.x) * (p1.y - p3.y));

    let beta = ((p3.y - p1.y) * (point.x - p3.x) + (p1.x - p3.x) * (point.y - p3.y))
        / ((p2.y - p3.y) * (p1.x - p3.x) + (p3.x - p2.x) * (p1.y - p3.y));

    let gamma = 1. - alpha - beta;

    (alpha > 0.) && (beta > 0.) && (gamma > 0.)
}

// Check if point is within a polygon described by array of vertices
// NOTE: Based on http://jeffreythompson.org/collision-detection/poly-point.php
pub fn check_collision_point_poly(point: Vector2, points: Vec<Vector2>) -> bool {
    let mut inside = false;

    let point_count = points.len();

    if point_count > 2 {
        let mut j = point_count - 1;
        for i in 0..point_count {
            if (points[i].y > point.y) != (points[j].y > point.y)
                && (point.x
                    < (points[j].x - points[i].x) * (point.y - points[i].y)
                        / (points[j].y - points[i].y)
                        + points[i].x)
            {
                inside = !inside;
            }

            j = i;
        }
    }

    return inside;
}

// Check collision between two rectangles
pub fn check_collision_recs(rec1: Rectangle, rec2: Rectangle) -> bool {
    (rec1.x < (rec2.x + rec2.width) && (rec1.x + rec1.width) > rec2.x)
        && (rec1.y < (rec2.y + rec2.height) && (rec1.y + rec1.height) > rec2.y)
}

// Check collision between two circles
pub fn check_collision_circles(
    center1: Vector2,
    radius1: f32,
    center2: Vector2,
    radius2: f32,
) -> bool {
    let dx = center2.x - center1.x; // X distance between centers
    let dy = center2.y - center1.y; // Y distance between centers

    let distance_squared = dx * dx + dy * dy; // Distance between centers squared
    let radius_sum = radius1 + radius2;

    distance_squared <= (radius_sum * radius_sum)
}

// Check collision between circle and rectangle
// NOTE: Reviewed version to take into account corner limit case
pub fn check_collision_circle_rec(center: Vector2, radius: f32, rec: Rectangle) -> bool {
    let rec_center_x = rec.x + rec.width / 2.0;
    let rec_center_y = rec.y + rec.height / 2.0;

    let dx = (center.x - rec_center_x).abs();
    let dy = (center.y - rec_center_y).abs();

    if dx > (rec.width / 2.0 + radius) {
        return false;
    }
    if dy > (rec.height / 2.0 + radius) {
        return false;
    }

    if dx <= (rec.width / 2.0) {
        return true;
    }
    if dy <= (rec.height / 2.0) {
        return true;
    }

    let corner_distance_sq = (dx - rec.width / 2.0) * (dx - rec.width / 2.0)
        + (dy - rec.height / 2.0) * (dy - rec.height / 2.0);

    corner_distance_sq <= (radius * radius)
}

// Check the collision between two lines defined by two points each, returns collision point by reference
pub fn check_collision_lines(
    start_pos1: Vector2,
    end_pos1: Vector2,
    start_pos2: Vector2,
    end_pos2: Vector2,
) -> Option<Vector2> {
    let div = (end_pos2.y - start_pos2.y) * (end_pos1.x - start_pos1.x)
        - (end_pos2.x - start_pos2.x) * (end_pos1.y - start_pos1.y);

    if div.abs() >= f32::EPSILON {
        let xi = ((start_pos2.x - end_pos2.x)
            * (start_pos1.x * end_pos1.y - start_pos1.y * end_pos1.x)
            - (start_pos1.x - end_pos1.x)
                * (start_pos2.x * end_pos2.y - start_pos2.y * end_pos2.x))
            / div;
        let yi = ((start_pos2.y - end_pos2.y)
            * (start_pos1.x * end_pos1.y - start_pos1.y * end_pos1.x)
            - (start_pos1.y - end_pos1.y)
                * (start_pos2.x * end_pos2.y - start_pos2.y * end_pos2.x))
            / div;

        if (((start_pos1.x - end_pos1.x).abs() > f32::EPSILON)
            && (xi < start_pos1.x.min(end_pos1.x) || (xi > start_pos1.x.max(end_pos1.x))))
            || (((start_pos2.x - end_pos2.x).abs() > f32::EPSILON)
                && (xi < start_pos2.x.min(end_pos2.x) || (xi > start_pos2.x.max(end_pos2.x))))
            || (((start_pos1.y - end_pos1.y).abs() > f32::EPSILON)
                && (yi < start_pos1.y.min(end_pos1.y) || (yi > start_pos1.y.max(end_pos1.y))))
            || (((start_pos2.y - end_pos2.y).abs() > f32::EPSILON)
                && (yi < start_pos2.y.min(end_pos2.y) || (yi > start_pos2.y.max(end_pos2.y))))
        {
            None
        } else {
            Some(Vector2 { x: xi, y: yi })
        }
    } else {
        None
    }
}

// Check if point belongs to line created between two points [p1] and [p2] with defined margin in pixels [threshold]
pub fn check_collision_point_line(
    point: Vector2,
    p1: Vector2,
    p2: Vector2,
    threshold: usize,
) -> bool {
    let mut collision = false;
    let threshold = threshold as f32;

    let dxc = point.x - p1.x;
    let dyc = point.y - p1.y;
    let dxl = p2.x - p1.x;
    let dyl = p2.y - p1.y;
    let cross = dxc * dyl - dyc * dxl;

    if cross.abs() < (threshold * dxl.abs().max(dyl.abs())) {
        if dxl.abs() >= dyl.abs() {
            collision = if dxl > 0. {
                (p1.x <= point.x) && (point.x <= p2.x)
            } else {
                (p2.x <= point.x) && (point.x <= p1.x)
            }
        } else {
            collision = if dyl > 0. {
                (p1.y <= point.y) && (point.y <= p2.y)
            } else {
                (p2.y <= point.y) && (point.y <= p1.y)
            }
        }
    }

    return collision;
}

// Check if circle collides with a line created betweeen two points [p1] and [p2]
pub fn check_collision_circle_line(center: Vector2, radius: f32, p1: Vector2, p2: Vector2) -> bool {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;

    if (dx.abs() + dy.abs()) <= f32::EPSILON {
        return check_collision_circles(p1, 0., center, radius);
    }

    let length_sq = (dx * dx) + (dy * dy);
    let mut dot_product =
        (((center.x - p1.x) * (p2.x - p1.x)) + ((center.y - p1.y) * (p2.y - p1.y))) / (length_sq);

    if dot_product > 1.0 {
        dot_product = 1.0;
    } else if dot_product < 0.0 {
        dot_product = 0.0;
    }

    let dx2 = (p1.x - (dot_product * (dx))) - center.x;
    let dy2 = (p1.y - (dot_product * (dy))) - center.y;
    let distance_sq = (dx2 * dx2) + (dy2 * dy2);

    distance_sq <= radius * radius
}

// Get collision rectangle for two rectangles collision
pub fn get_collision_rec(rec1: Rectangle, rec2: Rectangle) -> Rectangle {
    let mut overlap = Rectangle {
        x: 0.,
        y: 0.,
        width: 0.,
        height: 0.,
    };

    let left = if rec1.x > rec2.x { rec1.x } else { rec2.x };
    let right1 = rec1.x + rec1.width;
    let right2 = rec2.x + rec2.width;
    let right = if right1 < right2 { right1 } else { right2 };
    let top = if rec1.y > rec2.y { rec1.y } else { rec2.y };
    let bottom1 = rec1.y + rec1.height;
    let bottom2 = rec2.y + rec2.height;
    let bottom = if bottom1 < bottom2 { bottom1 } else { bottom2 };

    if (left < right) && (top < bottom) {
        overlap.x = left;
        overlap.y = top;
        overlap.width = right - left;
        overlap.height = bottom - top;
    }

    return overlap;
}

// Check collision between two spheres
pub fn check_collision_spheres(
    center1: Vector3,
    radius1: f32,
    center2: Vector3,
    radius2: f32,
) -> bool {
    // Simple way to check for collision, just checking distance between two points
    // Unfortunately, sqrtf() is a costly operation, so we avoid it with following solution
    /*
    float dx = center1.x - center2.x;      // X distance between centers
    float dy = center1.y - center2.y;      // Y distance between centers
    float dz = center1.z - center2.z;      // Z distance between centers

    float distance = sqrtf(dx*dx + dy*dy + dz*dz);  // Distance between centers

    if (distance <= (radius1 + radius2)) collision = true;
    */

    // Check for distances squared to avoid sqrtf()
    vector3_dot_product(
        vector3_subtract(center2, center1),
        vector3_subtract(center2, center1),
    ) <= (radius1 + radius2) * (radius1 + radius2)
}

// Check collision between two boxes
// NOTE: Boxes are defined by two points minimum and maximum
pub fn check_collision_boxes(box1: BoundingBox, box2: BoundingBox) -> bool {
    let mut collision = true;

    if (box1.max.x >= box2.min.x) && (box1.min.x <= box2.max.x) {
        if (box1.max.y < box2.min.y) || (box1.min.y > box2.max.y) {
            collision = false;
        }
        if (box1.max.z < box2.min.z) || (box1.min.z > box2.max.z) {
            collision = false;
        }
    } else {
        collision = false;
    }

    return collision;
}

// Check collision between box and sphere
pub fn check_collision_box_sphere(bbox: BoundingBox, center: Vector3, radius: f32) -> bool {
    let mut dmin = 0.;

    if center.x < bbox.min.x {
        dmin += (center.x - bbox.min.x).powi(2);
    } else if center.x > bbox.max.x {
        dmin += (center.x - bbox.max.x).powi(2);
    }

    if center.y < bbox.min.y {
        dmin += (center.y - bbox.min.y).powi(2);
    } else if center.y > bbox.max.y {
        dmin += (center.y - bbox.max.y).powi(2);
    }

    if center.z < bbox.min.z {
        dmin += (center.z - bbox.min.z).powi(2);
    } else if center.z > bbox.max.z {
        dmin += (center.z - bbox.max.z).powi(2);
    }

    if dmin <= (radius * radius) {
        true
    } else {
        false
    }
}

// Get collision info between ray and sphere
pub fn get_ray_collision_sphere(ray: Ray, center: Vector3, radius: f32) -> RayCollision {
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

    let ray_sphere_pos = vector3_subtract(center, ray.position);
    let vector = vector3_dot_product(ray_sphere_pos, ray.direction);
    let distance = vector3_length(ray_sphere_pos);
    let d = radius * radius - (distance * distance - vector * vector);

    collision.hit = d >= 0.;

    // Check if ray origin is inside the sphere to calculate the correct collision point
    if distance < radius {
        collision.distance = vector + d.sqrt();

        // Calculate collision point
        collision.point = vector3_add(
            ray.position,
            vector3_scale(ray.direction, collision.distance),
        );

        // Calculate collision normal (pointing outwards)
        collision.normal =
            vector3_negate(vector3_normalize(vector3_subtract(collision.point, center)));
    } else {
        collision.distance = vector - d.sqrt();

        // Calculate collision point
        collision.point = vector3_add(
            ray.position,
            vector3_scale(ray.direction, collision.distance),
        );

        // Calculate collision normal (pointing inwards)
        collision.normal = vector3_normalize(vector3_subtract(collision.point, center));
    }

    return collision;
}

// Get collision info between ray and box
pub fn get_ray_collision_box(mut ray: Ray, bbox: BoundingBox) -> RayCollision {
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

    // Note: If ray.position is inside the box, the distance is negative (as if the ray was reversed)
    // Reversing ray.direction will give use the correct result
    let inside_box = (ray.position.x > bbox.min.x)
        && (ray.position.x < bbox.max.x)
        && (ray.position.y > bbox.min.y)
        && (ray.position.y < bbox.max.y)
        && (ray.position.z > bbox.min.z)
        && (ray.position.z < bbox.max.z);

    if inside_box {
        ray.direction = vector3_negate(ray.direction);
    }

    let mut t: [f32; 11] = [0.; 11];

    t[8] = 1.0 / ray.direction.x;
    t[9] = 1.0 / ray.direction.y;
    t[10] = 1.0 / ray.direction.z;

    t[0] = (bbox.min.x - ray.position.x) * t[8];
    t[1] = (bbox.max.x - ray.position.x) * t[8];
    t[2] = (bbox.min.y - ray.position.y) * t[9];
    t[3] = (bbox.max.y - ray.position.y) * t[9];
    t[4] = (bbox.min.z - ray.position.z) * t[10];
    t[5] = (bbox.max.z - ray.position.z) * t[10];
    t[6] = f32::max(
        f32::max(f32::min(t[0], t[1]), f32::min(t[2], t[3])),
        f32::min(t[4], t[5]),
    );
    t[7] = f32::min(
        f32::min(f32::max(t[0], t[1]), f32::max(t[2], t[3])),
        f32::max(t[4], t[5]),
    );

    collision.hit = !((t[7] < 0.) || (t[6] > t[7]));

    if !collision.hit {
        return collision;
    }

    collision.distance = t[6];

    if collision.distance.is_nan() || collision.distance.is_infinite() {
        return collision;
    }

    collision.point = vector3_add(
        ray.position,
        vector3_scale(ray.direction, collision.distance),
    );

    // Get box center point
    collision.normal = vector3_lerp(bbox.min, bbox.max, 0.5);
    // Get vector center point->hit point
    collision.normal = vector3_subtract(collision.point, collision.normal);
    // Scale vector to unit cube
    // NOTE: We use an additional .01 to fix numerical errors
    collision.normal = vector3_scale(collision.normal, 2.01);
    collision.normal = vector3_divide(collision.normal, vector3_subtract(bbox.max, bbox.min));
    // The relevant elements of the vector are now slightly larger than 1.0f (or smaller than -1.0f)
    // and the others are somewhere between -1.0 and 1.0 casting to int is exactly our wanted normal!
    collision.normal.x = collision.normal.x as i32 as f32;
    collision.normal.y = collision.normal.y as i32 as f32;
    collision.normal.z = collision.normal.z as i32 as f32;

    collision.normal = vector3_normalize(collision.normal);

    if inside_box {
        // Reset ray.direction
        ray.direction = vector3_negate(ray.direction);
        // Fix result
        collision.distance *= -1.0;
        collision.normal = vector3_negate(collision.normal);
    }

    return collision;
}

// Get collision info between ray and mesh
pub fn get_ray_collision_mesh(
    ray: Ray,
    mesh: Vec<[Vector3; 3]>,
    transform: Matrix,
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

        let tri_hit_info = get_ray_collision_triangle(ray.clone(), a, b, c);

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
pub fn get_ray_collision_triangle(ray: Ray, p1: Vector3, p2: Vector3, p3: Vector3) -> RayCollision {
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
        collision.normal = vector3_normalize(vector3_cross_product(edge1, edge2));
        collision.point = vector3_add(ray.position, vector3_scale(ray.direction, t));
    }

    return collision;
}

// Get collision info between ray and quad
// NOTE: The points are expected to be in counter-clockwise winding
pub fn get_ray_collision_quad(
    ray: Ray,
    p1: Vector3,
    p2: Vector3,
    p3: Vector3,
    p4: Vector3,
) -> RayCollision {
    let mut collision = get_ray_collision_triangle(ray, p1, p2, p4);

    if !collision.hit {
        collision = get_ray_collision_triangle(ray, p2, p3, p4);
    }

    return collision;
}

// =========================================================

// todo -- deleteme
impl From<[f32; 3]> for Vector2 {
    fn from(f: [f32; 3]) -> Self {
        Vector2 { x: f[0], y: f[1] }
    }
}

// todo -- deleteme
impl From<[f32; 5]> for Vector3 {
    fn from(f: [f32; 5]) -> Self {
        Vector3 {
            x: f[0],
            y: f[1],
            z: f[2],
        }
    }
}

// todo -- deleteme
impl From<[f32; 16]> for Vector3 {
    fn from(f: [f32; 16]) -> Self {
        Vector3 {
            x: f[0],
            y: f[1],
            z: f[2],
        }
    }
}

// todo -- deleteme
impl From<[f32; 5]> for Quaternion {
    fn from(f: [f32; 5]) -> Self {
        Quaternion {
            x: f[0],
            y: f[1],
            z: f[2],
            w: f[3],
        }
    }
}

// todo -- deleteme
impl From<[f32; 16]> for Quaternion {
    fn from(f: [f32; 16]) -> Self {
        Quaternion {
            x: f[0],
            y: f[1],
            z: f[2],
            w: f[3],
        }
    }
}
