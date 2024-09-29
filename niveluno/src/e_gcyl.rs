use crate::e_entity::EntityInstance;
use crate::map::Entity;
use crate::time;

use crate::g_game;
use crate::render;
use raymath::Vector3;

pub enum GcylFrames {
    Default,
    Cylinder001,
    Cylinder002,
}

impl From<&str> for GcylFrames {
    fn from(value: &str) -> Self {
        match value {
            "default" => Self::Default,
            "Cylinder.001" => Self::Cylinder001,
            "Cylinder.002" => Self::Cylinder002,
            _ => {
                eprintln!("unmatched frame: {value}");
                Self::Default
            }
        }
    }
}

pub struct Gcyl {
    base: Entity,
    yaw: f32,
    pitch: f32,
    scale_mat: raymath::Matrix,
    quat: raymath::Quaternion,
}

impl EntityInstance for Gcyl {
    fn update(&mut self) {
        let dt = time::get_delta_time().unwrap() as f32;

        self.yaw += 1. * dt;
        self.pitch -= 3. * dt;

        let quat_y = raymath::quaternion_from_axis_angle(Vector3::new(1., 0., 0.), self.yaw);
        let quat_p = raymath::quaternion_from_axis_angle(Vector3::new(0., 1., 0.), self.pitch);

        self.quat = self.base.rotation.into();
        self.quat = raymath::quaternion_multiply(self.quat, quat_p);
        self.quat = raymath::quaternion_multiply(self.quat, quat_y);
    }

    fn draw_model(&mut self) {
        let ref_ent = g_game::get_ref_entity(self.base.index).unwrap();

        let mat_r = raymath::quaternion_to_matrix(self.quat);
        let mat_t = raymath::matrix_translate(
            self.base.location[0],
            self.base.location[1],
            self.base.location[2],
        );

        let mut mat = raymath::matrix_identity();
        mat = raymath::matrix_multiply(mat, self.scale_mat);
        mat = raymath::matrix_multiply(mat, mat_r);
        mat = raymath::matrix_multiply(mat, mat_t);

        let t = ((time::get_run_time().unwrap().sin() + 1.) / 2.) as f32;

        let dc = render::DrawCall {
            matrix: mat,
            texture: ref_ent.texture_handle as u32,
            f1: ref_ent.frame_handles[0] as i32,
            f2: ref_ent.frame_handles[1] as i32,
            mix: t,
            num_verts: ref_ent.num_verts,
            glow: None,
        };
        render::draw(dc).unwrap();
    }
}

impl Gcyl {
    pub fn new(entt: &Entity) -> Self {
        let ref_ent = g_game::get_ref_entity(entt.index).unwrap();

        eprintln!("re.names: {:?}", ref_ent.frame_names);

        Self {
            base: entt.clone(),
            yaw: 0.,
            pitch: 0.,
            scale_mat: raymath::matrix_scale(entt.scale[0], entt.scale[1], entt.scale[2]),
            quat: raymath::quaternion_identity(),
        }
    }
}
