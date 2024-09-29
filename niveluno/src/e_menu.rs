use core::f32;

use crate::e_entity::EntityInstance;
use crate::map::Entity;
use crate::time;

use crate::g_game;
use crate::render;

use raymath::{self, Vector3};

#[derive(Debug)]
pub struct Menu {
    base: Entity,
    yaw: f32,
    pitch: f32,
    scale_mat: raymath::Matrix,
    quat: raymath::Quaternion,
}

impl EntityInstance for Menu {
    fn update(&mut self) {
        let time = time::get_run_time().unwrap() as f32;

        self.yaw = time.sin() as f32 / 2.;
        self.pitch = (time * 2.).sin() as f32 / 4.;

        let quat_y = raymath::quaternion_from_axis_angle(Vector3::new(1., 0., 0.), self.yaw);
        let quat_p = raymath::quaternion_from_axis_angle(Vector3::new(0., 1., 0.), self.pitch);

        self.quat = self.base.rotation.into();
        self.quat = raymath::quaternion_multiply(self.quat, quat_y);
        self.quat = raymath::quaternion_multiply(self.quat, quat_p);
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

        let dc = render::DrawCall {
            matrix: mat,
            texture: ref_ent.texture_handle as u32,
            f1: ref_ent.frame_handles[0] as i32,
            f2: ref_ent.frame_handles[0] as i32,
            mix: 0.,
            num_verts: ref_ent.num_verts,
            glow: None,
        };
        render::draw(dc).unwrap();
    }
}

impl Menu {
    pub fn new(entt: &Entity) -> Self {
        let euler = raymath::quaternion_to_euler(entt.rotation.into());

        eprintln!("base.rot: {:?} -- euler: {:?}", entt.rotation, euler);
        Self {
            base: entt.clone(),
            yaw: f32::consts::PI,
            pitch: 0.,
            quat: raymath::quaternion_identity(),
            scale_mat: raymath::matrix_scale(entt.scale[0], entt.scale[1], entt.scale[2]),
        }
    }
}
