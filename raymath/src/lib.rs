//----------------------------------------------------------------------------------
// Consts
//----------------------------------------------------------------------------------

pub const DEG2RAD: f32 = std::f32::consts::PI / 180.0;
pub const RAD2DEG: f32 = 180.0 / std::f32::consts::PI;

//----------------------------------------------------------------------------------
// Types and Structures Definition
//----------------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}
#[derive(Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[derive(Copy, Clone)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
pub type Quaternion = Vector4;
#[derive(Copy, Clone)]
pub struct Matrix {
    pub m0: f32,
    pub m4: f32,
    pub m8: f32,
    pub m12: f32,
    pub m1: f32,
    pub m5: f32,
    pub m9: f32,
    pub m13: f32,
    pub m2: f32,
    pub m6: f32,
    pub m10: f32,
    pub m14: f32,
    pub m3: f32,
    pub m7: f32,
    pub m11: f32,
    pub m15: f32,
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
    let result: f32 = start + amount * (end - start);
    result
}

/// Normalize input value within input range
pub fn normalize(value: f32, start: f32, end: f32) -> f32 {
    let result: f32 = (value - start) / (end - start);
    result
}

/// Remap input value within input range to output range
pub fn remap(
    value: f32,
    input_start: f32,
    input_end: f32,
    output_start: f32,
    output_end: f32,
) -> f32 {
    let result: f32 = (value - input_start) / (input_end - input_start)
        * (output_end - output_start)
        + output_start;
    result
}

/// Wrap input value from min to max
pub fn wrap(value: f32, min: f32, max: f32) -> f32 {
    let result: f32 = value - (max - min) * ((value - min) / (max - min)).floor();
    result
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
    (v.x * v.x + v.y * v.y).sqrt()
}

/// Calculate vector square length
pub fn vector2_length_sqr(v: Vector2) -> f32 {
    let result: f32 = v.x * v.x + v.y * v.y;
    result
}

/// Calculate distance between two vectors
pub fn vector2_dot_product(v1: Vector2, v2: Vector2) -> f32 {
    let result: f32 = v1.x * v2.x + v1.y * v2.y;
    result
}

/// Calculate distance between two vectors
pub fn vector2_distance(v1: Vector2, v2: Vector2) -> f32 {
    ((v1.x - v2.x) * (v1.x - v2.x) + (v1.y - v2.y) * (v1.y - v2.y)).sqrt()
}

/// Calculate square distance between two vectors
pub fn vector2_distance_sqr(v1: Vector2, v2: Vector2) -> f32 {
    let result: f32 = (v1.x - v2.x) * (v1.x - v2.x) + (v1.y - v2.y) * (v1.y - v2.y);
    result
}

/// Calculate angle between two vectors
/// NOTE: Angle is calculated from origin point (0, 0)
pub fn vector2_angle(v1: Vector2, v2: Vector2) -> f32 {
    let dot: f32 = v1.x * v2.x + v1.y * v2.y;
    let det: f32 = v1.x * v2.y - v1.y * v2.x;

    det.atan2(dot)
}

/// Calculate angle defined by a two vectors line
/// NOTE: Parameters need to be normalized
/// Current implementation should be aligned with glm::angle
pub fn vector2_line_angle(start: Vector2, end: Vector2) -> f32 {
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
        let ilength: f32 = 1.0f32 / length;
        result.x = v.x * ilength;
        result.y = v.y * ilength;
    }
    result
}

/// Transforms a Vector2 by a given Matrix
pub fn vector2_transform(v: Vector2, mat: Matrix) -> Vector2 {
    let mut result = Vector2 { x: 0., y: 0. };
    let x: f32 = v.x;
    let y: f32 = v.y;
    let z: f32 = 0.;
    result.x = mat.m0 * x + mat.m4 * y + mat.m8 * z + mat.m12;
    result.y = mat.m1 * x + mat.m5 * y + mat.m9 * z + mat.m13;
    result
}

/// Calculate linear interpolation between two vectors
pub fn vector2_lerp(v1: Vector2, v2: Vector2, amount: f32) -> Vector2 {
    let mut result = Vector2 { x: 0., y: 0. };
    result.x = v1.x + amount * (v2.x - v1.x);
    result.y = v1.y + amount * (v2.y - v1.y);
    result
}

/// Calculate reflected vector to normal
pub fn vector2_reflect(v: Vector2, normal: Vector2) -> Vector2 {
    let mut result = Vector2 { x: 0., y: 0. };
    let dot_product: f32 = v.x * normal.x + v.y * normal.y;
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
    let cosres: f32 = angle.cos();
    let sinres: f32 = angle.sin();
    result.x = v.x * cosres - v.y * sinres;
    result.y = v.x * sinres + v.y * cosres;
    result
}

/// Move Vector towards target
pub fn vector2_move_towards(v: Vector2, target: Vector2, max_distance: f32) -> Vector2 {
    let mut result = Vector2 { x: 0., y: 0. };
    let dx: f32 = target.x - v.x;
    let dy: f32 = target.y - v.y;
    let value: f32 = dx * dx + dy * dy;
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
        x: 1.0f32 / v.x,
        y: 1.0f32 / v.y,
    }
}

/// Clamp the components of the vector between min and max values specified by the given vectors
pub fn vector2_clamp(v: Vector2, min: Vector2, max: Vector2) -> Vector2 {
    let mut result = Vector2 { x: 0., y: 0. };
    result.x = max.x.min(min.x.max(v.x));
    result.y = max.y.min(min.y.max(v.y));
    result
}

/// Clamp the magnitude of the vector between two min and max values
pub fn vector2_clamp_value(v: Vector2, min: f32, max: f32) -> Vector2 {
    let mut result: Vector2 = v;
    let mut length: f32 = v.x * v.x + v.y * v.y;
    if length > 0.0 {
        length = length.sqrt();
        let mut scale: f32 = 1.;
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
    (p.x - q.x).abs() <= f32::EPSILON * 1.0f32.max(p.x.abs().max(q.x.abs()))
        && (p.y - q.y).abs() <= f32::EPSILON * 1.0f32.max(p.y.abs().max(q.y.abs()))
}

/// Compute the direction of a refracted ray
/// 
/// v: normalized direction of the incoming ray
/// n: normalized normal vector of the interface of two optical media
/// r: ratio of the refractive index of the medium from where the ray comes
///    to the refractive index of the medium on the other side of the surface
pub fn vector2_refract(mut v: Vector2, n: Vector2, r: f32) -> Vector2 {
    let mut result = Vector2 { x: 0., y: 0. };
    let dot: f32 = v.x * n.x + v.y * n.y;
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
        x: 0.0f32,
        y: 0.0f32,
        z: 0.0f32,
    }
}

/// Vector with components value 1.0f
pub fn vector3_one() -> Vector3 {
    Vector3 {
        x: 1.0f32,
        y: 1.0f32,
        z: 1.0f32,
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
        x: 1.0f32,
        y: 0.0f32,
        z: 0.0f32,
    };
    if v.y.abs() < min {
        min = v.y.abs();
        let tmp = Vector3 {
            x: 0.0f32,
            y: 1.0f32,
            z: 0.0f32,
        };
        cardinal_axis = tmp;
    }
    if v.z.abs() < min {
        let tmp_0 = Vector3 {
            x: 0.0f32,
            y: 0.0f32,
            z: 1.0f32,
        };
        cardinal_axis = tmp_0;
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
    let dx: f32 = v2.x - v1.x;
    let dy: f32 = v2.y - v1.y;
    let dz: f32 = v2.z - v1.z;

    (dx * dx + dy * dy + dz * dz).sqrt()
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
    let cross = Vector3 {
        x: v1.y * v2.z - v1.z * v2.y,
        y: v1.z * v2.x - v1.x * v2.z,
        z: v1.x * v2.y - v1.y * v2.x,
    };
    let len: f32 = (cross.x * cross.x + cross.y * cross.y + cross.z * cross.z).sqrt();
    let dot: f32 = v1.x * v2.x + v1.y * v2.y + v1.z * v2.z;
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
    let length: f32 = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
    if length != 0.0f32 {
        let ilength: f32 = 1.0f32 / length;
        result.x *= ilength;
        result.y *= ilength;
        result.z *= ilength;
    }
    result
}

///Calculate the projection of the vector v1 on to v2
pub fn vector3_project(v1: Vector3, v2: Vector3) -> Vector3 {
    let mut result = Vector3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let v1dv2: f32 = v1.x * v2.x + v1.y * v2.y + v1.z * v2.z;
    let v2dv2: f32 = v2.x * v2.x + v2.y * v2.y + v2.z * v2.z;
    let mag: f32 = v1dv2 / v2dv2;
    result.x = v2.x * mag;
    result.y = v2.y * mag;
    result.z = v2.z * mag;
    result
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
pub fn vector3_ortho_normalize(v1: &mut Vector3, v2: &mut Vector3) {
    let mut v: Vector3 = *v1;
    let mut length = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
    if length == 0.0f32 {
        length = 1.0f32;
    }
    let mut ilength = 1.0f32 / length;
    v1.x *= ilength;
    v1.y *= ilength;
    v1.z *= ilength;
    
    let mut vn1 = Vector3 {
        x: v1.y * v2.z - v1.z * v2.y,
        y: v1.z * v2.x - v1.x * v2.z,
        z: v1.x * v2.y - v1.y * v2.x,
    };
    v = vn1;
    length = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
    if length == 0.0f32 {
        length = 1.0f32;
    }
    ilength = 1.0f32 / length;
    vn1.x *= ilength;
    vn1.y *= ilength;
    vn1.z *= ilength;
    let vn2 = Vector3 {
        x: vn1.y * v1.z - vn1.z * v1.y,
        y: vn1.z * v1.x - vn1.x * v1.z,
        z: vn1.x * v1.y - vn1.y * v1.x,
    };
    *v2 = vn2;
}

/// Transforms a Vector3 by a given Matrix
pub fn vector3_transform(v: Vector3, mat: Matrix) -> Vector3 {
    let mut result = Vector3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let x: f32 = v.x;
    let y: f32 = v.y;
    let z: f32 = v.z;
    result.x = mat.m0 * x + mat.m4 * y + mat.m8 * z + mat.m12;
    result.y = mat.m1 * x + mat.m5 * y + mat.m9 * z + mat.m13;
    result.z = mat.m2 * x + mat.m6 * y + mat.m10 * z + mat.m14;
    result
}

/// Transform a vector by quaternion rotation
pub fn vector3_rotate_by_quaternion(v: Vector3, q: Quaternion) -> Vector3 {
    Vector3 {
        x: v.x * (q.x * q.x + q.w * q.w - q.y * q.y - q.z * q.z)
            + v.y * (2. * q.x * q.y - 2. * q.w * q.z)
            + v.z * (2. * q.x * q.z + 2. * q.w * q.y),
        y: v.x * (2. * q.w * q.z + 2. * q.x * q.y)
            + v.y * (q.w * q.w - q.x * q.x + q.y * q.y - q.z * q.z)
            + v.z * (-2. * q.w * q.x + 2. * q.y * q.z),
        z: v.x * (-2. * q.w * q.y + 2. * q.x * q.z)
            + v.y * (2. * q.w * q.x + 2. * q.y * q.z)
            + v.z * (q.w * q.w - q.x * q.x - q.y * q.y + q.z * q.z),
    }
}

/// Rotates a vector around an axis
/// 
/// Using Euler-Rodrigues Formula
/// Ref.: https://en.wikipedia.org/w/index.php?title=Euler%E2%80%93Rodrigues_formula
pub fn vector3_rotate_by_axis_angle(v: Vector3, mut axis: Vector3, mut angle: f32) -> Vector3 {
    let mut result: Vector3 = v;
    let mut length: f32 = (axis.x * axis.x + axis.y * axis.y + axis.z * axis.z).sqrt();
    if length == 0.0f32 {
        length = 1.0f32;
    }
    let ilength: f32 = 1.0f32 / length;
    axis.x *= ilength;
    axis.y *= ilength;
    axis.z *= ilength;
    angle /= 2.0f32;
    let mut a: f32 = angle.sin();
    let b: f32 = axis.x * a;
    let c: f32 = axis.y * a;
    let d: f32 = axis.z * a;
    a = angle.cos();
    let w = Vector3 { x: b, y: c, z: d };
    let mut wv = Vector3 {
        x: w.y * v.z - w.z * v.y,
        y: w.z * v.x - w.x * v.z,
        z: w.x * v.y - w.y * v.x,
    };
    let mut wwv = Vector3 {
        x: w.y * wv.z - w.z * wv.y,
        y: w.z * wv.x - w.x * wv.z,
        z: w.x * wv.y - w.y * wv.x,
    };
    a *= 2.;
    wv.x *= a;
    wv.y *= a;
    wv.z *= a;
    wwv.x *= 2.;
    wwv.y *= 2.;
    wwv.z *= 2.;
    result.x += wv.x;
    result.y += wv.y;
    result.z += wv.z;
    result.x += wwv.x;
    result.y += wwv.y;
    result.z += wwv.z;
    result
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
    let mut result = Vector3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    result.x = v1.x + amount * (v2.x - v1.x);
    result.y = v1.y + amount * (v2.y - v1.y);
    result.z = v1.z + amount * (v2.z - v1.z);
    result
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
    let mut result = Vector3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let dot_product: f32 = v.x * normal.x + v.y * normal.y + v.z * normal.z;
    result.x = v.x - 2.0f32 * normal.x * dot_product;
    result.y = v.y - 2.0f32 * normal.y * dot_product;
    result.z = v.z - 2.0f32 * normal.z * dot_product;
    result
}

/// Get min value for each pair of components
pub fn vector3_min(v1: Vector3, v2: Vector3) -> Vector3 {
    let mut result = Vector3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    result.x = v1.x.min(v2.x);
    result.y = v1.y.min(v2.y);
    result.z = v1.z.min(v2.z);
    result
}

/// Get max value for each pair of components
pub fn vector3_max(v1: Vector3, v2: Vector3) -> Vector3 {
    let mut result = Vector3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    result.x = v1.x.max(v2.x);
    result.y = v1.y.max(v2.y);
    result.z = v1.z.max(v2.z);
    result
}

/// Compute barycenter coordinates (u, v, w) for point p with respect to triangle (a, b, c)
/// 
/// NOTE: Assumes P is on the plane of the triangle
pub fn vector3_barycenter(p: Vector3, a: Vector3, b: Vector3, c: Vector3) -> Vector3 {
    let mut result = Vector3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let v0 = Vector3 {
        x: b.x - a.x,
        y: b.y - a.y,
        z: b.z - a.z,
    };
    let v1 = Vector3 {
        x: c.x - a.x,
        y: c.y - a.y,
        z: c.z - a.z,
    };
    let v2 = Vector3 {
        x: p.x - a.x,
        y: p.y - a.y,
        z: p.z - a.z,
    };
    let d00: f32 = v0.x * v0.x + v0.y * v0.y + v0.z * v0.z;
    let d01: f32 = v0.x * v1.x + v0.y * v1.y + v0.z * v1.z;
    let d11: f32 = v1.x * v1.x + v1.y * v1.y + v1.z * v1.z;
    let d20: f32 = v2.x * v0.x + v2.y * v0.y + v2.z * v0.z;
    let d21: f32 = v2.x * v1.x + v2.y * v1.y + v2.z * v1.z;
    let denom: f32 = d00 * d11 - d01 * d01;
    result.y = (d11 * d20 - d01 * d21) / denom;
    result.z = (d00 * d21 - d01 * d20) / denom;
    result.x = 1.0f32 - (result.z + result.y);
    result
}

/// Projects a Vector3 from screen space into object space
/// 
/// NOTE: We are avoiding calling other raymath functions despite available
pub fn vector3_unproject(source: Vector3, projection: Matrix, view: Matrix) -> Vector3 {
    let mut result = Vector3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let mat_view_proj = Matrix {
        m0: view.m0 * projection.m0
            + view.m1 * projection.m4
            + view.m2 * projection.m8
            + view.m3 * projection.m12,
        m4: view.m0 * projection.m1
            + view.m1 * projection.m5
            + view.m2 * projection.m9
            + view.m3 * projection.m13,
        m8: view.m0 * projection.m2
            + view.m1 * projection.m6
            + view.m2 * projection.m10
            + view.m3 * projection.m14,
        m12: view.m0 * projection.m3
            + view.m1 * projection.m7
            + view.m2 * projection.m11
            + view.m3 * projection.m15,
        m1: view.m4 * projection.m0
            + view.m5 * projection.m4
            + view.m6 * projection.m8
            + view.m7 * projection.m12,
        m5: view.m4 * projection.m1
            + view.m5 * projection.m5
            + view.m6 * projection.m9
            + view.m7 * projection.m13,
        m9: view.m4 * projection.m2
            + view.m5 * projection.m6
            + view.m6 * projection.m10
            + view.m7 * projection.m14,
        m13: view.m4 * projection.m3
            + view.m5 * projection.m7
            + view.m6 * projection.m11
            + view.m7 * projection.m15,
        m2: view.m8 * projection.m0
            + view.m9 * projection.m4
            + view.m10 * projection.m8
            + view.m11 * projection.m12,
        m6: view.m8 * projection.m1
            + view.m9 * projection.m5
            + view.m10 * projection.m9
            + view.m11 * projection.m13,
        m10: view.m8 * projection.m2
            + view.m9 * projection.m6
            + view.m10 * projection.m10
            + view.m11 * projection.m14,
        m14: view.m8 * projection.m3
            + view.m9 * projection.m7
            + view.m10 * projection.m11
            + view.m11 * projection.m15,
        m3: view.m12 * projection.m0
            + view.m13 * projection.m4
            + view.m14 * projection.m8
            + view.m15 * projection.m12,
        m7: view.m12 * projection.m1
            + view.m13 * projection.m5
            + view.m14 * projection.m9
            + view.m15 * projection.m13,
        m11: view.m12 * projection.m2
            + view.m13 * projection.m6
            + view.m14 * projection.m10
            + view.m15 * projection.m14,
        m15: view.m12 * projection.m3
            + view.m13 * projection.m7
            + view.m14 * projection.m11
            + view.m15 * projection.m15,
    };
    let a00: f32 = mat_view_proj.m0;
    let a01: f32 = mat_view_proj.m1;
    let a02: f32 = mat_view_proj.m2;
    let a03: f32 = mat_view_proj.m3;
    let a10: f32 = mat_view_proj.m4;
    let a11: f32 = mat_view_proj.m5;
    let a12: f32 = mat_view_proj.m6;
    let a13: f32 = mat_view_proj.m7;
    let a20: f32 = mat_view_proj.m8;
    let a21: f32 = mat_view_proj.m9;
    let a22: f32 = mat_view_proj.m10;
    let a23: f32 = mat_view_proj.m11;
    let a30: f32 = mat_view_proj.m12;
    let a31: f32 = mat_view_proj.m13;
    let a32: f32 = mat_view_proj.m14;
    let a33: f32 = mat_view_proj.m15;
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
    let mat_view_proj_inv = Matrix {
        m0: (a11 * b11 - a12 * b10 + a13 * b09) * inv_det,
        m4: (-a01 * b11 + a02 * b10 - a03 * b09) * inv_det,
        m8: (a31 * b05 - a32 * b04 + a33 * b03) * inv_det,
        m12: (-a21 * b05 + a22 * b04 - a23 * b03) * inv_det,
        m1: (-a10 * b11 + a12 * b08 - a13 * b07) * inv_det,
        m5: (a00 * b11 - a02 * b08 + a03 * b07) * inv_det,
        m9: (-a30 * b05 + a32 * b02 - a33 * b01) * inv_det,
        m13: (a20 * b05 - a22 * b02 + a23 * b01) * inv_det,
        m2: (a10 * b10 - a11 * b08 + a13 * b06) * inv_det,
        m6: (-a00 * b10 + a01 * b08 - a03 * b06) * inv_det,
        m10: (a30 * b04 - a31 * b02 + a33 * b00) * inv_det,
        m14: (-a20 * b04 + a21 * b02 - a23 * b00) * inv_det,
        m3: (-a10 * b09 + a11 * b07 - a12 * b06) * inv_det,
        m7: (a00 * b09 - a01 * b07 + a02 * b06) * inv_det,
        m11: (-a30 * b03 + a31 * b01 - a32 * b00) * inv_det,
        m15: (a20 * b03 - a21 * b01 + a22 * b00) * inv_det,
    };
    let quat = Quaternion {
        x: source.x,
        y: source.y,
        z: source.z,
        w: 1.0f32,
    };
    let qtransformed = Quaternion {
        x: mat_view_proj_inv.m0 * quat.x
            + mat_view_proj_inv.m4 * quat.y
            + mat_view_proj_inv.m8 * quat.z
            + mat_view_proj_inv.m12 * quat.w,
        y: mat_view_proj_inv.m1 * quat.x
            + mat_view_proj_inv.m5 * quat.y
            + mat_view_proj_inv.m9 * quat.z
            + mat_view_proj_inv.m13 * quat.w,
        z: mat_view_proj_inv.m2 * quat.x
            + mat_view_proj_inv.m6 * quat.y
            + mat_view_proj_inv.m10 * quat.z
            + mat_view_proj_inv.m14 * quat.w,
        w: mat_view_proj_inv.m3 * quat.x
            + mat_view_proj_inv.m7 * quat.y
            + mat_view_proj_inv.m11 * quat.z
            + mat_view_proj_inv.m15 * quat.w,
    };
    result.x = qtransformed.x / qtransformed.w;
    result.y = qtransformed.y / qtransformed.w;
    result.z = qtransformed.z / qtransformed.w;
    result
}

/// Get Vector3 as float array
pub fn vector3_to_float(v: Vector3) -> [f32; 3] {
    vector3_to_float_v(v).v
}

/// Get Vector3 as float array
pub fn vector3_to_float_v(v: Vector3) -> Float3 {
    let mut buffer = Float3 { v: [0., 0., 0.] };
    buffer.v[0] = v.x;
    buffer.v[1] = v.y;
    buffer.v[2] = v.z;
    buffer
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
    let mut result = Vector3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    result.x = max.x.min(min.x.max(v.x));
    result.y = max.y.min(min.y.max(v.y));
    result.z = max.z.min(min.z.max(v.z));
    result
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
    (p.x - q.x).abs() <= f32::EPSILON * 1.0f32.max((p.x).abs().max((q.x).abs()))
        && (p.y - q.y).abs() <= f32::EPSILON * 1.0f32.max((p.y).abs().max((q.y).abs()))
        && (p.z - q.z).abs() <= f32::EPSILON * 1.0f32.max((p.z).abs().max((q.z).abs()))
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
    let dot: f32 = v.x * n.x + v.y * n.y + v.z * n.z;
    let mut d: f32 = 1.0f32 - r * r * (1.0f32 - dot * dot);
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
        x: 0.0f32,
        y: 0.0f32,
        z: 0.0f32,
        w: 0.0f32,
    }
}

pub fn vector4_one() -> Vector4 {
    Vector4 {
        x: 1.0f32,
        y: 1.0f32,
        z: 1.0f32,
        w: 1.0f32,
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
    (v.x * v.x + v.y * v.y + v.z * v.z + v.w * v.w).sqrt()
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
    let mut result = Vector4 {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    let length: f32 = (v.x * v.x + v.y * v.y + v.z * v.z + v.w * v.w).sqrt();
    if length > 0. {
        let ilength: f32 = 1.0f32 / length;
        result.x = v.x * ilength;
        result.y = v.y * ilength;
        result.z = v.z * ilength;
        result.w = v.w * ilength;
    }
    result
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
    let mut result = Vector4 {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    result.x = v1.x.max(v2.x);
    result.y = v1.y.max(v2.y);
    result.z = v1.z.max(v2.z);
    result.w = v1.w.max(v2.w);
    result
}

/// Calculate linear interpolation between two vectors
pub fn vector4_lerp(v1: Vector4, v2: Vector4, amount: f32) -> Vector4 {
    let mut result = Vector4 {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    result.x = v1.x + amount * (v2.x - v1.x);
    result.y = v1.y + amount * (v2.y - v1.y);
    result.z = v1.z + amount * (v2.z - v1.z);
    result.w = v1.w + amount * (v2.w - v1.w);
    result
}

/// Move Vector towards target
pub fn vector4_move_towards(v: Vector4, target: Vector4, max_distance: f32) -> Vector4 {
    let mut result = Vector4 {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    let dx: f32 = target.x - v.x;
    let dy: f32 = target.y - v.y;
    let dz: f32 = target.z - v.z;
    let dw: f32 = target.w - v.w;
    let value: f32 = dx * dx + dy * dy + dz * dz + dw * dw;
    if value == 0. || max_distance >= 0. && value <= max_distance * max_distance {
        return target;
    }
    let dist: f32 = value.sqrt();
    result.x = v.x + dx / dist * max_distance;
    result.y = v.y + dy / dist * max_distance;
    result.z = v.z + dz / dist * max_distance;
    result.w = v.w + dw / dist * max_distance;
    result
}

/// Invert the given vector
pub fn vector4_invert(v: Vector4) -> Vector4 {
    Vector4 {
        x: 1.0f32 / v.x,
        y: 1.0f32 / v.y,
        z: 1.0f32 / v.z,
        w: 1.0f32 / v.w,
    }
}

/// Check whether two given vectors are almost equal
pub fn vector4_equals(p: Vector4, q: Vector4) -> bool {
    (p.x - q.x).abs() <= f32::EPSILON * 1.0f32.max((p.x).abs().max((q.x).abs()))
        && (p.y - q.y).abs() <= f32::EPSILON * 1.0f32.max((p.y).abs().max((q.y).abs()))
        && (p.z - q.z).abs() <= f32::EPSILON * 1.0f32.max((p.z).abs().max((q.z).abs()))
        && (p.w - q.w).abs() <= f32::EPSILON * 1.0f32.max((p.w).abs().max((q.w).abs()))
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
    result.m0 = (a11 * b11 - a12 * b10 + a13 * b09) * inv_det;
    result.m1 = (-a01 * b11 + a02 * b10 - a03 * b09) * inv_det;
    result.m2 = (a31 * b05 - a32 * b04 + a33 * b03) * inv_det;
    result.m3 = (-a21 * b05 + a22 * b04 - a23 * b03) * inv_det;
    result.m4 = (-a10 * b11 + a12 * b08 - a13 * b07) * inv_det;
    result.m5 = (a00 * b11 - a02 * b08 + a03 * b07) * inv_det;
    result.m6 = (-a30 * b05 + a32 * b02 - a33 * b01) * inv_det;
    result.m7 = (a20 * b05 - a22 * b02 + a23 * b01) * inv_det;
    result.m8 = (a10 * b10 - a11 * b08 + a13 * b06) * inv_det;
    result.m9 = (-a00 * b10 + a01 * b08 - a03 * b06) * inv_det;
    result.m10 = (a30 * b04 - a31 * b02 + a33 * b00) * inv_det;
    result.m11 = (-a20 * b04 + a21 * b02 - a23 * b00) * inv_det;
    result.m12 = (-a10 * b09 + a11 * b07 - a12 * b06) * inv_det;
    result.m13 = (a00 * b09 - a01 * b07 + a02 * b06) * inv_det;
    result.m14 = (-a30 * b03 + a31 * b01 - a32 * b00) * inv_det;
    result.m15 = (a20 * b03 - a21 * b01 + a22 * b00) * inv_det;
    result
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
    let mut result = Matrix {
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
    };
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
    let mut result = Matrix {
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
    };
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
    let mut result = Matrix {
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
    };
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
    let mut result = Matrix {
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
    };
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
    let mut result = Quaternion {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    let mut length: f32 = (q.x * q.x + q.y * q.y + q.z * q.z + q.w * q.w).sqrt();
    if length == 0.0f32 {
        length = 1.0f32;
    }
    let ilength: f32 = 1.0f32 / length;
    result.x = q.x * ilength;
    result.y = q.y * ilength;
    result.z = q.z * ilength;
    result.w = q.w * ilength;
    result
}

/// Invert provided quaternion
pub fn quaternion_invert(q: Quaternion) -> Quaternion {
    let mut result: Quaternion = q;
    let length_sq: f32 = q.x * q.x + q.y * q.y + q.z * q.z + q.w * q.w;
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
    let mut result = Quaternion {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    result.x = q1.x + amount * (q2.x - q1.x);
    result.y = q1.y + amount * (q2.y - q1.y);
    result.z = q1.z + amount * (q2.z - q1.z);
    result.w = q1.w + amount * (q2.w - q1.w);
    let q: Quaternion = result;
    let mut length: f32 = (q.x * q.x + q.y * q.y + q.z * q.z + q.w * q.w).sqrt();
    if length == 0.0f32 {
        length = 1.0f32;
    }
    let ilength: f32 = 1.0f32 / length;
    result.x = q.x * ilength;
    result.y = q.y * ilength;
    result.z = q.z * ilength;
    result.w = q.w * ilength;
    result
}

/// Calculates spherical linear interpolation between two quaternions
pub fn quaternion_slerp(q1: Quaternion, mut q2: Quaternion, amount: f32) -> Quaternion {
    let mut result = Quaternion {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    let mut cos_half_theta: f32 = q1.x * q2.x + q1.y * q2.y + q1.z * q2.z + q1.w * q2.w;
    if cos_half_theta < 0. {
        q2.x = -q2.x;
        q2.y = -q2.y;
        q2.z = -q2.z;
        q2.w = -q2.w;
        cos_half_theta = -cos_half_theta;
    }
    if cos_half_theta.abs() >= 1.0f32 {
        result = q1;
    } else if cos_half_theta > 0.95f32 {
        result = quaternion_nlerp(q1, q2, amount);
    } else {
        let half_theta: f32 = cos_half_theta.acos();
        let sin_half_theta: f32 = (1.0f32 - cos_half_theta * cos_half_theta).sqrt();
        if sin_half_theta.abs() < f32::EPSILON {
            result.x = q1.x * 0.5f32 + q2.x * 0.5f32;
            result.y = q1.y * 0.5f32 + q2.y * 0.5f32;
            result.z = q1.z * 0.5f32 + q2.z * 0.5f32;
            result.w = q1.w * 0.5f32 + q2.w * 0.5f32;
        } else {
            let ratio_a: f32 = ((1. - amount) * half_theta).sin() / sin_half_theta;
            let ratio_b: f32 = (amount * half_theta).sin() / sin_half_theta;
            result.x = q1.x * ratio_a + q2.x * ratio_b;
            result.y = q1.y * ratio_a + q2.y * ratio_b;
            result.z = q1.z * ratio_a + q2.z * ratio_b;
            result.w = q1.w * ratio_a + q2.w * ratio_b;
        }
    }
    result
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
    let mut result = Quaternion {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    let cos2_theta: f32 = from.x * to.x + from.y * to.y + from.z * to.z;
    let cross = Vector3 {
        x: from.y * to.z - from.z * to.y,
        y: from.z * to.x - from.x * to.z,
        z: from.x * to.y - from.y * to.x,
    };
    result.x = cross.x;
    result.y = cross.y;
    result.z = cross.z;
    result.w = 1.0f32 + cos2_theta;
    let q: Quaternion = result;
    let mut length: f32 = (q.x * q.x + q.y * q.y + q.z * q.z + q.w * q.w).sqrt();
    if length == 0.0f32 {
        length = 1.0f32;
    }
    let ilength: f32 = 1.0f32 / length;
    result.x = q.x * ilength;
    result.y = q.y * ilength;
    result.z = q.z * ilength;
    result.w = q.w * ilength;
    result
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
    let mut result = Matrix {
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
    };
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
pub fn quaternion_from_axis_angle(mut axis: Vector3, mut angle: f32) -> Quaternion {
    let mut result = Quaternion {
        x: 0.0f32,
        y: 0.0f32,
        z: 0.0f32,
        w: 1.0f32,
    };
    let axis_length: f32 = (axis.x * axis.x + axis.y * axis.y + axis.z * axis.z).sqrt();
    if axis_length != 0.0f32 {
        angle *= 0.5f32;
        let mut length = axis_length;
        if length == 0.0f32 {
            length = 1.0f32;
        }
        let mut ilength = 1.0f32 / length;
        axis.x *= ilength;
        axis.y *= ilength;
        axis.z *= ilength;
        let sinres: f32 = angle.sin();
        let cosres: f32 = angle.cos();
        result.x = axis.x * sinres;
        result.y = axis.y * sinres;
        result.z = axis.z * sinres;
        result.w = cosres;
        let q: Quaternion = result;
        length = (q.x * q.x + q.y * q.y + q.z * q.z + q.w * q.w).sqrt();
        if length == 0.0f32 {
            length = 1.0f32;
        }
        ilength = 1.0f32 / length;
        result.x = q.x * ilength;
        result.y = q.y * ilength;
        result.z = q.z * ilength;
        result.w = q.w * ilength;
    }
    result
}

/// Get the rotation angle and axis for a given quaternion
pub fn quaternion_to_axis_angle(mut q: Quaternion, out_axis: &mut Vector3, out_angle: &mut f32) {
    if q.w.abs() > 1.0f32 {
        let mut length: f32 = (q.x * q.x + q.y * q.y + q.z * q.z + q.w * q.w).sqrt();
        if length == 0.0f32 {
            length = 1.0f32;
        }
        let ilength: f32 = 1.0f32 / length;
        q.x *= ilength;
        q.y *= ilength;
        q.z *= ilength;
        q.w *= ilength;
    }
    let mut res_axis = Vector3 {
        x: 0.0f32,
        y: 0.0f32,
        z: 0.0f32,
    };
    let res_angle: f32 = 2.0f32 * q.w.acos();
    let den: f32 = (1.0f32 - q.w * q.w).sqrt();
    if den > f32::EPSILON {
        res_axis.x = q.x / den;
        res_axis.y = q.y / den;
        res_axis.z = q.z / den;
    } else {
        res_axis.x = 1.0f32;
    }
    *out_axis = res_axis;
    *out_angle = res_angle;
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
    let x0: f32 = 2.0f32 * (q.w * q.x + q.y * q.z);
    let x1: f32 = 1.0f32 - 2.0f32 * (q.x * q.x + q.y * q.y);
    result.x = x0.atan2(x1);
    let mut y0: f32 = 2.0f32 * (q.w * q.y - q.z * q.x);
    y0 = if y0 > 1.0f32 { 1.0f32 } else { y0 };
    y0 = if y0 < -1.0f32 { -1.0f32 } else { y0 };
    result.y = y0.asin();
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
    (p.x - q.x).abs() <= f32::EPSILON * 1.0f32.max((p.x).abs().max((q.x).abs()))
        && (p.y - q.y).abs() <= f32::EPSILON * 1.0f32.max((p.y).abs().max((q.y).abs()))
        && (p.z - q.z).abs() <= f32::EPSILON * 1.0f32.max((p.z).abs().max((q.z).abs()))
        && (p.w - q.w).abs() <= f32::EPSILON * 1.0f32.max((p.w).abs().max((q.w).abs()))
        || (p.x + q.x).abs() <= f32::EPSILON * 1.0f32.max((p.x).abs().max((q.x).abs()))
            && (p.y + q.y).abs() <= f32::EPSILON * 1.0f32.max((p.y).abs().max((q.y).abs()))
            && (p.z + q.z).abs() <= f32::EPSILON * 1.0f32.max((p.z).abs().max((q.z).abs()))
            && (p.w + q.w).abs() <= f32::EPSILON * 1.0f32.max((p.w).abs().max((q.w).abs()))
}

/// Decompose a transformation matrix into its rotational, translational and scaling components
pub fn matrix_decompose(
    mat: Matrix,
    translation: &mut Vector3,
    rotation: &mut Quaternion,
    scale: &mut Vector3,
) {
    translation.x = mat.m12;
    translation.y = mat.m13;
    translation.z = mat.m14;

    let a: f32 = mat.m0;
    let b: f32 = mat.m4;
    let c: f32 = mat.m8;
    let d: f32 = mat.m1;
    let e: f32 = mat.m5;
    let f: f32 = mat.m9;
    let g: f32 = mat.m2;
    let h: f32 = mat.m6;
    let i: f32 = mat.m10;

    let det_a: f32 = e * i - f * h;
    let det_b: f32 = f * g - d * i;
    let det_c: f32 = d * h - e * g;
    let det: f32 = a * det_a + b * det_b + c * det_c;

    let abc = Vector3 { x: a, y: b, z: c };
    let def = Vector3 { x: d, y: e, z: f };
    let ghi = Vector3 { x: g, y: h, z: i };
    let scalex: f32 = vector3_length(abc);
    let scaley: f32 = vector3_length(def);
    let scalez: f32 = vector3_length(ghi);
    let mut s = Vector3 {
        x: scalex,
        y: scaley,
        z: scalez,
    };
    if det < 0. {
        s = vector3_negate(s);
    }
    *scale = s;
    let mut clone: Matrix = mat;
    if float_equals(det, 0.) {
        clone.m0 /= s.x;
        clone.m5 /= s.y;
        clone.m10 /= s.z;
        *rotation = quaternion_from_matrix(clone);
    } else {
        *rotation = quaternion_identity();
    };
}