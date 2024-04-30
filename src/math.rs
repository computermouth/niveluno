#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[deprecated]
pub fn clamp(v: f32, min: f32, max: f32) -> f32 {
    v.clamp(min, max)
}

pub fn scale(v: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    out_min + ((out_max) - out_min) * (((v) - in_min) / ((in_max) - in_min))
}

pub fn anglemod(r: f32) -> f32 {
    let l = r.sin();
    let r = r.cos();
    l.atan2(r)
}

pub fn vec3_rotate_y(p: Vec3, rad: f32) -> Vec3 {
    Vec3 {
        x: p.z * rad.sin() + p.x * rad.cos(),
        y: p.y,
        z: p.z * rad.cos() - p.x * rad.sin(),
    }
}

pub fn vec3_rotate_x(p: Vec3, rad: f32) -> Vec3 {
    Vec3 {
        x: p.x,
        y: p.y * rad.cos() - p.z * rad.sin(),
        z: p.y * rad.sin() + p.z * rad.cos(),
    }
}

pub fn vec3_rotate_yaw_pitch(p: Vec3, yaw: f32, pitch: f32) -> Vec3 {
    vec3_rotate_y(vec3_rotate_x(p, pitch), yaw)
}

pub fn vec3_2d_angle(a: Vec3, b: Vec3) -> f32 {
    (b.x - a.x).atan2(b.z - a.z)
}

pub fn vec3_length(a: Vec3) -> f32 {
    // Rust's f32 doesn't have a 3-argument hypot()
    (a.x * a.x + a.y * a.y + a.z * a.z).sqrt()
}

pub fn vec3_sub(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
    }
}

pub fn vec3_dist(a: Vec3, b: Vec3) -> f32 {
    vec3_length(vec3_sub(a, b))
}

pub fn vec3_dot(a: Vec3, b: Vec3) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

pub fn vec3_add(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    }
}

pub fn vec3_mul(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 {
        x: a.x * b.x,
        y: a.y * b.y,
        z: a.z * b.z,
    }
}

pub fn vec3_mulf(a: Vec3, b: f32) -> Vec3 {
    Vec3 {
        x: a.x * b,
        y: a.y * b,
        z: a.z * b,
    }
}

pub fn vec3_divf(a: Vec3, b: f32) -> Vec3 {
    Vec3 {
        x: a.x / b,
        y: a.y / b,
        z: a.z / b,
    }
}

pub fn vec3_cross(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    }
}

pub fn vec3_normalize(v: Vec3) -> Vec3 {
    // this seems like trash, but some
    // of the finer models have a vlen of ~0
    let mut vlen = vec3_length(v);
    if vlen <= 0.001 {
        vlen = 0.001;
    }
    return vec3_mulf(v, 1. / vlen);
}

pub fn vec3_face_normal(v0: Vec3, v1: Vec3, v2: Vec3) -> Vec3 {
    vec3_normalize(vec3_cross(vec3_sub(v0, v1), vec3_sub(v2, v1)))
}
