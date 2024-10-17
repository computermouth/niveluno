use raymath::vector3_distance;

use crate::map::Entity;

use crate::g_game;
use crate::render;

pub struct Floor {
    base: Entity,
    mat: raymath::Matrix,
}

impl Floor {
    pub fn new(decor: &Entity) -> Self {
        // scale, rotation, translation
        let mat_s = raymath::matrix_scale(decor.scale[0], decor.scale[2], decor.scale[1]);
        let mat_r = raymath::quaternion_to_matrix(decor.rotation.into());
        let mat_t =
            raymath::matrix_translate(decor.location[0], decor.location[1], decor.location[2]);

        let mut matrix = raymath::matrix_identity();
        matrix = raymath::matrix_multiply(matrix, mat_s);
        matrix = raymath::matrix_multiply(matrix, mat_r);
        matrix = raymath::matrix_multiply(matrix, mat_t);

        // todo, scale + quat
        Self {
            base: decor.clone(),
            mat: matrix,
        }
    }

    pub fn update(&mut self) {
        // self.base.location[1] += 0.001;
        // eprintln!("floor pos: {:?}", self.base.location);
    }

    pub fn draw_model(&mut self) {
        let ref_dec = g_game::get_ref_entity(self.base.ref_id).unwrap();

        let dc = render::DrawCall {
            matrix: self.mat,
            texture: ref_dec.texture_handle as u32,
            f1: ref_dec.frame_handles[0] as i32,
            f2: ref_dec.frame_handles[0] as i32,
            mix: 0.,
            num_verts: ref_dec.num_verts,
            glow: None,
        };
        render::draw(dc).unwrap();
    }

    pub fn get_mesh(&self) -> Vec<[raymath::Vector3; 3]> {
        let ldr = g_game::get_ref_entity(self.base.ref_id).unwrap();
        ldr.mesh
    }

    pub fn get_matrix(&self) -> raymath::Matrix {
        self.mat
    }
}
