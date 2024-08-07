pub use raymath::*;

pub fn scale(v: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    out_min + ((out_max) - out_min) * (((v) - in_min) / ((in_max) - in_min))
}

pub fn vec3_face_normal(v0: Vector3, v1: Vector3, v2: Vector3) -> Vector3 {
    let lh = v0 - v1;
    let rh = v2 - v1;

    let cross = lh.cross(rh);

    cross.normalized()
}
