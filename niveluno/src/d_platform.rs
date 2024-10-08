use crate::d_decor::DecorInstance;
use crate::map::Decor;

use crate::g_game;
use crate::render;

pub struct Platform {
    base: Decor,
    mat: raymath::Matrix,
}

impl DecorInstance for Platform {
    fn update(&mut self) {
        // self.base.location[1] += 0.001;
        // eprintln!("floor pos: {:?}", self.base.location);
    }

    fn draw_model(&mut self) {
        let ref_dec = g_game::get_ref_decor(self.base.ref_id).unwrap();

        let dc = render::DrawCall {
            matrix: self.mat,
            texture: ref_dec.texture_handle as u32,
            f1: ref_dec.frame_handle as i32,
            f2: ref_dec.frame_handle as i32,
            mix: 0.,
            num_verts: ref_dec.num_verts,
            glow: None,
        };
        render::draw(dc).unwrap();
    }

    fn get_mesh(&self) -> Vec<[raymath::Vector3; 3]> {
        let ldr = g_game::get_ref_decor(self.base.ref_id).unwrap();
        ldr.mesh
    }

    fn get_matrix(&self) -> raymath::Matrix {
        self.mat
    }
}

impl Platform {
    pub fn new(decor: &Decor) -> Self {
        // todo, scale + quat
        Self {
            base: decor.clone(),
            mat: raymath::matrix_translate(decor.location[0], decor.location[1], decor.location[2]),
        }
    }
}
