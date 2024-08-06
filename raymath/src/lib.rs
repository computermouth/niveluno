//----------------------------------------------------------------------------------
// Defines and Macros
//----------------------------------------------------------------------------------

const DEG2RAD: f32 = std::f32::consts::PI / 180.;
const RAD2DEG: f32 = 180. / std::f32::consts::PI;

/// Get float vector for Matrix
// pub matrix_to_float(mat: Matrix) -> f32 {
//     matrix_to_float_v(mat).v
// }

// pub vec3_to_float(vec: Vec3) -> f32 {
//     vec3_to_float_v(vec).v
// }

//----------------------------------------------------------------------------------
// Types and Structures Definition
//----------------------------------------------------------------------------------

/// Vec2 type
pub struct Vec2 {
    x: f32,
    y: f32,
}

/// Vec3 type
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

/// Vec4 type
pub struct Vec4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

/// Quaternion type
pub type Quaternion = Vec4;

/// Matrix type (OpenGL style 4x4 - right handed, column major)
pub struct Matrix {
    m0: f32,
    m4: f32,
    m8: f32,
    m12: f32, // Matrix first row (4 components)
    m1: f32,
    m5: f32,
    m9: f32,
    m13: f32, // Matrix second row (4 components)
    m2: f32,
    m6: f32,
    m10: f32,
    m14: f32, // Matrix third row (4 components)
    m3: f32,
    m7: f32,
    m11: f32,
    m15: f32, // Matrix fourth row (4 components)
}

// NOTE: Helper types to be used instead of array return types for *ToFloat functions
pub struct Float3 {
    v: [f32; 3],
}

pub struct Float16 {
    v: [f32; 16],
}

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
    value - (max - min) * ((value - min) / (max - min)).floor()
}

/// Check whether two given floats are almost equal
pub fn float_equals(x: f32, y: f32) -> bool {
    (x - y).abs() <= (f32::EPSILON * 1.0_f32.max(x.abs().max(y.abs())))
}

//----------------------------------------------------------------------------------
// Module Functions Definition - Vector2 math
//----------------------------------------------------------------------------------

/// Vec2 with components value 0.0f
pub fn vec2_zero() -> Vec2 {
    Vec2 { x: 0., y: 0. }
}

/// Vec2 with components value 1.0f
pub fn vec2_one() -> Vec2 {
    Vec2 { x: 1., y: 1. }
}

/// Add two vectors (v1 + v2)
pub fn vec2_add(v1: Vec2, v2: Vec2) -> Vec2 {
    Vec2 {
        x: v1.x + v2.x,
        y: v1.y + v2.y,
    }
}

/// Add vector and float value
pub fn vec2_add_value(v: Vec2, add: f32) -> Vec2 {
    Vec2 {
        x: v.x + add,
        y: v.y + add,
    }
}

/// Subtract two vectors (v1 - v2)
pub fn vec2_subtract(v1: Vec2, v2: Vec2) -> Vec2 {
    Vec2 {
        x: v1.x - v2.x,
        y: v1.y - v2.y,
    }
}

/// Subtract vector by float value
pub fn vec2_subtract_value(v: Vec2, sub: f32) -> Vec2 {
    Vec2 {
        x: v.x - sub,
        y: v.y - sub,
    }
}

/// Calculate vector length
pub fn vec2_length(v: Vec2) -> f32 {
    ((v.x * v.x) + (v.y * v.y)).sqrt()
}

/// Calculate vector square length
pub fn vec2_length_sqr(v: Vec2) -> f32 {
    (v.x * v.x) + (v.y * v.y)
}

/// Calculate two vectors dot product
pub fn vec2_dot_product(v1: Vec2, v2: Vec2) -> f32 {
    v1.x * v2.x + v1.y * v2.y
}

/// Calculate distance between two vectors
pub fn vec2_distance(v1: Vec2, v2: Vec2) -> f32 {
    ((v1.x - v2.x) * (v1.x - v2.x) + (v1.y - v2.y) * (v1.y - v2.y)).sqrt()
}

/// Calculate square distance between two vectors
pub fn vec2_distance_sqr(v1: Vec2, v2: Vec3) -> f32 {
    (v1.x - v2.x) * (v1.x - v2.x) + (v1.y - v2.y) * (v1.y - v2.y)
}

/// Calculate angle between two vectors
/// NOTE: Angle is calculated from origin point (0, 0)
pub fn vec2_angle(v1: Vec2, v2: Vec2) -> f32 {
    let dot = v1.x * v2.x + v1.y * v2.y;
    let det = v1.x * v2.y - v1.y * v2.x;

    det.atan2(dot)
}

/// Calculate angle defined by a two vectors line
/// NOTE: Parameters need to be normalized
/// Current implementation should be aligned with glm::angle
pub fn vec2_line_angle(start: Vec2, end: Vec2) -> f32 {
    -(end.y - start.y).atan2(end.x - start.x)
}

/// Scale vector (multiply by value)
pub fn vec2_scale(v: Vec2, scale: f32) -> Vec2 {
    Vec2 {
        x: v.x * scale,
        y: v.y * scale,
    }
}
