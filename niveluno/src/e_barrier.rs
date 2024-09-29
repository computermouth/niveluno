use raymath::{vector3_add, vector3_multiply, vector3_scale};

use crate::math::Vector3;

use crate::e_entity::EntityInstance;
use crate::map::Entity;

use crate::{g_game, render, time};

pub struct Barrier {
    base: Entity,
    id: Option<u32>,
}

impl EntityInstance for Barrier {
    fn update(&mut self) {}
    fn draw_model(&mut self) {
        let ref_ent = g_game::get_ref_entity(self.base.index).unwrap();

        let mat_s =
            raymath::matrix_scale(self.base.scale[0], self.base.scale[1], self.base.scale[2]);
        let mat_r = raymath::quaternion_to_matrix(self.base.rotation.into());
        let mat_t = raymath::matrix_translate(
            self.base.location[0],
            self.base.location[1],
            self.base.location[2],
        );

        let mut mat = raymath::matrix_identity();
        mat = raymath::matrix_multiply(mat, mat_s);
        mat = raymath::matrix_multiply(mat, mat_r);
        mat = raymath::matrix_multiply(mat, mat_t);

        let color = match self.id.unwrap() {
            0 => Vector3::new(0., 0., 255.),
            1 => Vector3::new(0., 128., 255.),
            2 => Vector3::new(255., 128., 0.),
            3 => Vector3::new(255., 0., 0.),
            _ => Vector3::new(255., 255., 255.),
        };

        let dc = render::DrawCall {
            matrix: mat,
            texture: ref_ent.texture_handle as u32,
            f1: ref_ent.frame_handles[0] as i32,
            f2: ref_ent.frame_handles[0] as i32,
            mix: 0.0,
            num_verts: ref_ent.num_verts,
            glow: Some(color),
        };
        render::draw(dc).unwrap();
    }
}

impl Barrier {
    pub fn new(entt: &Entity) -> Self {
        // let ref_ent = g_game::get_ref_entity(entt.index).unwrap();

        // eprintln!("re.names: {:?}", ref_ent.frame_names);

        let mut id = None;

        for (i, v) in entt.params.iter().enumerate() {
            let key = g_game::get_param(*v as usize).unwrap();
            if key == "id" {
                let value = g_game::get_param(entt.params[i + 1] as usize).unwrap();
                id = Some(value.parse().unwrap());
                break;
            }
        }

        Self {
            base: entt.clone(),
            id,
        }
    }
}
